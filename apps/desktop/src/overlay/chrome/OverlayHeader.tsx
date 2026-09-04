import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-shell";
import type { ReactNode } from "react";
import { useState } from "react";
import type { UserProfile } from "@aniki/shared";
import { WEB_URL } from "../constants";
import { IconButton } from "./IconButton";

function CollapseIcon() {
  return (
    <svg viewBox="0 0 16 16" className="h-3.5 w-3.5" fill="none" stroke="currentColor" strokeWidth="1.5">
      <path d="M5 5L2 2M11 5L14 2M5 11L2 14M11 11L14 14" />
    </svg>
  );
}

function MoveIcon() {
  return (
    <svg viewBox="0 0 16 16" className="h-3.5 w-3.5" fill="currentColor">
      <path d="M8 1v14M1 8h14M4 4l4-3 4 3M4 12l4 3 4-3M4 4l-3 4 3 4M12 4l3 4-3 4" opacity="0.7" />
    </svg>
  );
}

function MenuDots() {
  return (
    <svg viewBox="0 0 16 16" className="h-3.5 w-3.5" fill="currentColor">
      <circle cx="8" cy="3" r="1.2" />
      <circle cx="8" cy="8" r="1.2" />
      <circle cx="8" cy="13" r="1.2" />
    </svg>
  );
}

export function OverlayHeader({
  user,
  onCollapse,
  onLogout,
  showCredits = true,
}: {
  user?: UserProfile | null;
  onCollapse: () => void;
  onLogout?: () => void;
  showCredits?: boolean;
}) {
  const [menuOpen, setMenuOpen] = useState(false);

  const startDrag = (e: React.PointerEvent) => {
    if (e.button !== 0) return;
    e.preventDefault();
    void getCurrentWindow().startDragging();
  };

  const stealthHide = () => {
    void invoke("hide_overlay");
    setMenuOpen(false);
  };

  const toggleClickThrough = async () => {
    await invoke("toggle_click_through");
    setMenuOpen(false);
  };

  return (
    <div className="relative flex items-center gap-2 border-b border-white/10 px-3 py-2.5">
      <div className="flex min-w-0 flex-1 items-center gap-2">
        <span className="flex h-6 w-6 items-center justify-center rounded-full bg-primary text-xs font-bold text-primary-foreground">
          A
        </span>
        <span className="truncate text-sm font-semibold text-foreground">Aniki</span>
        {showCredits && user && (
          <button
            type="button"
            onClick={() => void open(`${WEB_URL}/billing`)}
            className="ml-1 rounded-full border border-white/15 px-2 py-0.5 text-[10px] text-muted-foreground hover:bg-white/5"
          >
            {user.credits} credits
          </button>
        )}
      </div>

      <div className="flex shrink-0 items-center gap-1">
        <IconButton title="Collapse to icon" onClick={onCollapse}>
          <CollapseIcon />
        </IconButton>
        <div
          data-tauri-drag-region
          onPointerDown={startDrag}
          className="cursor-grab active:cursor-grabbing"
          title="Drag to move"
        >
          <IconButton title="Move">
            <MoveIcon />
          </IconButton>
        </div>
        <div className="relative">
          <IconButton title="More options" onClick={() => setMenuOpen((v) => !v)}>
            <MenuDots />
          </IconButton>
          {menuOpen && (
            <div className="absolute right-0 top-9 z-50 min-w-[180px] rounded-xl border border-white/10 bg-black/90 py-1 shadow-xl backdrop-blur-xl">
              {user && (
                <p className="truncate px-3 py-2 text-xs text-muted-foreground">{user.email}</p>
              )}
              <button
                type="button"
                className="w-full px-3 py-2 text-left text-sm hover:bg-white/5"
                onClick={() => {
                  void open(WEB_URL);
                  setMenuOpen(false);
                }}
              >
                Dashboard ↗
              </button>
              <button
                type="button"
                className="w-full px-3 py-2 text-left text-sm hover:bg-white/5"
                onClick={() => void toggleClickThrough()}
              >
                Toggle click-through
              </button>
              <button
                type="button"
                className="w-full px-3 py-2 text-left text-sm hover:bg-white/5"
                onClick={stealthHide}
              >
                Stealth hide
              </button>
              {onLogout && (
                <button
                  type="button"
                  className="w-full px-3 py-2 text-left text-sm text-destructive hover:bg-white/5"
                  onClick={() => {
                    onLogout();
                    setMenuOpen(false);
                  }}
                >
                  Logout
                </button>
              )}
            </div>
          )}
        </div>
        <IconButton
          title="Stealth hide"
          className="border-destructive/40 bg-destructive/20 text-destructive hover:bg-destructive/30"
          onClick={stealthHide}
        >
          <span className="text-xs font-bold">×</span>
        </IconButton>
      </div>
    </div>
  );
}

export function SegmentTabs<T extends string>({
  tabs,
  active,
  onChange,
}: {
  tabs: { id: T; label: string }[];
  active: T;
  onChange: (id: T) => void;
}) {
  return (
    <div className="mx-3 mt-3 flex rounded-xl bg-white/5 p-1">
      {tabs.map((tab) => (
        <button
          key={tab.id}
          type="button"
          onClick={() => onChange(tab.id)}
          className={`flex-1 rounded-lg py-2 text-sm font-medium transition ${
            active === tab.id
              ? "bg-white/10 text-foreground"
              : "text-muted-foreground hover:text-foreground"
          }`}
        >
          {tab.label}
        </button>
      ))}
    </div>
  );
}

export function FieldLabel({ children, hint }: { children: ReactNode; hint?: string }) {
  return (
    <div className="mb-1 flex items-center gap-1">
      <label className="text-xs font-medium text-muted-foreground">{children}</label>
      {hint && (
        <span className="text-[10px] text-muted-foreground/60" title={hint}>
          ⓘ
        </span>
      )}
    </div>
  );
}
