<script lang="ts">
  // SF Symbols-style line icons. currentColor + stroke, so they inherit
  // text color and the accent everywhere. Replaces all emoji in the UI.
  export let name:
    | 'xmark'
    | 'plus'
    | 'minus'
    | 'checkmark'
    | 'arrow-clockwise'
    | 'upload'
    | 'chevron-left'
    | 'chevron-right';
  export let size: number = 16;
  export let strokeWidth: number = 1.6;
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width={strokeWidth}
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
  style="display:block"
>
  {#if name === 'xmark'}
    <path d="M6 6l12 12M18 6L6 18" />
  {:else if name === 'plus'}
    <path d="M12 5v14M5 12h14" />
  {:else if name === 'minus'}
    <path d="M5 12h14" />
  {:else if name === 'checkmark'}
    <path d="M5 12.5l4.5 4.5L19 6" />
  {:else if name === 'arrow-clockwise'}
    <path d="M21 12a9 9 0 1 1-2.64-6.36" />
    <path d="M21 3v6h-6" />
  {:else if name === 'upload'}
    <path d="M12 15.5V5" />
    <path d="M8 9l4-4 4 4" />
    <path d="M5 19h14" />
  {:else if name === 'chevron-left'}
    <path d="M15 6l-6 6 6 6" />
  {:else if name === 'chevron-right'}
    <path d="M9 6l6 6-6 6" />
  {/if}
</svg>
