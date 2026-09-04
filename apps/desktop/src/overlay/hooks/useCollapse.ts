import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useCallback, useEffect, useState } from "react";

export function useCollapse() {
  const [collapsed, setCollapsed] = useState(false);

  useEffect(() => {
    invoke<boolean>("is_collapsed_cmd")
      .then(setCollapsed)
      .catch(() => {});

    const unlisten = listen<boolean>("overlay-collapsed", (event) => {
      setCollapsed(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const collapse = useCallback(async () => {
    await invoke("collapse_overlay_cmd");
    setCollapsed(true);
  }, []);

  const expand = useCallback(async () => {
    await invoke("expand_overlay_cmd");
    setCollapsed(false);
  }, []);

  const toggle = useCallback(async () => {
    const next = await invoke<boolean>("toggle_collapsed_cmd");
    setCollapsed(next);
    return next;
  }, []);

  return { collapsed, collapse, expand, toggle };
}
