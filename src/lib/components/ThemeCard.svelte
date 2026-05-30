<script lang="ts">
  import { configStore } from '../stores';
  import { applyTheme, deleteTheme } from '../actions';
  import type { Theme } from '../types';

  export let theme: Theme;

  $: isActive = Boolean(
    $configStore?.enabled &&
      $configStore?.activeIdentifier &&
      $configStore?.selectedThemeId === theme.id
  );
  $: previewSrc = theme.previewDataUri;

  function handleApply() {
    applyTheme(theme.id);
  }

  function handleDelete() {
    if (confirm('Delete this custom background?')) {
      deleteTheme();
    }
  }
</script>

<div
  class="relative rounded-[16px] overflow-hidden cursor-pointer
    border transition-all duration-300 h-[250px]
    {isActive
      ? 'border-[#3b82f6] shadow-[0_0_16px_rgba(59,130,246,0.25)]'
      : 'border-[rgba(59,130,246,0.15)] hover:border-[rgba(59,130,246,0.35)]'}"
  on:click={handleApply}
  on:keydown={(e) => e.key === 'Enter' && handleApply()}
  role="button"
  tabindex="0"
>
  <!-- Preview Image -->
  <img
    src={previewSrc}
    alt={theme.displayName.en}
    class="w-full h-full object-cover"
  />

  <!-- Gradient Overlay -->
  <div class="absolute inset-0 bg-gradient-to-t from-[rgba(241,245,250,0.92)] via-[rgba(241,245,250,0.4)] to-transparent pointer-events-none"></div>

  <!-- Bottom Info -->
  <div class="absolute bottom-0 left-0 right-0 p-[12px_16px] flex items-end justify-between">
    <div>
      <h3 class="text-[0.95rem] font-semibold text-[#1e293b]">{theme.displayName.en}</h3>
      <p class="text-[0.75rem] text-[#94a3b8]">
        {theme.isCustom ? 'User uploaded' : 'Built-in'}
      </p>
    </div>

    <div class="flex items-center gap-2">
      {#if theme.isCustom}
        <button
          class="w-[24px] h-[24px] rounded-full flex items-center justify-center
            bg-[rgba(239,68,68,0.2)] border border-[rgba(239,68,68,0.4)]
            text-white text-[0.7rem] font-bold hover:bg-[rgba(239,68,68,0.4)]
            transition-colors"
          title="Delete custom theme"
          on:click|stopPropagation={handleDelete}
        >
          ✕
        </button>
      {/if}

      {#if isActive}
        <span class="text-[0.75rem] font-semibold text-[#10b981]">Applied</span>
      {:else}
        <span class="text-[0.75rem] font-semibold text-[#3b82f6]">Use Theme</span>
      {/if}
    </div>
  </div>
</div>
