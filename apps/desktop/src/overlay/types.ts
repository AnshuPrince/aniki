export type OverlayScreen = "login" | "shell" | "live";

export interface TranscriptLine {
  id: string;
  speaker: "interviewer" | "candidate";
  text: string;
  isFinal: boolean;
}
