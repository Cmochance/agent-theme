<script lang="ts">
  export let status: 'online' | 'offline' | 'checking' = 'checking';

  const colors = {
    online: 'var(--green)',
    offline: 'var(--red)',
    checking: 'var(--yellow)',
  } as const;
</script>

<span
  class="inline-block h-[8px] w-[8px] shrink-0 rounded-full"
  style="background:{colors[status]}; box-shadow:0 0 0 3px color-mix(in srgb, {colors[status]} 20%, transparent);"
></span>
