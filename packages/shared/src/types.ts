export interface UserProfile {
  id: string;
  email: string;
  display_name: string | null;
  credits: number;
}

export interface AuthResponse {
  access_token: string;
  token_type: string;
  expires_at: string;
  user: UserProfile;
}

export interface MagicLinkResponse {
  message: string;
}

export interface GoogleOAuthStartResponse {
  authorization_url: string;
}

export type ResumeStatus = "pending" | "processing" | "ready" | "failed";

export interface Resume {
  id: string;
  user_id: string;
  filename: string;
  status: ResumeStatus;
  chunk_count: number;
  created_at: string;
  updated_at: string;
}

export interface ResumeListResponse {
  resumes: Resume[];
}

export interface UploadResumeResponse {
  resume: Resume;
  upload_url: string | null;
}

export type SessionStatus = "active" | "ended" | "failed";
export type LlmModel = "gpt41" | "claude_sonnet" | "gpt41_mini";

export interface Session {
  id: string;
  user_id: string;
  resume_id: string | null;
  status: SessionStatus;
  model: LlmModel;
  extra_context: string | null;
  enable_screen_ocr: boolean;
  started_at: string;
  ended_at: string | null;
}

export interface CreateSessionRequest {
  resume_id?: string;
  model?: LlmModel;
  extra_context?: string;
  enable_screen_ocr?: boolean;
}

export interface CreateSessionResponse {
  session: Session;
  stt_jwt: string;
  stt_endpoint: string;
  stt_expires_at: string;
}

export interface AnswerRequest {
  question: string;
  transcript_context?: string;
  screen_ocr?: string;
}

export interface SttJwtResponse {
  jwt: string;
  endpoint: string;
  expires_at: string;
  transcription_config: Record<string, unknown>;
}

export interface CreditsResponse {
  balance: number;
  entries: CreditLedgerEntry[];
}

export interface CreditLedgerEntry {
  id: string;
  user_id: string;
  delta: number;
  reason: string;
  session_id: string | null;
  created_at: string;
}

export interface HealthResponse {
  status: string;
  service: string;
  version: string;
}

export interface SessionNotes {
  session_id: string;
  summary: string;
  key_points: string[];
  action_items: string[];
  generated_at: string;
}

export interface SessionListResponse {
  sessions: Session[];
}

export interface SessionDetailResponse {
  session: Session;
  notes: SessionNotes | null;
  transcript: string | null;
}

export interface TranscriptSegment {
  id: string;
  speaker: string;
  text: string;
  is_final: boolean;
}

export class ApiError extends Error {
  constructor(
    public status: number,
    message: string
  ) {
    super(message);
    this.name = "ApiError";
  }
}
