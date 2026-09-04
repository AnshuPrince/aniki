import { LogicalSize } from "@tauri-apps/api/dpi";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { MIN_EXPANDED } from "../constants";

export function ResizeHandle() {
  const onPointerDown = (e: React.PointerEvent<HTMLDivElement>) => {
    if (e.button !== 0) return;
    e.preventDefault();
    e.stopPropagation();

    const handle = e.currentTarget;
    handle.setPointerCapture(e.pointerId);

    const win = getCurrentWindow();
    const startScreenX = e.screenX;
    const startScreenY = e.screenY;
    let startWidth = 0;
    let startHeight = 0;
    let scale = 1;
    let ready = false;

    void (async () => {
      const size = await win.innerSize();
      scale = await win.scaleFactor();
      startWidth = size.width;
      startHeight = size.height;
      ready = true;
    })();

    const onMove = (ev: PointerEvent) => {
      if (!ready) return;
      const width = Math.max(
        MIN_EXPANDED.width,
        (startWidth + (ev.screenX - startScreenX)) / scale,
      );
      const height = Math.max(
        MIN_EXPANDED.height,
        (startHeight + (ev.screenY - startScreenY)) / scale,
      );
      void win.setSize(new LogicalSize(width, height));
    };

    const onUp = (ev: PointerEvent) => {
      handle.releasePointerCapture(ev.pointerId);
      handle.removeEventListener("pointermove", onMove);
      handle.removeEventListener("pointerup", onUp);
      handle.removeEventListener("pointercancel", onUp);
    };

    handle.addEventListener("pointermove", onMove);
    handle.addEventListener("pointerup", onUp);
    handle.addEventListener("pointercancel", onUp);
  };

  return (
    <div
      onPointerDown={onPointerDown}
      title="Drag to resize"
      className="absolute bottom-2 right-2 z-20 h-4 w-4 cursor-nwse-resize text-white/40 hover:text-white/70"
      aria-label="Resize overlay"
    >
      <svg viewBox="0 0 16 16" className="h-4 w-4" aria-hidden>
        <path
          fill="currentColor"
          d="M15 6v9H6l9-9zm0 4.4L10.4 15H15v-4.6zM15 15H9.8L15 9.8V15z"
        />
      </svg>
    </div>
  );
}
