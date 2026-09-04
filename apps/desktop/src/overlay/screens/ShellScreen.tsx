import { useState } from "react";
import { GlassPanel } from "../chrome/GlassPanel";
import { OverlayHeader, SegmentTabs } from "../chrome/OverlayHeader";
import { ResizeHandle } from "../chrome/ResizeHandle";
import { useOverlay } from "../context/OverlayContext";
import { CreateTab } from "./CreateTab";
import { SessionsTab } from "./SessionsTab";

export function ShellScreen() {
  const { user, collapse, logout } = useOverlay();
  const [tab, setTab] = useState<"create" | "sessions">("create");

  return (
    <div className="relative flex h-screen w-screen flex-col p-2">
      <GlassPanel className="relative flex min-h-0 flex-1 flex-col overflow-hidden">
        <OverlayHeader user={user} onCollapse={() => void collapse()} onLogout={logout} />
        <SegmentTabs
          tabs={[
            { id: "create" as const, label: "Create" },
            { id: "sessions" as const, label: "Call Sessions" },
          ]}
          active={tab}
          onChange={setTab}
        />
        <div className="min-h-0 flex-1 overflow-y-auto pt-3">
          {tab === "create" ? <CreateTab /> : <SessionsTab />}
        </div>
        <ResizeHandle />
      </GlassPanel>
    </div>
  );
}
