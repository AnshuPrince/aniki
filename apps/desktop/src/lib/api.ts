import { AnikiClient } from "@aniki/shared";
import { API_URL, TOKEN_KEY } from "../overlay/constants";

let accessToken: string | null = localStorage.getItem(TOKEN_KEY);

export const api = new AnikiClient({
  baseUrl: API_URL,
  getToken: () => accessToken ?? localStorage.getItem(TOKEN_KEY),
  setToken: (token) => {
    accessToken = token;
    if (token) {
      localStorage.setItem(TOKEN_KEY, token);
    } else {
      localStorage.removeItem(TOKEN_KEY);
    }
  },
});

export function setAccessToken(token: string) {
  accessToken = token;
  localStorage.setItem(TOKEN_KEY, token);
}

export function clearAccessToken() {
  accessToken = null;
  localStorage.removeItem(TOKEN_KEY);
}
