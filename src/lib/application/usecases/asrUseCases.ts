import { asrStore } from '$lib/application/stores/asrStore.svelte';
import {
  asrRepository,
  type AsrProgressPayload,
} from '$lib/infrastructure/repositories/asrRepository';
import type { UnlistenFn } from '@tauri-apps/api/event';

async function startProcessing(filePath: string): Promise<void> {
  // NOTE: `asrStore.start` でファイル名だけを渡したいが、
  //       Tauri の FileDropEvent からフルパスしか取得できないため、一旦フルパスを渡している
  asrStore.start(filePath);

  let unlistenFns: UnlistenFn[] = [];

  const cleanup = () => {
    unlistenFns.forEach((unlisten) => unlisten());
    unlistenFns = [];
  };

  const unlistenStarted = await asrRepository.onAsrStarted((payload) => {
    asrStore.setTotalDuration(payload.totalDurationMs);
  });

  const unlistenProgress = await asrRepository.onAsrProgress((payload: AsrProgressPayload) => {
    asrStore.addSegment(payload);

    const totalDuration = asrStore.value.totalDurationMs;
    if (totalDuration > 0) {
      const progress = Math.round((payload.endTimeMs / totalDuration) * 100);
      asrStore.updateProgress(progress);
    }
  });

  const unlistenFinished = await asrRepository.onAsrFinished(() => {
    asrStore.finish();
    cleanup();
  });

  const unlistenError = await asrRepository.onAsrError((errorMessage) => {
    asrStore.setError(errorMessage);
    cleanup();
  });

  unlistenFns = [unlistenStarted, unlistenProgress, unlistenFinished, unlistenError];

  try {
    await asrRepository.startAsrProcess(filePath);
  } catch (error) {
    console.error('Failed to start ASR process:', error);
    asrStore.setError('ASRプロセスの開始に失敗しました。');
    cleanup();
  }
}

export const asrUseCases = {
  startProcessing,
};
