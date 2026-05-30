<script lang="ts">
  import { configStore } from '../stores';
  import { switchAgent } from '../actions';

  const agents = ['codex', 'antigravity'] as const;

  function handleClick(agent: 'codex' | 'antigravity') {
    switchAgent(agent);
  }
</script>

<div class="flex self-center w-fit justify-center gap-[5px] p-[4px] rounded-[10px]
  border border-[rgba(59,130,246,0.12)]
  shadow-[inset_0_2px_4px_rgba(30,41,59,0.1),inset_0_-1px_2px_rgba(255,255,255,0.6)]
  bg-[rgba(59,130,246,0.03)] overflow-hidden">
  {#each agents as agent}
    <button
      class="w-[120px] py-[6px] text-[0.85rem] font-medium text-center cursor-pointer
        rounded-[6px] border-none transition-all duration-200
        border-t border-b
        {$configStore?.selectedAgent === agent
          ? 'text-white bg-[linear-gradient(180deg,#7bb3f9_0%,#4a8de8_100%)] border-t-[rgba(255,255,255,0.4)] border-b-[rgba(30,80,160,0.25)] shadow-[0_2px_6px_rgba(37,99,235,0.3),0_-1px_0_rgba(255,255,255,0.3)_inset]'
          : 'text-[#1e293b] bg-[linear-gradient(180deg,rgba(255,255,255,0.9)_0%,rgba(230,238,250,0.85)_100%)] border-t-[rgba(255,255,255,0.95)] border-b-[rgba(180,200,230,0.35)] shadow-[0_1px_3px_rgba(30,41,59,0.12),0_-1px_0_rgba(255,255,255,0.8)_inset]'}"
      on:click={() => handleClick(agent)}
    >
      {agent === 'codex' ? 'Codex' : 'Antigravity'}
    </button>
  {/each}
</div>
