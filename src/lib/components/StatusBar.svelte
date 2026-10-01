<script lang="ts">
  import { statusStore } from '../stores';
  import StatusDot from './StatusDot.svelte';

  $: hasDebugPort = Boolean($statusStore?.cdpPort);
  $: dotStatus = ($statusStore === null
    ? 'checking'
    : $statusStore.running && hasDebugPort
      ? 'online'
      : $statusStore.running
        ? 'checking'
        : 'offline') as 'online' | 'offline' | 'checking';

  $: statusText = $statusStore === null
    ? 'Checking…'
    : $statusStore.running && hasDebugPort
      ? 'Ready'
      : $statusStore.running
        ? 'No debug port'
        : 'Not running';

  $: cdpPort = $statusStore?.cdpPort ?? null;
</script>

<div data-tauri-drag-region class="flex items-center gap-[7px]">
  <StatusDot status={dotStatus} />
  <span class="text-[13px] font-medium text-[var(--ink)]">{statusText}</span>
  {#if cdpPort}
    <span class="text-[12px] text-[var(--ink-3)]" style="font-family:var(--font-mono)">:{cdpPort}</span>
  {/if}
</div>
