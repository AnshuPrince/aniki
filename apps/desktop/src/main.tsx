import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "@aniki/ui/globals.css";
import "./overlay.css";
import { OverlayApp } from "./OverlayApp";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <OverlayApp />
  </StrictMode>
);
