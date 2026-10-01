<script lang="ts">
  import Icon from './Icon.svelte';

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
  class="flex cursor-pointer flex-col items-center gap-[10px] rounded-[10px] border border-dashed
    p-[36px_20px] text-center transition-colors
    {isDragOver
      ? 'border-[var(--accent)] bg-[var(--accent-soft)]'
      : 'border-[var(--separator)] hover:border-[var(--ink-3)]'}"
  on:drop={handleDrop}
  on:dragover={handleDragOver}
  on:dragleave={handleDragLeave}
  on:click={() => fileInput?.click()}
  on:keydown={(e) => e.key === 'Enter' && fileInput?.click()}
  role="button"
  tabindex="0"
>
  <span class="text-[var(--ink-3)]"><Icon name="upload" size={28} strokeWidth={1.4} /></span>
  <p class="text-[13px] text-[var(--ink-2)]">
    Drag image here, or <span class="font-medium text-[var(--accent)]">browse files</span>
  </p>
  <span class="text-[11px] text-[var(--ink-3)]">JPG, PNG · up to 20MB</span>
  <input
    bind:this={fileInput}
    type="file"
    accept="image/png, image/jpeg"
    class="hidden"
    on:change={handleChange}
  />
</div>
