import { AnikiClient } from "@aniki/shared";

const API_URL = import.meta.env.VITE_API_URL ?? "http://localhost:8080";
const TOKEN_KEY = "aniki_access_token";

export const api = new AnikiClient({
  baseUrl: API_URL,
  getToken: () => localStorage.getItem(TOKEN_KEY),
  setToken: (token) => {
    if (token) {
      localStorage.setItem(TOKEN_KEY, token);
    } else {
      localStorage.removeItem(TOKEN_KEY);
    }
  },
});

export { TOKEN_KEY };
