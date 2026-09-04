import type { ReactNode } from "react";

export function GlassPanel({
  children,
  className = "",
}: {
  children: ReactNode;
  className?: string;
}) {
  return (
    <div
      className={`rounded-2xl border border-white/10 bg-black/70 shadow-2xl backdrop-blur-xl ${className}`}
    >
      {children}
    </div>
  );
}
