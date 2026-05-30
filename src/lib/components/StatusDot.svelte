<script lang="ts">
  export let status: 'online' | 'offline' | 'checking' = 'checking';
</script>

<span
  class="inline-block w-[10px] h-[10px] rounded-full shadow-[0_1px_3px_rgba(0,0,0,0.2),inset_0_1px_2px_rgba(255,255,255,0.4)]"
  class:bg-[radial-gradient(circle_at_35%_35%,#4ade80,#16a34a)]={status === 'online'}
  class:bg-[radial-gradient(circle_at_35%_35%,#f87171,#dc2626)]={status === 'offline'}
  class:bg-[radial-gradient(circle_at_35%_35%,#fbbf24,#d97706)]={status === 'checking'}
></span>
