<script lang="ts">
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { open } from '@tauri-apps/plugin-dialog';
  import { UploadOutline } from 'flowbite-svelte-icons';

  let { onFileSelected }: { onFileSelected: (filePath: string) => void } = $props();

  let isDragover = $state(false);

  async function openFileDialog() {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [{ name: 'Audio', extensions: ['wav', 'mp3', 'm4a', 'flac', 'ogg', 'aac'] }],
    });
    if (file !== null) {
      onFileSelected(file);
    }
  }

  $effect(() => {
    let unlisten: UnlistenFn | undefined;

    const setupListener = async () => {
      unlisten = await listen<{ paths: string[] }>('tauri://drag-drop', (event) => {
        if (event.payload.paths.length > 0) {
          onFileSelected(event.payload.paths[0]);
        }
        isDragover = false;
      });
    };

    setupListener();

    return () => {
      unlisten?.();
    };
  });
</script>

<div
  class="flex h-64 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-gray-300 bg-gray-50 hover:bg-gray-100 dark:border-gray-600 dark:bg-gray-700 dark:hover:border-gray-500 dark:hover:bg-gray-600 {isDragover &&
    'border-primary-500 bg-primary-50 dark:bg-gray-800'}"
  onclick={openFileDialog}
  ondragenter={() => (isDragover = true)}
  ondragover={(e) => {
    e.preventDefault();
    isDragover = true;
  }}
  ondragleave={() => (isDragover = false)}
  ondrop={() => {
    isDragover = false;
  }}
  role="button"
  tabindex="0"
  onkeydown={(e) => {
    if (e.key === 'Enter' || e.key === ' ') {
      openFileDialog();
    }
  }}
>
  <div class="flex flex-col items-center justify-center pt-5 pb-6">
    <UploadOutline class="mb-4 h-8 w-8 text-gray-500 dark:text-gray-400" />
    <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
      <span class="font-semibold">クリックしてファイル選択</span> またはドラッグ＆ドロップ
    </p>
    <p class="text-xs text-gray-500 dark:text-gray-400">AUDIO (MP3, WAV, M4A, FLAC, etc.)</p>
  </div>
</div>
