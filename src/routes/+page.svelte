<script lang="ts">
  import { asrStore } from '$lib/application/stores/asrStore.svelte';
  import { asrUseCases } from '$lib/application/usecases/asrUseCases';
  import { saveTranscriptionFile } from '$lib/application/usecases/saveTranscriptionFile';
  import InitialView from '$lib/presentation/components/InitialView.svelte';
  import ProcessingView from '$lib/presentation/components/ProcessingView.svelte';
  import { save } from '@tauri-apps/plugin-dialog';
  import { Alert, Button, Heading, P } from 'flowbite-svelte';
  import { ExclamationCircleOutline } from 'flowbite-svelte-icons';

  // --- Event Handlers ---
  function handleFileSelected(filePath: string) {
    if (!filePath) return;
    asrUseCases.startProcessing(filePath);
  }

  async function handleSave(transcription: string) {
    const path = await save({
      defaultPath: `${asrStore.fileName}.sswt`,
      filters: [{ name: 'Simple Subtitle With Timestamp Files', extensions: ['sswt'] }],
    });
    if (path) {
      try {
        await saveTranscriptionFile(path, transcription);
        asrStore.reset();
      } catch (error) {
        console.error('Failed to save transcription file:', error);
        asrStore.setError('ファイル保存に失敗しました。');
      }
    }
  }

  function handleReset() {
    asrStore.reset();
  }
</script>

<div class="container mx-auto p-8">
  <header class="mb-10 text-center">
    <Heading tag="h1" class="mb-8">Kotonoha-ASR</Heading>
    <P class="text-center text-lg text-gray-600 dark:text-gray-400"
      >英語音声ファイルから文字起こしを行います</P
    >
  </header>

  {#if asrStore.status === 'initial'}
    <InitialView onFileSelected={handleFileSelected} />
  {:else if asrStore.status === 'error'}
    <div class="flex flex-col items-center gap-4">
      <Alert color="red" class="w-full">
        {#snippet icon()}
          <ExclamationCircleOutline class="h-5 w-5" />
        {/snippet}
        <span class="font-medium">エラーが発生しました:</span>
        {asrStore.errorMessage}
      </Alert>
      <Button onclick={handleReset}>最初の画面に戻る</Button>
    </div>
  {:else}
    <ProcessingView
      fileName={asrStore.fileName}
      progress={asrStore.progress}
      transcriptionSegments={asrStore.transcriptionSegments}
      status={asrStore.status}
      totalDurationMs={asrStore.totalDurationMs}
      processingTimeMs={asrStore.processingTimeMs}
      onSave={handleSave}
      onReset={handleReset}
    />
  {/if}
</div>
