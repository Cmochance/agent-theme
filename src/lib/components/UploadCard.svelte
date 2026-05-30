<script lang="ts">
  import { showUploadModal } from '../stores';
</script>

<div
  class="rounded-[16px] h-[250px] flex flex-col items-center justify-center cursor-pointer
    transition-all duration-300 gap-[12px]
    border-2 border-dashed border-[rgba(59,130,246,0.2)]
    bg-[rgba(59,130,246,0.02)]
    hover:border-[#3b82f6] hover:bg-[rgba(59,130,246,0.05)]"
  on:click={() => showUploadModal.set(true)}
  on:keydown={(e) => e.key === 'Enter' && showUploadModal.set(true)}
  role="button"
  tabindex="0"
>
  <span class="text-[2.2rem] transition-transform duration-300 hover:scale-110">📤</span>
  <span class="font-semibold text-[1rem] text-[#475569]">Custom Background</span>
</div>
