export const TOKEN_KEY = "aniki_access_token";

export const API_URL = import.meta.env.VITE_API_URL ?? "http://localhost:8080";
export const WEB_URL = import.meta.env.VITE_WEB_URL ?? "http://localhost:3000";

export const SHELL_SIZE = { width: 380, height: 640 } as const;
export const LIVE_SIZE = { width: 520, height: 720 } as const;
export const MIN_EXPANDED = { width: 280, height: 280 } as const;

export const CURRENT_MODEL = {
  value: "gpt41" as const,
  label: "GPT-5.6 Luna",
};
