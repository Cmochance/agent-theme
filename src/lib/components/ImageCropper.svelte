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

<div class="flex flex-col gap-[20px] items-center">
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    bind:this={cropContainer}
    class="w-full h-[350px] bg-[#e2e8f0] rounded-[12px] overflow-hidden relative
      border border-[rgba(59,130,246,0.15)] flex items-center justify-center select-none"
    on:mousedown={handleMouseDown}
    on:mousemove={handleMouseMove}
    on:mouseup={handleMouseUp}
    on:mouseleave={handleMouseUp}
  >
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <img
      bind:this={cropImage}
      src={imageSrc}
      alt="To Crop"
      class="max-w-full max-h-full absolute cursor-move select-none"
      style="transform: translate({imageX}px, {imageY}px) scale({imageScale}); transform-origin: 0 0;"
      on:load={onImageLoad}
      draggable="false"
    />
    <div
      class="absolute w-[300px] h-[300px] border-2 border-[#3b82f6]
        shadow-[0_0_0_9999px_rgba(30,41,59,0.2)] pointer-events-none rounded-[4px]"
    ></div>
  </div>

  <div class="w-full">
    <div class="flex items-center gap-[16px]">
      <label for="zoom-slider" class="font-medium text-[#475569]">Zoom:</label>
      <input
        type="range" id="zoom-slider"
        min="10"
        max="200"
        value="100"
        class="flex-grow accent-[#3b82f6]"
        on:input={handleZoom}
      />
    </div>
    <p class="text-[0.8rem] text-[#94a3b8] mt-[8px] text-center">
      💡 Tip: Drag on the image to adjust crop position.
    </p>
  </div>

  <div class="flex gap-[12px] justify-end w-full">
    <button
      class="px-[20px] py-[10px] rounded-[10px] text-[0.9rem] font-medium cursor-pointer
        bg-[rgba(59,130,246,0.07)] text-[#1e293b] border border-[rgba(59,130,246,0.1)]
        hover:bg-[rgba(59,130,246,0.12)] transition-colors"
      on:click={onCancel}
    >
      Re-select
    </button>
    <button
      class="px-[20px] py-[10px] rounded-[10px] text-[0.9rem] font-medium cursor-pointer
        bg-[#3b82f6] text-white shadow-[0_4px_14px_rgba(59,130,246,0.35)]
        hover:bg-[#60a5fa] transition-colors"
      on:click={performCrop}
    >
      Save & Apply
    </button>
  </div>
</div>
