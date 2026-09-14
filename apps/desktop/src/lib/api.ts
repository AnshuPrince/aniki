import { AnikiClient } from "@aniki/shared";
import { invoke } from "@tauri-apps/api/core";
import { API_URL, TOKEN_KEY } from "../overlay/constants";

let accessToken: string | null = null;

export const api = new AnikiClient({
  baseUrl: API_URL,
  getToken: () => accessToken,
  setToken: (token) => {
    accessToken = token;
  },
});

export async function restoreAccessToken() {
  accessToken = await invoke<string | null>("credential_get");
  localStorage.removeItem(TOKEN_KEY);
  return accessToken;
}

export async function setAccessToken(token: string) {
  accessToken = token;
  await invoke("credential_set", { token });
  localStorage.removeItem(TOKEN_KEY);
}

export async function clearAccessToken() {
  accessToken = null;
  localStorage.removeItem(TOKEN_KEY);
  await invoke("credential_delete");
}
