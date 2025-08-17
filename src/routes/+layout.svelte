<script lang="ts">
  import { setupStore } from '$lib/application/stores/setupStore.svelte';
  import { Progressbar } from 'flowbite-svelte';
  import '../app.css';
  import type { LayoutProps } from './$types';

  let { children, data }: LayoutProps = $props();
</script>

<main class="container mx-auto p-4">
  {#await data.setupPromise}
    <div class="flex h-64 flex-col items-center justify-center">
      <p class="mb-4">
        {setupStore.status === 'checking'
          ? 'モデルファイルを確認しています...'
          : `モデルファイルをダウンロードしています... (${setupStore.progress}%)`}
      </p>
      {#if setupStore.status === 'downloading'}
        <Progressbar progress={setupStore.progress} class="w-64" />
      {/if}
    </div>
  {:then}
    {#if setupStore.status === 'ready'}
      {@render children()}
    {:else if setupStore.status === 'error'}
      <div class="flex h-64 flex-col items-center justify-center text-red-500">
        <p>モデルの準備に失敗しました。</p>
        <p class="text-sm">{setupStore.errorMessage}</p>
      </div>
    {/if}
  {:catch error}
    <div class="flex h-64 flex-col items-center justify-center text-red-500">
      <p>予期せぬエラーが発生しました。</p>
      <p class="text-sm">{error.message}</p>
    </div>
  {/await}
</main>
