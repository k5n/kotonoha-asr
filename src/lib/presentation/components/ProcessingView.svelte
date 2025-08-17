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
    processingTimeMs,
    onSave,
    onReset,
  }: {
    fileName: string;
    progress: number;
    transcriptionSegments: readonly AsrProgressPayload[];
    status: Status;
    totalDurationMs: number;
    processingTimeMs: number;
    onSave: (_transcription: string) => void;
    onReset: () => void;
  } = $props();

  let textareaElement: HTMLTextAreaElement | undefined = $state(undefined);

  // --- Derived State ---
  const formattedDuration = $derived(formatTime(totalDurationMs));
  const formattedProcessingTime = $derived(formatTime(processingTimeMs));
  const formattedTranscription = $derived(
    transcriptionSegments
      .map((segment) => {
        const { text, startTimeMs, endTimeMs } = segment;
        return `[${formatTime(startTimeMs)} -> ${formatTime(endTimeMs)}] ${text}`;
      })
      .join('\n')
  );

  // Auto-scroll to the bottom of the textarea when new content is added
  $effect(() => {
    const _ = formattedTranscription; // Make formattedTranscription an explicit dependency

    if (textareaElement) {
      textareaElement.scrollTop = textareaElement.scrollHeight;
    }
  });
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
    {#if status === 'done' && processingTimeMs > 0}
      <p class="text-sm text-gray-500 dark:text-gray-400">処理時間: {formattedProcessingTime}</p>
    {/if}
  </div>

  <div class="w-full max-w-3xl">
    <div class="mb-2 flex justify-between">
      <span class="text-base font-medium text-blue-700 dark:text-white">進捗</span>
      <span class="text-sm font-medium text-blue-700 dark:text-white">{progress}%</span>
    </div>
    <Progressbar {progress} class="mb-5 w-full" />

    <Textarea
      bind:elementRef={textareaElement}
      value={formattedTranscription}
      readonly
      rows={15}
      class="mb-5 w-full"
      placeholder="ここに文字起こし結果が表示されます..."
    />

    <div class="flex justify-center gap-4">
      <Button color="light" disabled={status !== 'done'} onclick={onReset}>初期画面に戻る</Button>
      <Button
        color="blue"
        disabled={status !== 'done'}
        onclick={() => onSave(formattedTranscription)}>ファイルとして保存</Button
      >
    </div>
  </div>
</div>
