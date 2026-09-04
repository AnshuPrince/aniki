import { getCurrentWindow } from "@tauri-apps/api/window";
import { useCollapse } from "../hooks/useCollapse";

export function PebbleScreen() {
  const { expand } = useCollapse();

  const startDrag = (e: React.PointerEvent) => {
    if (e.button !== 0) return;
    e.preventDefault();
    void getCurrentWindow().startDragging();
  };

  return (
    <button
      type="button"
      onClick={() => void expand()}
      onPointerDown={startDrag}
      title="Show Aniki"
      className="flex h-full w-full cursor-pointer items-center justify-center rounded-full border border-white/20 bg-black/80 shadow-lg backdrop-blur-md transition hover:scale-105 hover:border-primary/50"
      aria-label="Expand Aniki overlay"
    >
      <span className="flex h-7 w-7 items-center justify-center rounded-full bg-primary text-sm font-bold text-primary-foreground">
        A
      </span>
    </button>
  );
}
