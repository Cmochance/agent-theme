<script lang="ts">
  import { themesStore } from '../stores';
  import ThemeCard from './ThemeCard.svelte';
  import Icon from './Icon.svelte';

  const PAGE_SIZE = 9;
  let page = 1;

  $: total = $themesStore.length;
  $: pageCount = Math.max(1, Math.ceil(total / PAGE_SIZE));
  $: if (page > pageCount) page = pageCount;
  $: start = (page - 1) * PAGE_SIZE;
  $: pageThemes = $themesStore.slice(start, start + PAGE_SIZE);

  function go(p: number) {
    page = Math.min(pageCount, Math.max(1, p));
  }
</script>

<div class="flex flex-col gap-[14px]">
  <div class="grid grid-cols-3 gap-[12px]">
    {#each pageThemes as theme (theme.id)}
      <ThemeCard {theme} />
    {/each}
  </div>

  {#if pageCount > 1}
    <div class="flex items-center justify-center gap-[4px]">
      <button
        class="flex h-[24px] w-[24px] items-center justify-center rounded-[6px] text-[var(--ink-2)]
          transition-colors hover:bg-[var(--fill-hover)] hover:text-[var(--ink)]
          disabled:opacity-30 disabled:hover:bg-transparent"
        disabled={page === 1}
        aria-label="Previous page"
        on:click={() => go(page - 1)}
      >
        <Icon name="chevron-left" size={14} />
      </button>

      {#each Array(pageCount) as _, i}
        <button
          class="h-[24px] min-w-[24px] rounded-[6px] px-[7px] text-[12px] font-medium transition-colors
            {page === i + 1
              ? 'bg-[var(--accent)] text-[var(--on-accent)]'
              : 'text-[var(--ink-2)] hover:bg-[var(--fill-hover)] hover:text-[var(--ink)]'}"
          on:click={() => go(i + 1)}
        >
          {i + 1}
        </button>
      {/each}

      <button
        class="flex h-[24px] w-[24px] items-center justify-center rounded-[6px] text-[var(--ink-2)]
          transition-colors hover:bg-[var(--fill-hover)] hover:text-[var(--ink)]
          disabled:opacity-30 disabled:hover:bg-transparent"
        disabled={page === pageCount}
        aria-label="Next page"
        on:click={() => go(page + 1)}
      >
        <Icon name="chevron-right" size={14} />
      </button>
    </div>
  {/if}
</div>
