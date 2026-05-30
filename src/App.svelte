<script lang="ts">
  import { onMount } from 'svelte';
  import AgentSelector from './lib/components/AgentSelector.svelte';
  import StatusBar from './lib/components/StatusBar.svelte';
  import ThemeGrid from './lib/components/ThemeGrid.svelte';
  import UploadModal from './lib/components/UploadModal.svelte';
  import { lastErrorStore } from './lib/stores';
  import { startPolling, stopPolling } from './lib/polling';

  onMount(() => {
    startPolling();
    return stopPolling;
  });
</script>

<main class="min-h-screen bg-[linear-gradient(135deg,#f8fbff_0%,#eef6ff_45%,#f6fbf7_100%)] text-[#1e293b]">
  <div class="mx-auto flex max-w-[1180px] flex-col gap-[28px] px-[24px] py-[28px]">
    <header class="flex flex-col items-center gap-[18px]">
      <h1 class="text-[1.9rem] font-semibold tracking-[0] text-[#0f172a]">
        Agent Theme Companion
      </h1>
      <AgentSelector />
      <StatusBar />
      {#if $lastErrorStore}
        <p class="max-w-[680px] rounded-[8px] border border-red-200 bg-red-50 px-[14px] py-[10px] text-center text-[0.86rem] leading-[1.45] text-red-700">
          {$lastErrorStore}
        </p>
      {/if}
    </header>

    <ThemeGrid />
  </div>

  <UploadModal />
</main>
