export type AsrStartedPayload = {
  readonly totalDurationMs: number;
};

export type AsrProgressPayload = {
  readonly text: string;
  readonly startTimeMs: number;
  readonly endTimeMs: number;
};
