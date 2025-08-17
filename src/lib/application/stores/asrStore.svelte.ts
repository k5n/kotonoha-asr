import type { AsrProgressPayload } from '$lib/domain/entities/asr';

type Status = 'initial' | 'processing' | 'done' | 'error';

let store = $state({
  status: 'initial' as Status,
  progress: 0,
  fileName: '',
  totalDurationMs: 0,
  transcriptionSegments: [] as AsrProgressPayload[],
  errorMessage: null as string | null,
});

export const asrStore = {
  get value() {
    return store;
  },

  start(fileName: string) {
    store.status = 'processing';
    store.fileName = fileName;
    store.progress = 0;
    store.totalDurationMs = 0;
    store.transcriptionSegments = [];
    store.errorMessage = null;
  },

  setTotalDuration(ms: number) {
    store.totalDurationMs = ms;
  },

  addSegment(segment: AsrProgressPayload) {
    store.transcriptionSegments.push(segment);
  },

  updateProgress(progress: number) {
    store.progress = progress;
  },

  finish() {
    store.status = 'done';
    store.progress = 100;
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
  },
};
