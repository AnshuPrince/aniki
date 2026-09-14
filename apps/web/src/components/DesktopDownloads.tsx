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

  if (!release?.macos_dmg && !release?.windows_exe) {
    return (
      <p className="text-sm text-muted-foreground">Desktop installers are not published yet.</p>
    );
  }

  return (
    <div className="space-y-2">
      <div className="flex flex-wrap gap-2">
        {release.macos_dmg && (
          <a
            className="inline-flex h-8 items-center rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:bg-primary/90"
            href={release.macos_dmg}
          >
            Download for Mac
          </a>
        )}
        {release.windows_exe && (
          <a
            className="inline-flex h-8 items-center rounded-md bg-primary px-3 text-xs font-medium text-primary-foreground hover:bg-primary/90"
            href={release.windows_exe}
          >
            Download for Windows
          </a>
        )}
      </div>
      <p className="text-xs text-muted-foreground">
        Installers are unsigned. On Mac, right-click the app and choose Open if Gatekeeper
        warns about an unidentified developer. On Windows, use More info → Run anyway if
        SmartScreen appears.
      </p>
    </div>
  );
}
