import type { AsrProgressPayload, AsrStartedPayload } from '$lib/domain/entities/asr';
import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

// --- Repository ---
async function startAsrProcess(filePath: string): Promise<void> {
  await invoke('start_asr_process', { filePath });
}

async function onAsrStarted(callback: (payload: AsrStartedPayload) => void): Promise<UnlistenFn> {
  return await listen<AsrStartedPayload>('asr-started', (event) => callback(event.payload));
}

async function onAsrProgress(callback: (payload: AsrProgressPayload) => void): Promise<UnlistenFn> {
  return await listen<AsrProgressPayload>('asr-progress', (event) => callback(event.payload));
}

async function onAsrFinished(callback: () => void): Promise<UnlistenFn> {
  return await listen('asr-finished', callback);
}

async function onAsrError(callback: (payload: string) => void): Promise<UnlistenFn> {
  return await listen<string>('asr-error', (event) => callback(event.payload));
}

export const asrRepository = {
  startAsrProcess,
  onAsrStarted,
  onAsrProgress,
  onAsrFinished,
  onAsrError,
};
