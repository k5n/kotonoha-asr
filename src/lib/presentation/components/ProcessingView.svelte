<script lang="ts">
  import { Button, Label, P, Progressbar, Textarea } from 'flowbite-svelte';

  type Props = {
    fileName: string;
    progress: number;
    transcription: string;
    status: 'processing' | 'done';
    onSave: () => void;
  };
  let { fileName, progress, transcription, status, onSave }: Props = $props();

  let progressMessage = `${progress}%`;
</script>

<div class="space-y-6">
  <div>
    <p class="text-2xl font-semibold">{fileName}</p>
  </div>
  <div>
    <Progressbar {progress} />
    <P class="mt-1 text-right text-sm text-gray-500 dark:text-gray-400">
      {progressMessage}
    </P>
  </div>
  <div>
    <Label for="transcription-output" class="mb-2 block text-lg">文字起こし結果</Label>
    <Textarea id="transcription-output" class="w-full" rows={10} readonly value={transcription} />
  </div>

  {#if status === 'done'}
    <div class="pt-4 text-center">
      <Button size="xl" onclick={onSave}>ファイル保存</Button>
    </div>
  {/if}
</div>
