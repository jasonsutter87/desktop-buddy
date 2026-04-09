import type { SheetDef } from './sheets';

export interface AnimationState {
  sheet: SheetDef;
  image: HTMLImageElement;
  currentFrame: number;
  lastFrameTime: number;
  done: boolean;
}

const imageCache = new Map<string, HTMLImageElement>();

export function loadImage(src: string): Promise<HTMLImageElement> {
  const cached = imageCache.get(src);
  if (cached?.complete) return Promise.resolve(cached);

  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = () => {
      imageCache.set(src, img);
      resolve(img);
    };
    img.onerror = reject;
    img.src = src;
  });
}

export function createAnimState(sheet: SheetDef, image: HTMLImageElement): AnimationState {
  return {
    sheet,
    image,
    currentFrame: 0,
    lastFrameTime: 0,
    done: false,
  };
}

export function renderFrame(
  ctx: CanvasRenderingContext2D,
  state: AnimationState,
  canvasSize: number,
  frameSize: number,
  timestamp: number,
): AnimationState {
  const frameDuration = 1000 / state.sheet.fps;

  let { currentFrame, lastFrameTime, done } = state;

  if (timestamp - lastFrameTime >= frameDuration) {
    if (state.sheet.loop) {
      currentFrame = (currentFrame + 1) % state.sheet.frames;
    } else if (currentFrame < state.sheet.frames - 1) {
      currentFrame++;
    } else {
      done = true;
    }
    lastFrameTime = timestamp;
  }

  ctx.clearRect(0, 0, canvasSize, canvasSize);
  ctx.imageSmoothingEnabled = false;

  ctx.drawImage(
    state.image,
    currentFrame * frameSize, 0,
    frameSize, frameSize,
    0, 0,
    canvasSize, canvasSize,
  );

  return { ...state, currentFrame, lastFrameTime, done };
}
