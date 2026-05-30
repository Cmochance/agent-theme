<script lang="ts">
  export let onFileSelected: (file: File) => void;

  let isDragOver = false;
  let fileInput: HTMLInputElement;

  function handleDrop(e: DragEvent) {
    e.preventDefault();
    isDragOver = false;
    const file = e.dataTransfer?.files[0];
    if (file) onFileSelected(file);
  }

  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    isDragOver = true;
  }

  function handleDragLeave() {
    isDragOver = false;
  }

  function handleChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (file) onFileSelected(file);
  }
</script>

<div
  class="border-2 border-dashed rounded-[12px] p-[40px_20px] text-center cursor-pointer
    flex flex-col items-center gap-[12px] transition-all duration-200
    {isDragOver
      ? 'border-[#3b82f6] bg-[rgba(59,130,246,0.05)]'
      : 'border-[rgba(59,130,246,0.15)]'}"
  on:drop={handleDrop}
  on:dragover={handleDragOver}
  on:dragleave={handleDragLeave}
  on:click={() => fileInput?.click()}
  on:keydown={(e) => e.key === 'Enter' && fileInput?.click()}
  role="button"
  tabindex="0"
>
  <span class="text-[3rem]">📤</span>
  <p class="text-[#475569]">Drag image here, or <span class="text-[#3b82f6] font-semibold">Browse Files</span></p>
  <span class="text-[0.75rem] text-[#94a3b8]">JPG, PNG supported, max 20MB</span>
  <input
    bind:this={fileInput}
    type="file"
    accept="image/png, image/jpeg"
    class="hidden"
    on:change={handleChange}
  />
</div>
