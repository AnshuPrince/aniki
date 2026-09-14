import { useEffect, useState } from "react";
import type { DesktopRelease } from "@aniki/shared";
import { api } from "../lib/api";

export function DesktopDownloads() {
  const [release, setRelease] = useState<DesktopRelease | null>(null);
  const [loaded, setLoaded] = useState(false);

  useEffect(() => {
    api
      .desktopLatest()
      .then(setRelease)
      .catch(() => setRelease(null))
      .finally(() => setLoaded(true));
  }, []);

  if (!loaded) {
    return <p className="text-sm text-muted-foreground">Checking for installers…</p>;
  }

  if (!release?.macos_dmg) {
    return (
      <p className="text-sm text-muted-foreground">
        The Mac overlay is not published yet. Windows is not offered in this release.
      </p>
    );
  }

  return (
    <div className="space-y-2">
      <div className="flex flex-wrap gap-2">
        <a
          className="inline-flex h-8 items-center rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:bg-primary/90"
          href={release.macos_dmg}
        >
          Download for Mac
        </a>
      </div>
      <p className="text-xs text-muted-foreground">
        Apple Silicon only. The installer is unsigned — right-click the app and choose Open if
        Gatekeeper warns about an unidentified developer. Windows is not available yet.
      </p>
    </div>
  );
}
