type ModelStatus = 'checking' | 'downloading' | 'ready' | 'error';

let status: ModelStatus = $state('checking');
let progress = $state(0);
let errorMessage = $state('');

export const setupStore = {
  get status() {
    return status;
  },
  get progress() {
    return progress;
  },
  get errorMessage() {
    return errorMessage;
  },
  updateProgress(p: number) {
    if (p < 100 && status !== 'downloading') {
      status = 'downloading';
    }
    progress = p;
  },
  setStatus(s: ModelStatus) {
    status = s;
  },
  setError(message: string) {
    status = 'error';
    errorMessage = message;
  },
};
