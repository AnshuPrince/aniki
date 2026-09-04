import { LogicalSize } from "@tauri-apps/api/dpi";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { LIVE_SIZE, MIN_EXPANDED, SHELL_SIZE } from "../constants";

export async function setOverlaySize(width: number, height: number) {
  await getCurrentWindow().setSize(
    new LogicalSize(Math.max(MIN_EXPANDED.width, width), Math.max(MIN_EXPANDED.height, height)),
  );
}

export async function setShellSize() {
  await setOverlaySize(SHELL_SIZE.width, SHELL_SIZE.height);
}

export async function setLiveSize() {
  await setOverlaySize(LIVE_SIZE.width, LIVE_SIZE.height);
}
