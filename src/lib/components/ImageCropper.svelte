<script lang="ts">
  export let imageSrc: string;
  export let onSave: (bgBase64: string, previewBase64: string) => void;
  export let onCancel: () => void;

  let cropContainer: HTMLDivElement;
  let cropImage: HTMLImageElement;
  let imageX = 0;
  let imageY = 0;
  let imageScale = 1.0;
  let isDragging = false;
  let startX = 0;
  let startY = 0;
  let imgWidth = 0;
  let imgHeight = 0;

  const BOX_SIZE = 300;

  function onImageLoad(e: Event) {
    const img = e.target as HTMLImageElement;
    imgWidth = img.naturalWidth;
    imgHeight = img.naturalHeight;
    const containerW = cropContainer?.clientWidth || 500;
    const containerH = cropContainer?.clientHeight || 350;
    const scale = Math.max(containerW / imgWidth, containerH / imgHeight);
    imageScale = scale;
    imageX = (containerW - imgWidth * scale) / 2;
    imageY = (containerH - imgHeight * scale) / 2;
  }

  function handleMouseDown(e: MouseEvent) {
    isDragging = true;
    startX = e.clientX - imageX;
    startY = e.clientY - imageY;
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging) return;
    imageX = e.clientX - startX;
    imageY = e.clientY - startY;
  }

  function handleMouseUp() {
    isDragging = false;
  }

  function handleZoom(e: Event) {
    const target = e.target as HTMLInputElement;
    const containerW = cropContainer?.clientWidth || 500;
    const containerH = cropContainer?.clientHeight || 350;
    const oldScale = imageScale;
    imageScale = parseFloat(target.value) / 100;
    const scaleRatio = imageScale / oldScale;
    const centerX = containerW / 2;
    const centerY = containerH / 2;
    imageX = centerX - (centerX - imageX) * scaleRatio;
    imageY = centerY - (centerY - imageY) * scaleRatio;
  }

  function performCrop() {
    const containerW = cropContainer?.clientWidth || 500;
    const containerH = cropContainer?.clientHeight || 350;
    const boxX = (containerW - BOX_SIZE) / 2;
    const boxY = (containerH - BOX_SIZE) / 2;
    const sx = (boxX - imageX) / imageScale;
    const sy = (boxY - imageY) / imageScale;
    const sw = BOX_SIZE / imageScale;
    const sh = BOX_SIZE / imageScale;

    // Full resolution crop
    const bgCanvas = document.createElement('canvas');
    bgCanvas.width = imgWidth;
    bgCanvas.height = imgHeight;
    const bgCtx = bgCanvas.getContext('2d')!;
    bgCtx.drawImage(cropImage, 0, 0);
    const fullSx = sx * (imgWidth / imgWidth);
    const fullSy = sy * (imgHeight / imgHeight);
    const fullSw = sw;
    const fullSh = sh;
    const bgOutCanvas = document.createElement('canvas');
    bgOutCanvas.width = Math.round(fullSw);
    bgOutCanvas.height = Math.round(fullSh);
    bgOutCanvas.getContext('2d')!.drawImage(
      bgCanvas,
      Math.round(fullSx), Math.round(fullSy), Math.round(fullSw), Math.round(fullSh),
      0, 0, bgOutCanvas.width, bgOutCanvas.height
    );
    const bgBase64 = bgOutCanvas.toDataURL('image/jpeg', 0.92);

    // Preview crop
    const previewSize = 400;
    const previewCanvas = document.createElement('canvas');
    previewCanvas.width = previewSize;
    previewCanvas.height = previewSize;
    const previewCtx = previewCanvas.getContext('2d')!;
    previewCtx.drawImage(
      cropImage,
      sx, sy, sw, sh,
      0, 0, previewSize, previewSize
    );
    const previewBase64 = previewCanvas.toDataURL('image/jpeg', 0.8);

    onSave(bgBase64, previewBase64);
  }
</script>

<div class="flex flex-col items-center gap-[16px]">
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    bind:this={cropContainer}
    class="relative flex h-[320px] w-full select-none items-center justify-center overflow-hidden rounded-[10px] border border-[var(--separator)]"
    style="background:var(--fill);"
    on:mousedown={handleMouseDown}
    on:mousemove={handleMouseMove}
    on:mouseup={handleMouseUp}
    on:mouseleave={handleMouseUp}
  >
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <img
      bind:this={cropImage}
      src={imageSrc}
      alt="To crop"
      class="absolute max-h-full max-w-full cursor-move select-none"
      style="transform: translate({imageX}px, {imageY}px) scale({imageScale}); transform-origin: 0 0;"
      on:load={onImageLoad}
      draggable="false"
    />
    <div
      class="pointer-events-none absolute h-[300px] w-[300px] rounded-[4px]"
      style="border:2px solid var(--accent); box-shadow:0 0 0 9999px rgba(0,0,0,0.35);"
    ></div>
  </div>

  <div class="w-full">
    <div class="flex items-center gap-[12px]">
      <label for="zoom-slider" class="text-[12px] font-medium text-[var(--ink-2)]">Zoom</label>
      <input
        type="range"
        id="zoom-slider"
        min="10"
        max="200"
        value="100"
        class="flex-grow"
        style="accent-color:var(--accent)"
        on:input={handleZoom}
      />
    </div>
    <p class="mt-[8px] text-center text-[11px] text-[var(--ink-3)]">
      Drag the image to reposition the crop.
    </p>
  </div>

  <div class="flex w-full justify-end gap-[10px]">
    <button
      class="rounded-[var(--radius-control)] bg-[var(--fill)] px-[16px] py-[8px] text-[13px]
        font-medium text-[var(--ink)] transition-colors hover:bg-[var(--fill-hover)]"
      on:click={onCancel}
    >
      Re-select
    </button>
    <button
      class="rounded-[var(--radius-control)] bg-[var(--accent)] px-[16px] py-[8px] text-[13px]
        font-semibold text-white transition-colors hover:bg-[var(--accent-hover)]"
      on:click={performCrop}
    >
      Save &amp; Apply
    </button>
  </div>
</div>
