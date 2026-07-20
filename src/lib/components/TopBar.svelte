<script lang="ts">
  import { configStore, isRestartingStore, showUploadModal } from '../stores';
  import { restartAgent, setEnabled } from '../actions';
  import StatusBar from './StatusBar.svelte';
  import AgentSelector from './AgentSelector.svelte';
  import Toggle from './Toggle.svelte';
  import Icon from './Icon.svelte';

  function handleToggle(value: boolean) {
    setEnabled(value);
  }
  function handleRestart() {
    restartAgent();
  }
</script>

<header
  data-tauri-drag-region
  class="flex flex-col gap-[8px] border-b border-[var(--separator)] px-[16px] pb-[10px]"
>
  <!-- title-bar row: native traffic lights sit on the left, controls on the right -->
  <div data-tauri-drag-region class="flex h-[30px] items-center justify-end gap-[8px] pl-[68px]">
    <Toggle checked={$configStore?.enabled ?? false} onChange={handleToggle} />
    <button
      class="flex h-[24px] w-[24px] items-center justify-center rounded-[6px] text-[var(--ink-2)]
        transition-colors hover:bg-[var(--fill-hover)] hover:text-[var(--ink)]
        disabled:opacity-40 disabled:hover:bg-transparent disabled:hover:text-[var(--ink-2)]"
      title="Restart agent with debug port"
      disabled={$isRestartingStore}
      on:click={handleRestart}
    >
      <span class="flex" class:animate-spin={$isRestartingStore}>
        <Icon name="arrow-clockwise" size={14} />
      </span>
    </button>
  </div>

  <!-- status + custom upload -->
  <div data-tauri-drag-region class="flex items-center justify-between">
    <StatusBar />
    <button
      class="flex items-center gap-[5px] rounded-[var(--radius-control)] bg-[var(--fill)] px-[10px] py-[5px]
        text-[12px] font-medium text-[var(--ink)] transition-colors hover:bg-[var(--fill-hover)]"
      title="Upload custom background"
      on:click={() => showUploadModal.set(true)}
    >
      <Icon name="plus" size={13} />
      Custom
    </button>
  </div>

  <AgentSelector />
</header>
