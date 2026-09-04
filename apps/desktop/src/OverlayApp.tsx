import { OverlayProvider, useOverlay } from "./overlay/context/OverlayContext";
import { LiveOverlay } from "./overlay/screens/LiveOverlay";
import { LoginScreen } from "./overlay/screens/LoginScreen";
import { MirrorOverlay } from "./overlay/screens/MirrorOverlay";
import { PebbleScreen } from "./overlay/screens/PebbleScreen";
import { ShellScreen } from "./overlay/screens/ShellScreen";

const isMirrorWindow =
  new URLSearchParams(window.location.search).get("mirror") === "1";

function OverlayRouter() {
  const { screen, collapsed } = useOverlay();

  if (collapsed) {
    return (
      <div className="h-screen w-screen p-0.5">
        <PebbleScreen />
      </div>
    );
  }

  switch (screen) {
    case "login":
      return <LoginScreen />;
    case "shell":
      return <ShellScreen />;
    case "live":
      return <LiveOverlay />;
    default:
      return <LoginScreen />;
  }
}

export function OverlayApp() {
  if (isMirrorWindow) {
    return <MirrorOverlay />;
  }

  return (
    <OverlayProvider>
      <OverlayRouter />
    </OverlayProvider>
  );
}
