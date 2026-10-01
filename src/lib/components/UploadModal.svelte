<script lang="ts">
  import { showUploadModal } from '../stores';
  import { uploadTheme } from '../actions';
  import DropZone from './DropZone.svelte';
  import ImageCropper from './ImageCropper.svelte';
  import Icon from './Icon.svelte';

  let imageSrc: string | null = null;
  let mode: 'select' | 'crop' = 'select';

  function handleFileSelected(file: File) {
    const reader = new FileReader();
    reader.onload = () => {
      imageSrc = reader.result as string;
      mode = 'crop';
    };
    reader.readAsDataURL(file);
  }

  function handleReSelect() {
    mode = 'select';
    imageSrc = null;
  }

  async function handleSave(bgBase64: string, previewBase64: string) {
    await uploadTheme(bgBase64, previewBase64);
    closeModal();
  }

  function closeModal() {
    showUploadModal.set(false);
    mode = 'select';
    imageSrc = null;
  }
</script>

{#if $showUploadModal}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="fixed inset-0 z-[1000] flex items-center justify-center p-[20px]"
    style="background:rgba(0,0,0,0.32); backdrop-filter:blur(8px);"
    on:click={closeModal}
    on:keydown={(e) => e.key === 'Escape' && closeModal()}
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="flex w-full max-w-[520px] flex-col overflow-hidden rounded-[14px]"
      style="background:var(--panel-elevated); border:1px solid var(--separator); box-shadow:var(--shadow-panel); backdrop-filter:blur(30px) saturate(180%);"
      on:click|stopPropagation
      on:keydown|stopPropagation
    >
      <div class="flex items-center justify-between border-b border-[var(--separator)] px-[18px] py-[14px]">
        <h3 class="text-[14px] font-semibold text-[var(--ink)]">Upload Custom Background</h3>
        <button
          class="flex h-[24px] w-[24px] items-center justify-center rounded-[6px] text-[var(--ink-3)]
            transition-colors hover:bg-[var(--fill-hover)] hover:text-[var(--ink)]"
          on:click={closeModal}
        >
          <Icon name="xmark" size={14} />
        </button>
      </div>

      <div class="flex flex-col gap-[16px] p-[18px]">
        {#if mode === 'select'}
          <DropZone onFileSelected={handleFileSelected} />
        {:else if imageSrc}
          <ImageCropper {imageSrc} onSave={handleSave} onCancel={handleReSelect} />
        {/if}
      </div>
    </div>
  </div>
{/if}
