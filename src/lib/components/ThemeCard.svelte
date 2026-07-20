<script lang="ts">
  import { configStore } from '../stores';
  import { applyTheme, deleteTheme } from '../actions';
  import Icon from './Icon.svelte';
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
  class="group relative h-[120px] cursor-pointer overflow-hidden rounded-[var(--radius-card)]
    transition duration-200 hover:brightness-[1.03]"
  style="box-shadow:{isActive ? '0 0 0 2px var(--accent)' : '0 0 0 1px var(--separator)'};"
  on:click={handleApply}
  on:keydown={(e) => e.key === 'Enter' && handleApply()}
  role="button"
  tabindex="0"
>
  <img src={previewSrc} alt={theme.displayName.en} class="h-full w-full object-cover" />

  <!-- neutral bottom scrim -->
  <div
    class="pointer-events-none absolute inset-x-0 bottom-0 h-[56px]"
    style="background:linear-gradient(to top, rgba(0,0,0,0.72), rgba(0,0,0,0.30) 45%, transparent);"
  ></div>

  <div class="absolute inset-x-0 bottom-0 flex items-end justify-between gap-[6px] p-[8px_10px]">
    <div class="min-w-0">
      <h3 class="truncate text-[12px] font-semibold text-white">{theme.displayName.en}</h3>
    </div>
  </div>

  {#if isActive}
    <div
      class="absolute right-[8px] top-[8px] flex h-[20px] w-[20px] items-center justify-center rounded-full text-white"
      style="background:var(--accent); box-shadow:0 1px 3px rgba(0,0,0,0.3);"
    >
      <Icon name="checkmark" size={13} strokeWidth={2.2} />
    </div>
  {:else if theme.isCustom}
    <button
      class="absolute right-[8px] top-[8px] flex h-[20px] w-[20px] items-center justify-center rounded-full
        bg-black/45 text-white opacity-0 transition-colors hover:bg-[var(--red)] group-hover:opacity-100"
      title="Delete custom theme"
      on:click|stopPropagation={handleDelete}
    >
      <Icon name="xmark" size={12} strokeWidth={2} />
    </button>
  {/if}
</div>
