import { asrStore } from '$lib/application/stores/asrStore.svelte';
import {
  asrRepository,
  type AsrProgressPayload,
} from '$lib/infrastructure/repositories/asrRepository';
import type { UnlistenFn } from '@tauri-apps/api/event';

async function startProcessing(filePath: string): Promise<void> {
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

  unlistenFns = [unlistenStarted, unlistenProgress, unlistenFinished];

  try {
    await asrRepository.startAsrProcess(filePath);
  } catch (error) {
    console.error('Failed to start ASR process:', error);
    asrStore.reset();
    cleanup();
  }
}

export const asrUseCases = {
  startProcessing,
};
