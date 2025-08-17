<script lang="ts">
  import { asrStore } from '$lib/application/stores/asrStore.svelte';
  import { asrUseCases } from '$lib/application/usecases/asrUseCases';
  import InitialView from '$lib/presentation/components/InitialView.svelte';
  import ProcessingView from '$lib/presentation/components/ProcessingView.svelte';
  import { Alert, Button, Heading, P } from 'flowbite-svelte';
  import { ExclamationCircleOutline } from 'flowbite-svelte-icons';

  // --- Event Handlers ---
  function handleFileSelected(filePath: string) {
    if (!filePath) return;
    asrUseCases.startProcessing(filePath);
  }

  function handleSave() {
    // TODO: SRTファイルとして保存する処理を実装する
  }

  function handleReset() {
    asrStore.reset();
  }
</script>

<div class="container mx-auto p-8">
  <header class="mb-10 text-center">
    <Heading tag="h1" class="mb-8">Kotonoha-ASR</Heading>
    <P class="text-center text-lg text-gray-600 dark:text-gray-400"
      >音声ファイルから文字起こしを行います</P
    >
  </header>

  {#if asrStore.value.status === 'initial'}
    <InitialView onFileSelected={handleFileSelected} />
  {:else if asrStore.value.status === 'error'}
    <div class="flex flex-col items-center gap-4">
      <Alert color="red" class="w-full">
        {#snippet icon()}
          <ExclamationCircleOutline class="h-5 w-5" />
        {/snippet}
        <span class="font-medium">エラーが発生しました:</span>
        {asrStore.value.errorMessage}
      </Alert>
      <Button onclick={handleReset}>最初の画面に戻る</Button>
    </div>
  {:else}
    <ProcessingView
      fileName={asrStore.value.fileName}
      progress={asrStore.value.progress}
      transcriptionSegments={asrStore.value.transcriptionSegments}
      status={asrStore.value.status}
      totalDurationMs={asrStore.value.totalDurationMs}
      onSave={handleSave}
    />
  {/if}
</div>
