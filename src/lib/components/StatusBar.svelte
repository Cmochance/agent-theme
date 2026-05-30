<script lang="ts">
  import { statusStore, configStore, isRestartingStore } from '../stores';
  import { restartAgent, setEnabled } from '../actions';
  import StatusDot from './StatusDot.svelte';
  import Toggle from './Toggle.svelte';

  $: hasDebugPort = Boolean($statusStore?.cdpPort);
  $: dotStatus = ($statusStore === null
    ? 'checking'
    : $statusStore.running && hasDebugPort
      ? 'online'
      : $statusStore.running
        ? 'checking'
      : 'offline') as 'online' | 'offline' | 'checking';

  $: statusText = $statusStore === null
    ? 'Checking...'
    : $statusStore.running && hasDebugPort
      ? 'Ready'
      : $statusStore.running
        ? 'No debug port'
      : 'Not running';

  $: cdpPort = $statusStore?.cdpPort ?? '-';

  function handleToggle(value: boolean) {
    setEnabled(value);
  }

  function handleRestart() {
    restartAgent();
  }
</script>

<div class="flex flex-nowrap items-center p-[12px_16px] gap-[20px]
  w-fit self-center bg-white/75 rounded-[16px]
  border border-[rgba(59,130,246,0.15)]
  backdrop-blur-[20px] shadow-[0_8px_32px_rgba(30,41,59,0.08)]">

  <div class="flex flex-col items-center w-[120px]">
    <div class="flex items-center gap-2">
      <StatusDot status={dotStatus} />
      <span class="text-[1rem] font-semibold text-[#1e293b]">{statusText}</span>
    </div>
  </div>

  <div class="flex flex-col items-center w-[120px]">
    <span class="text-[1rem] font-semibold text-[#0ea5e9]">{cdpPort}</span>
  </div>

  <div class="flex flex-col items-center w-[120px]">
    <Toggle
      checked={$configStore?.enabled ?? false}
      onChange={handleToggle}
    />
  </div>

  <div class="flex flex-col items-center w-[132px]">
    <button
      class="h-[36px] w-[120px] rounded-[8px]
        border border-[rgba(37,99,235,0.18)]
        bg-[linear-gradient(180deg,#ffffff_0%,#eef5ff_100%)]
        text-[0.86rem] font-semibold text-[#2563eb]
        shadow-[0_1px_3px_rgba(30,41,59,0.12),0_-1px_0_rgba(255,255,255,0.9)_inset]
        transition-all duration-200
        hover:border-[rgba(37,99,235,0.32)] hover:bg-[linear-gradient(180deg,#ffffff_0%,#e3efff_100%)]
        disabled:cursor-not-allowed disabled:opacity-60"
      disabled={$isRestartingStore}
      title="Restart selected app with local debug port"
      on:click={handleRestart}
    >
      {$isRestartingStore ? 'Restarting' : 'Restart App'}
    </button>
  </div>
</div>
