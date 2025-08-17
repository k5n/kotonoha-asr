<script lang="ts">
  import InitialView from '$lib/presentation/components/InitialView.svelte';
  import ProcessingView from '$lib/presentation/components/ProcessingView.svelte';
  import { Heading, P } from 'flowbite-svelte';
  import { asrStore } from '$lib/application/stores/asrStore.svelte';
  import { asrUseCases } from '$lib/application/usecases/asrUseCases';

  // --- Event Handlers ---
  function startProcessing(files: FileList) {
    if (!files || files.length === 0) return;
    // The file name is used as a filePath in this stub implementation.
    asrUseCases.startProcessing(files[0].name);
  }

  function handleSave() {
    // TODO: SRTファイルとして保存する処理を実装する
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
    <InitialView onFileSelected={startProcessing} />
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
