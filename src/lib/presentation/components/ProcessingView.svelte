<script lang="ts">
  import type { AsrProgressPayload } from '$lib/domain/entities/asr';
  import { formatTime } from '$lib/presentation/utils/time';
  import { Button, Progressbar, Textarea } from 'flowbite-svelte';

  // --- Props ---
  type Status = 'initial' | 'processing' | 'done';
  let {
    fileName,
    progress,
    transcriptionSegments,
    status,
    totalDurationMs,
    onSave,
  }: {
    fileName: string;
    progress: number;
    transcriptionSegments: AsrProgressPayload[];
    status: Status;
    totalDurationMs: number;
    onSave: () => void;
  } = $props();

  // --- Derived State ---
  const formattedDuration = $derived(formatTime(totalDurationMs));
  const formattedTranscription = $derived(
    transcriptionSegments
      .map((segment) => {
        const { text, startTimeMs, endTimeMs } = segment;
        return `[${formatTime(startTimeMs)} -> ${formatTime(endTimeMs)}] ${text}`;
      })
      .join('\n')
  );
</script>

<div class="flex w-full flex-col items-center">
  <div
    class="mb-4 w-full max-w-3xl rounded-lg border border-gray-200 bg-white p-4 dark:border-gray-700 dark:bg-gray-800"
  >
    <p class="truncate text-lg font-semibold text-gray-900 dark:text-white">
      {fileName}
    </p>
    {#if totalDurationMs > 0}
      <p class="text-sm text-gray-500 dark:text-gray-400">全体時間: {formattedDuration}</p>
    {/if}
  </div>

  <div class="w-full max-w-3xl">
    <div class="mb-2 flex justify-between">
      <span class="text-base font-medium text-blue-700 dark:text-white">進捗</span>
      <span class="text-sm font-medium text-blue-700 dark:text-white">{progress}%</span>
    </div>
    <Progressbar {progress} class="mb-5 w-full" />

    <Textarea
      value={formattedTranscription}
      readonly
      rows={15}
      class="mb-5 w-full"
      placeholder="ここに文字起こし結果が表示されます..."
    />

    <div class="text-center">
      <Button color="blue" disabled={status !== 'done'} onclick={onSave}>ファイルとして保存</Button>
    </div>
  </div>
</div>
