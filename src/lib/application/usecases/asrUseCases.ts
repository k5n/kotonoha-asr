import { asrStore } from '$lib/application/stores/asrStore.svelte';
import { asrRepository } from '$lib/infrastructure/repositories/asrRepository';
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
    asrStore.setStarted(payload.totalDurationMs);
  });

  const unlistenProgress = await asrRepository.onAsrProgress((payload) => {
    asrStore.addProgress(payload);
  });

  const unlistenFinished = await asrRepository.onAsrFinished((payload) => {
    asrStore.setFinished(payload.processingTimeMs);
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
