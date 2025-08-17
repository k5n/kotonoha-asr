<script lang="ts">
  import { Fileupload, Label } from 'flowbite-svelte';
  import { UploadOutline } from 'flowbite-svelte-icons';

  let { onFileSelected }: { onFileSelected: (files: FileList) => void } = $props();

  let isDragover = $state(false);

  function handleFileChange(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files) {
      onFileSelected(target.files);
    }
  }

  function handleDrop(event: DragEvent) {
    if (event.dataTransfer?.files && event.dataTransfer.files.length > 0) {
      onFileSelected(event.dataTransfer.files);
    }
    isDragover = false;
  }
</script>

<Label
  for="file-upload"
  class="flex h-64 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-gray-300 bg-gray-50 hover:bg-gray-100 dark:border-gray-600 dark:bg-gray-700 dark:hover:border-gray-500 dark:hover:bg-gray-600 {isDragover &&
    'border-primary-500 bg-primary-50 dark:bg-gray-800'}"
  ondragenter={() => (isDragover = true)}
  ondragover={(e) => {
    e.preventDefault();
    isDragover = true;
  }}
  ondragleave={() => (isDragover = false)}
  ondrop={(e) => {
    e.preventDefault();
    handleDrop(e);
  }}
>
  <div class="flex flex-col items-center justify-center pt-5 pb-6">
    <UploadOutline class="mb-4 h-8 w-8 text-gray-500 dark:text-gray-400" />
    <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
      <span class="font-semibold">クリックしてアップロード</span> またはドラッグ＆ドロップ
    </p>
    <p class="text-xs text-gray-500 dark:text-gray-400">AUDIO (MP3, WAV, etc.)</p>
  </div>
  <Fileupload id="file-upload" class="hidden" onchange={handleFileChange} accept="audio/*" />
</Label>
