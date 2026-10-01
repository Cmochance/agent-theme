<script lang="ts">
  import { configStore } from '../stores';
  import { switchAgent } from '../actions';
  import type { AgentKind } from '../types';

  const agents: AgentKind[] = ['codex', 'antigravity', 'linear'];
  const labels: Record<AgentKind, string> = {
    codex: 'Codex',
    antigravity: 'Antigravity',
    linear: 'Linear',
  };
</script>

<div class="grid grid-cols-3 gap-[2px] rounded-[var(--radius-control)] bg-[var(--fill)] p-[2px]">
  {#each agents as agent}
    <button
      class="rounded-[5px] py-[5px] text-center text-[12px] font-medium transition-colors duration-150
        {$configStore?.selectedAgent === agent
          ? 'bg-[var(--accent)] text-[var(--on-accent)] shadow-[0_1px_2px_rgba(0,0,0,0.12)]'
          : 'text-[var(--ink-2)] hover:bg-[var(--fill-hover)] hover:text-[var(--ink)]'}"
      on:click={() => switchAgent(agent)}
    >
      {labels[agent]}
    </button>
  {/each}
</div>
