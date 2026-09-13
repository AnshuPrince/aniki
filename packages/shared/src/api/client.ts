import { DEFAULT_API_URL } from "../constants";
import type {
  AuthResponse,
  CreateSessionRequest,
  CreateSessionResponse,
  CreditsResponse,
  GoogleOAuthStartResponse,
  HealthResponse,
  MagicLinkResponse,
  ResumeListResponse,
  SessionDetailResponse,
  SessionListResponse,
  SttJwtResponse,
  TranscriptSegment,
  UploadResumeResponse,
  UserProfile,
} from "../types";
import { ApiError } from "../types";

export interface AnikiClientOptions {
  baseUrl?: string;
  getToken?: () => string | null;
  setToken?: (token: string | null) => void;
}

export class AnikiClient {
  private baseUrl: string;
  private getToken: () => string | null;
  private setToken: (token: string | null) => void;

  constructor(options: AnikiClientOptions = {}) {
    this.baseUrl = options.baseUrl ?? DEFAULT_API_URL;
    this.getToken = options.getToken ?? (() => null);
    this.setToken = options.setToken ?? (() => {});
  }

  private async request<T>(
    path: string,
    init: RequestInit = {}
  ): Promise<T> {
    const headers = new Headers(init.headers);
    headers.set("Content-Type", "application/json");

    const token = this.getToken();
    if (token) {
      headers.set("Authorization", `Bearer ${token}`);
    }

    const response = await fetch(`${this.baseUrl}${path}`, {
      ...init,
      headers,
    });

    if (!response.ok) {
      const text = await response.text().catch(() => "Request failed");
      throw new ApiError(response.status, text);
    }

    if (response.status === 204) {
      return undefined as T;
    }

    return response.json() as Promise<T>;
  }

  async health(): Promise<HealthResponse> {
    return this.request<HealthResponse>("/health");
  }

  async requestMagicLink(email: string): Promise<MagicLinkResponse> {
    return this.request<MagicLinkResponse>("/auth/magic-link", {
      method: "POST",
      body: JSON.stringify({ email }),
    });
  }

  async verifyToken(token: string): Promise<AuthResponse> {
    const response = await this.request<AuthResponse>("/auth/verify", {
      method: "POST",
      body: JSON.stringify({ token }),
    });
    this.setToken(response.access_token);
    return response;
  }

  async me(): Promise<UserProfile> {
    return this.request<UserProfile>("/auth/me");
  }

  async oauthGoogleStart(): Promise<GoogleOAuthStartResponse> {
    return this.request<GoogleOAuthStartResponse>("/auth/oauth/google/start");
  }

  async oauthGoogleCallback(code: string, state: string): Promise<AuthResponse> {
    const response = await this.request<AuthResponse>("/auth/oauth/google/callback", {
      method: "POST",
      body: JSON.stringify({ code, state }),
    });
    this.setToken(response.access_token);
    return response;
  }

  async listResumes(): Promise<ResumeListResponse> {
    return this.request<ResumeListResponse>("/resumes");
  }

  async uploadResume(filename: string, contentBase64?: string): Promise<UploadResumeResponse> {
    return this.request<UploadResumeResponse>("/resumes", {
      method: "POST",
      body: JSON.stringify({ filename, content_base64: contentBase64 }),
    });
  }

  async listSessions(): Promise<SessionListResponse> {
    return this.request<SessionListResponse>("/sessions");
  }

  async getSession(sessionId: string): Promise<SessionDetailResponse> {
    return this.request<SessionDetailResponse>(`/sessions/${sessionId}`);
  }

  async appendTranscript(sessionId: string, segments: TranscriptSegment[]): Promise<void> {
    await this.request(`/sessions/${sessionId}/transcript`, {
      method: "POST",
      body: JSON.stringify({ segments }),
    });
  }

  async confirmQuestion(text: string): Promise<{ is_question: boolean }> {
    return this.request("/questions/confirm", {
      method: "POST",
      body: JSON.stringify({ text }),
    });
  }

  async createSession(req: CreateSessionRequest): Promise<CreateSessionResponse> {
    return this.request<CreateSessionResponse>("/sessions", {
      method: "POST",
      body: JSON.stringify(req),
    });
  }

  async refreshSttJwt(sessionId: string): Promise<SttJwtResponse> {
    return this.request<SttJwtResponse>(`/sessions/${sessionId}/stt-jwt`, {
      method: "POST",
    });
  }

  async getCredits(): Promise<CreditsResponse> {
    return this.request<CreditsResponse>("/billing/credits");
  }

  async *streamAnswer(
    sessionId: string,
    question: string,
    transcriptContext?: string,
    screenOcr?: string
  ): AsyncGenerator<string> {
    const token = this.getToken();
    const response = await fetch(`${this.baseUrl}/sessions/${sessionId}/answer`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        ...(token ? { Authorization: `Bearer ${token}` } : {}),
      },
      body: JSON.stringify({
        question,
        transcript_context: transcriptContext,
        screen_ocr: screenOcr,
      }),
    });

    if (!response.ok || !response.body) {
      throw new ApiError(response.status, "Failed to stream answer");
    }

    const reader = response.body.getReader();
    const decoder = new TextDecoder();
    let buffer = "";

    while (true) {
      const { done, value } = await reader.read();
      if (done) break;

      buffer += decoder.decode(value, { stream: true });
      const lines = buffer.split("\n");
      buffer = lines.pop() ?? "";

      for (const line of lines) {
        if (line.startsWith("data: ")) {
          yield line.slice(6);
        }
      }
    }
  }

  async finalizeSession(sessionId: string): Promise<{ session_id: string; notes_status: string }> {
    return this.request(`/sessions/${sessionId}/finalize`, { method: "POST" });
  }
}
