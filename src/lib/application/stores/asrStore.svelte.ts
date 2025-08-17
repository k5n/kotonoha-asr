import type { AsrProgressPayload } from '$lib/domain/entities/asr';

type Status = 'initial' | 'processing' | 'done' | 'error';

const store = $state({
  status: 'initial' as Status,
  progress: 0,
  fileName: '',
  totalDurationMs: 0,
  transcriptionSegments: [] as readonly AsrProgressPayload[],
  errorMessage: null as string | null,
  processingTimeMs: 0,
});

export const asrStore = {
  get status() {
    return store.status;
  },
  get progress() {
    return store.progress;
  },
  get fileName() {
    return store.fileName;
  },
  get totalDurationMs() {
    return store.totalDurationMs;
  },
  get transcriptionSegments() {
    return store.transcriptionSegments;
  },
  get errorMessage() {
    return store.errorMessage;
  },
  get processingTimeMs() {
    return store.processingTimeMs;
  },

  start(fileName: string) {
    store.status = 'processing';
    store.fileName = fileName;
    store.progress = 0;
    store.totalDurationMs = 0;
    store.transcriptionSegments = [];
    store.errorMessage = null;
    store.processingTimeMs = 0;
  },

  setStarted(totalDurationMs: number) {
    store.totalDurationMs = totalDurationMs;
  },

  addProgress(payload: AsrProgressPayload) {
    store.transcriptionSegments = [...store.transcriptionSegments, payload];
    if (store.totalDurationMs > 0) {
      const progress = Math.min(100, Math.round((payload.endTimeMs / store.totalDurationMs) * 100));
      store.progress = progress;
    }
  },

  setFinished(processingTimeMs: number) {
    store.status = 'done';
    store.progress = 100;
    store.processingTimeMs = processingTimeMs;
  },

  setError(message: string) {
    store.status = 'error';
    store.errorMessage = message;
  },

  reset() {
    store.status = 'initial';
    store.fileName = '';
    store.progress = 0;
    store.totalDurationMs = 0;
    store.transcriptionSegments = [];
    store.errorMessage = null;
    store.processingTimeMs = 0;
  },
};
