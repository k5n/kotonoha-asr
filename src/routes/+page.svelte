<script lang="ts">
  import InitialView from '$lib/presentation/components/InitialView.svelte';
  import ProcessingView from '$lib/presentation/components/ProcessingView.svelte';
  import { Heading, P } from 'flowbite-svelte';

  // --- State ---
  let status: 'initial' | 'processing' | 'done' = $state('initial');
  let progress = $state(45);
  let transcription = $state(
    '本日はお集まりいただきありがとうございます。\n今回は新プロジェクトの進捗についてお話します。'
  );
  let fileName = $state('dummy_audio.mp3');

  // --- Event Handlers ---
  function startProcessing(files: FileList) {
    if (!files || files.length === 0) return;

    // TODO: ファイルが選択されたので、ここで処理を開始する
    console.log('File selected:', files[0]);
    fileName = files[0].name;
    status = 'processing';
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

  {#if status === 'initial'}
    <InitialView onFileSelected={startProcessing} />
  {:else}
    <ProcessingView {fileName} {progress} {transcription} {status} onSave={handleSave} />
  {/if}
</div>
