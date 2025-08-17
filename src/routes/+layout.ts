import { setupStore } from '$lib/application/stores/setupStore.svelte';
import { ensureModelIsReady } from '$lib/application/usecases/modelSetup';
import type { LayoutLoad } from './$types';

export const prerender = false;
export const ssr = false;

export const load: LayoutLoad = async () => {
  const setupPromise = ensureModelIsReady((p) => {
    setupStore.updateProgress(p);
  })
    .then(() => {
      setupStore.setStatus('ready');
    })
    .catch((e) => {
      setupStore.setError(e instanceof Error ? e.message : String(e));
      // load関数でエラーを再スローすると、SvelteKitのエラーページが表示される
      // ここではUI側でハンドリングするため、再スローはしない
    });

  return {
    setupPromise,
  };
};
