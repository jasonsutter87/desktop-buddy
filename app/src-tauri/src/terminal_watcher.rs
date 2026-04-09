use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::mpsc;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize)]
pub struct TerminalEvent {
    pub event_type: String,
    pub message: String,
}

fn get_history_path() -> Option<PathBuf> {
    if let Some(appdata) = dirs::config_dir() {
        let ps_history = appdata
            .join("Microsoft")
            .join("Windows")
            .join("PowerShell")
            .join("PSReadLine")
            .join("ConsoleHost_history.txt");
        if ps_history.exists() {
            return Some(ps_history);
        }
    }
    if let Some(home) = dirs::home_dir() {
        let bash = home.join(".bash_history");
        if bash.exists() {
            return Some(bash);
        }
    }
    None
}

fn get_buddy_log_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join("buddy").join("buddy-messages.log")
}

fn classify_terminal(line: &str) -> Option<TerminalEvent> {
    let lower = line.to_lowercase();

    if lower.contains("error") || lower.contains("failed") || lower.contains("exception") {
        Some(TerminalEvent {
            event_type: "error".into(),
            message: "Something broke!".into(),
        })
    } else if lower.contains("git push") {
        Some(TerminalEvent {
            event_type: "git-push".into(),
            message: "Shipped it!".into(),
        })
    } else if lower.contains("git commit") {
        Some(TerminalEvent {
            event_type: "git-commit".into(),
            message: "Saved!".into(),
        })
    } else if lower.contains("npm run build")
        || lower.contains("cargo build")
        || lower.contains("pnpm build")
    {
        Some(TerminalEvent {
            event_type: "build".into(),
            message: "Building...".into(),
        })
    } else if lower.contains("npm test")
        || lower.contains("cargo test")
        || lower.contains("pnpm test")
    {
        Some(TerminalEvent {
            event_type: "test".into(),
            message: "Testing...".into(),
        })
    } else {
        None
    }
}

/// Classify user messages from Claude Code
fn classify_message(line: &str) -> Option<TerminalEvent> {
    let lower = line.to_lowercase().trim().to_string();

    if lower.is_empty() {
        return None;
    }

    // Mentions buddy/clamber by name
    if lower.contains("clamber") || lower.contains("buddy") {
        return Some(TerminalEvent {
            event_type: "mentioned".into(),
            message: "You called?".into(),
        });
    }

    // Frustration
    if lower.contains("ugh")
        || lower.contains("wtf")
        || lower.contains("why is")
        || lower.contains("doesn't work")
        || lower.contains("broken")
        || lower.contains("hate")
        || lower.contains("frustrated")
        || lower.contains("stupid")
        || lower.contains("damn")
    {
        return Some(TerminalEvent {
            event_type: "frustrated".into(),
            message: pick_frustrated_response(&lower),
        });
    }

    // Celebration
    if lower.contains("works!")
        || lower.contains("finally")
        || lower.contains("perfect")
        || lower.contains("awesome")
        || lower.contains("let's go")
        || lower.contains("hell yeah")
        || lower.contains("nailed it")
        || lower.contains("ship it")
        || lower.contains("done!")
        || lower.contains("fixed!")
    {
        return Some(TerminalEvent {
            event_type: "celebration".into(),
            message: pick_celebration_response(&lower),
        });
    }

    // Asking for help
    if lower.starts_with("how do")
        || lower.starts_with("how to")
        || lower.starts_with("can you help")
        || lower.starts_with("help me")
        || lower.contains("i'm stuck")
        || lower.contains("im stuck")
        || lower.contains("no idea")
        || lower.contains("confused")
    {
        return Some(TerminalEvent {
            event_type: "help".into(),
            message: "You got this!".into(),
        });
    }

    // Thinking / planning
    if lower.starts_with("what if")
        || lower.starts_with("should i")
        || lower.starts_with("maybe")
        || lower.contains("i think")
        || lower.contains("let me think")
    {
        return Some(TerminalEvent {
            event_type: "thinking".into(),
            message: "*watches thoughtfully*".into(),
        });
    }

    // Greetings
    if lower == "hi"
        || lower == "hello"
        || lower == "hey"
        || lower.starts_with("good morning")
        || lower.starts_with("good evening")
        || lower.starts_with("gm")
    {
        return Some(TerminalEvent {
            event_type: "greeting".into(),
            message: "*perks up*".into(),
        });
    }

    // Late night
    if lower.contains("tired")
        || lower.contains("sleep")
        || lower.contains("bed")
        || lower.contains("exhausted")
        || lower.contains("goodnight")
        || lower.contains("gn")
    {
        return Some(TerminalEvent {
            event_type: "sleepy".into(),
            message: "*yawns sympathetically*".into(),
        });
    }

    None
}

fn pick_frustrated_response(msg: &str) -> String {
    // Use message length as a simple "random" seed
    let responses = [
        "*nuzzles your hand*",
        "Deep breaths...",
        "Bugs happen to the best",
        "You'll figure it out",
        "*sits closer*",
        "Take a break?",
    ];
    let idx = msg.len() % responses.len();
    responses[idx].to_string()
}

fn pick_celebration_response(msg: &str) -> String {
    let responses = [
        "*happy bouncing*",
        "YESSS!",
        "*does a little dance*",
        "Knew you could do it!",
        "*victory wiggle*",
        "Champion!",
    ];
    let idx = msg.len() % responses.len();
    responses[idx].to_string()
}

/// Watch a single file, emit events for new lines using the classifier
fn watch_file(
    app_handle: AppHandle,
    path: PathBuf,
    classifier: fn(&str) -> Option<TerminalEvent>,
    label: &str,
) {
    let label = label.to_string();
    std::thread::spawn(move || {
        // Create the file if it doesn't exist
        if !path.exists() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = OpenOptions::new().create(true).write(true).open(&path);
        }

        let mut last_pos = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);

        let (tx, rx) = mpsc::channel();
        let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
            Ok(w) => w,
            Err(e) => {
                log::warn!("Failed to create watcher for {}: {}", label, e);
                return;
            }
        };

        if let Err(e) = watcher.watch(&path, RecursiveMode::NonRecursive) {
            log::warn!("Failed to watch {} ({}): {}", label, path.display(), e);
            return;
        }

        log::info!("Watching {}: {:?}", label, path);

        for event in rx {
            if let Ok(event) = event {
                if matches!(event.kind, EventKind::Modify(_)) {
                    if let Ok(mut file) = File::open(&path) {
                        let current_len = file.metadata().map(|m| m.len()).unwrap_or(0);
                        if current_len > last_pos {
                            let _ = file.seek(SeekFrom::Start(last_pos));
                            let reader = BufReader::new(&file);
                            for line in reader.lines().flatten() {
                                if let Some(te) = classifier(&line) {
                                    let _ = app_handle.emit("terminal-event", &te);
                                }
                            }
                            last_pos = current_len;
                        }
                    }
                }
            }
        }
    });
}

pub fn start_watcher(app_handle: AppHandle) {
    // Watch terminal history
    if let Some(history_path) = get_history_path() {
        watch_file(
            app_handle.clone(),
            history_path,
            classify_terminal,
            "terminal history",
        );
    }

    // Watch Claude Code messages
    let buddy_log = get_buddy_log_path();
    watch_file(app_handle, buddy_log, classify_message, "buddy messages");
}
