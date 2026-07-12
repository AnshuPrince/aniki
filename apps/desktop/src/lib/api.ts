import { AnikiClient } from "@aniki/shared";

const API_URL = import.meta.env.VITE_API_URL ?? "http://localhost:8080";

let accessToken: string | null = null;

export const api = new AnikiClient({
  baseUrl: API_URL,
  getToken: () => accessToken,
  setToken: (token) => {
    accessToken = token;
  },
});

export function setAccessToken(token: string) {
  accessToken = token;
}
