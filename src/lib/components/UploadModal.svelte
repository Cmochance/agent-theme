<script lang="ts">
  import { showUploadModal } from '../stores';
  import { uploadTheme } from '../actions';
  import DropZone from './DropZone.svelte';
  import ImageCropper from './ImageCropper.svelte';

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

  function handleCancel() {
    mode = 'select';
    imageSrc = null;
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
    class="fixed inset-0 z-[1000] flex items-center justify-center p-[20px]
      bg-[rgba(15,23,42,0.4)] backdrop-blur-[10px]"
    on:click={closeModal}
    on:keydown={(e) => e.key === 'Escape' && closeModal()}
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="w-full max-w-[680px] flex flex-col bg-white/75 rounded-[16px]
        border border-[rgba(59,130,246,0.15)] backdrop-blur-[20px]
        shadow-[0_8px_32px_rgba(30,41,59,0.08)]"
      on:click|stopPropagation
      on:keydown|stopPropagation
    >
      <!-- Header -->
      <div class="flex justify-between items-center p-[20px_24px]
        border-b border-[rgba(59,130,246,0.15)]">
        <h3 class="text-[1.3rem] font-semibold text-[#1e293b]">Upload Custom Background</h3>
        <button
          class="bg-none border-none text-[#475569] text-[1.8rem] cursor-pointer
            leading-none hover:text-[#3b82f6]"
          on:click={closeModal}
        >
          &times;
        </button>
      </div>

      <!-- Body -->
      <div class="p-[24px] flex flex-col gap-[20px]">
        {#if mode === 'select'}
          <DropZone onFileSelected={handleFileSelected} />
        {:else if imageSrc}
          <ImageCropper
            {imageSrc}
            onSave={handleSave}
            onCancel={handleReSelect}
          />
        {/if}
      </div>
    </div>
  </div>
{/if}
