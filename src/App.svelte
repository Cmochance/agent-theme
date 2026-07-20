<script lang="ts">
  import { onMount } from 'svelte';
  import TopBar from './lib/components/TopBar.svelte';
  import ThemeGrid from './lib/components/ThemeGrid.svelte';
  import UploadModal from './lib/components/UploadModal.svelte';
  import { lastErrorStore } from './lib/stores';
  import { startPolling, stopPolling } from './lib/polling';

  onMount(() => {
    startPolling();
    return stopPolling;
  });
</script>

<main class="flex h-screen flex-col text-[var(--ink)]">
  <TopBar />

  <div class="flex-1 overflow-y-auto px-[16px] py-[14px]">
    {#if $lastErrorStore}
      <p
        class="mb-[12px] rounded-[7px] px-[12px] py-[8px] text-[12px] leading-[1.45]"
        style="color:var(--red); border:1px solid color-mix(in srgb,var(--red) 30%,transparent); background:color-mix(in srgb,var(--red) 12%,transparent);"
      >
        {$lastErrorStore}
      </p>
    {/if}
    <ThemeGrid />
  </div>

  <UploadModal />
</main>
