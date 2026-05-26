// Agent Theme Companion Frontend Application
import { invoke, convertFileSrc } from '@tauri-apps/api/core';

let appConfig = null;
let currentThemes = [];

// Image Cropper State
let cropImageSrc = null;
let cropImageObj = null;
let imageX = 0;
let imageY = 0;
let imageScale = 1.0;
let isDragging = false;
let startX = 0;
let startY = 0;

// DOM Elements
const statusDot = document.getElementById('agent-status-dot');
const statusText = document.getElementById('agent-status-text');
const cdpPortText = document.getElementById('cdp-port-text');
const autoLaunchToggle = document.getElementById('auto-launch-toggle');
const themeEnabledToggle = document.getElementById('theme-enabled-toggle');
const btnStartAgent = document.getElementById('btn-start-agent');
const btnRestartAgent = document.getElementById('btn-restart-agent');
const themesGrid = document.getElementById('themes-grid');
const notificationText = document.getElementById('notification-text');

// Modal Elements
const uploadModal = document.getElementById('upload-modal');
const btnCloseModal = document.getElementById('btn-close-modal');
const dropZone = document.getElementById('drop-zone');
const fileInput = document.getElementById('file-input');
const cropArea = document.getElementById('crop-area');
const cropImage = document.getElementById('crop-image');
const cropBox = document.getElementById('crop-box');
const zoomSlider = document.getElementById('zoom-slider');
const btnCancelCrop = document.getElementById('btn-cancel-crop');
const btnSaveCrop = document.getElementById('btn-save-crop');

// Notifications
function notify(text, type = 'info') {
  notificationText.textContent = text;
  notificationText.className = `notification-info ${type}`;
}

// Fetch Status
async function refreshStatus() {
  try {
    const status = await invoke('get_agent_status');
    const config = await invoke('get_config');
    appConfig = config;
    
    // Update Agent Process status
    if (status.running) {
      statusDot.className = 'dot online';
      statusText.textContent = '正在运行';
      btnStartAgent.disabled = true;
    } else {
      statusDot.className = 'dot offline';
      statusText.textContent = '未运行';
      btnStartAgent.disabled = false;
    }
    
    // Update CDP port
    cdpPortText.textContent = status.cdpPort || '未绑定';
    
    // Update Toggles
    autoLaunchToggle.checked = appConfig.autoLaunchAgent;
    themeEnabledToggle.checked = appConfig.enabled;
    
    return status;
  } catch (err) {
    notify('无法获取后端状态', 'error');
    console.error(err);
  }
}

// Fetch Themes list
async function loadThemes() {
  try {
    currentThemes = await invoke('get_all_themes');
    renderThemesGrid();
  } catch (err) {
    notify('无法获取主题列表', 'error');
    console.error(err);
  }
}

// Render theme card grid
function renderThemesGrid() {
  themesGrid.innerHTML = '';
  
  if (currentThemes.length === 0) {
    themesGrid.innerHTML = '<div class="loading-placeholder">暂无可用主题</div>';
    return;
  }
  
  currentThemes.forEach(theme => {
    const isActive = appConfig && appConfig.selectedThemeId === theme.id;
    
    const card = document.createElement('div');
    card.className = `card theme-card${isActive ? ' active' : ''}`;
    card.dataset.id = theme.id;
    
    // Background preview
    const previewDiv = document.createElement('div');
    previewDiv.className = 'theme-preview';
    const previewPath = theme.dir + '/' + theme.preview;
    previewDiv.style.backgroundImage = `url(${convertFileSrc(previewPath)})`;
    card.appendChild(previewDiv);
    
    // Overlay
    const overlay = document.createElement('div');
    overlay.className = 'theme-overlay';
    card.appendChild(overlay);
    
    // Info Container
    const info = document.createElement('div');
    info.className = 'theme-info';
    
    const details = document.createElement('div');
    details.className = 'theme-details';
    
    const name = document.createElement('h3');
    name.textContent = theme.displayName.zh;
    details.appendChild(name);
    
    const description = document.createElement('p');
    description.textContent = theme.isCustom ? '用户上传背景' : '内置壁纸';
    details.appendChild(description);
    
    // Badges
    const badges = document.createElement('div');
    badges.className = 'theme-meta-badges';
    
    if (theme.isCustom) {
      const customBadge = document.createElement('span');
      customBadge.className = 'badge badge-custom';
      customBadge.textContent = 'Custom';
      badges.appendChild(customBadge);
    }
    
    details.appendChild(badges);
    info.appendChild(details);
    
    // Action status text
    const actionText = document.createElement('span');
    actionText.className = 'theme-action';
    actionText.textContent = isActive ? '应用中' : '使用主题';
    info.appendChild(actionText);
    
    card.appendChild(info);
    
    // Add delete button for custom theme
    if (theme.isCustom) {
      const deleteBtn = document.createElement('button');
      deleteBtn.className = 'delete-theme-btn';
      deleteBtn.innerHTML = '&times;';
      deleteBtn.title = '删除自定义主题';
      deleteBtn.addEventListener('click', async (e) => {
        e.stopPropagation(); // Avoid triggering card selection
        if (confirm('确认删除自定义背景主题？')) {
          await deleteCustomTheme();
        }
      });
      card.appendChild(deleteBtn);
    }
    
    // Click behavior
    card.addEventListener('click', () => applyTheme(theme.id));
    
    themesGrid.appendChild(card);
  });
  
  // Append Upload Card
  const uploadCard = document.createElement('div');
  uploadCard.className = 'upload-card';
  
  const uploadIcon = document.createElement('div');
  uploadIcon.className = 'upload-icon';
  uploadIcon.textContent = '➕';
  uploadCard.appendChild(uploadIcon);
  
  const uploadText = document.createElement('span');
  uploadText.textContent = '自定义背景';
  uploadCard.appendChild(uploadText);
  
  uploadCard.addEventListener('click', () => {
    openUploadModal();
  });
  
  themesGrid.appendChild(uploadCard);
}

// Apply selected theme
async function applyTheme(themeId) {
  notify(`正在应用主题 "${themeId}"...`, 'info');
  try {
    await invoke('apply_theme', { themeId });
    notify('主题应用成功！', 'info');
    await refreshStatus();
    loadThemes();
  } catch (err) {
    notify(`应用主题失败: ${err}`, 'error');
    console.error(err);
  }
}

// Clear active theme (Disable)
async function clearActiveTheme() {
  notify('正在清除当前主题...', 'info');
  try {
    await invoke('clear_theme');
    notify('主题已成功清除。', 'info');
    await refreshStatus();
    loadThemes();
  } catch (err) {
    notify(`清除主题失败: ${err}`, 'error');
    themeEnabledToggle.checked = true; // revert checkbox
    console.error(err);
  }
}

// Delete custom theme
async function deleteCustomTheme() {
  notify('正在删除自定义主题...', 'info');
  try {
    await invoke('delete_custom_theme_cmd');
    notify('自定义主题已删除。', 'info');
    await refreshStatus();
    loadThemes();
  } catch (err) {
    notify(`删除失败: ${err}`, 'error');
    console.error(err);
  }
}

// Launch Agent process
async function startAgent(forceClean = false) {
  notify('正在启动 Agent App...', 'info');
  try {
    await invoke('restart_agent');
    notify('Agent 已启动！', 'info');
    setTimeout(refreshStatus, 3000);
  } catch (err) {
    notify(`启动失败: ${err}`, 'error');
    console.error(err);
  }
}

// Restart Agent process
async function restartAgent() {
  notify('正在重启 Agent App...', 'info');
  try {
    await invoke('restart_agent');
    notify('Agent 已成功重启！', 'info');
    setTimeout(refreshStatus, 3000);
  } catch (err) {
    notify(`重启失败: ${err}`, 'error');
    console.error(err);
  }
}

// Save Config Toggles
async function updateConfig(updates) {
  try {
    if (updates.enabled !== undefined) {
      await invoke('set_enabled', { enabled: updates.enabled });
    }
    // We only have set_enabled right now. If autoLaunchAgent needs saving we should add a command for it.
    await refreshStatus();
  } catch (err) {
    console.error('Failed to update config:', err);
  }
}

// === IMAGE CROPPER MODAL LOGIC ===

function openUploadModal() {
  // Reset fields
  dropZone.style.display = 'flex';
  cropArea.style.display = 'none';
  btnCancelCrop.style.display = 'none';
  btnSaveCrop.style.display = 'none';
  fileInput.value = '';
  
  uploadModal.classList.add('show');
}

function closeUploadModal() {
  uploadModal.classList.remove('show');
  cropImageSrc = null;
  cropImageObj = null;
}

// File drop zone interaction
dropZone.addEventListener('dragover', (e) => {
  e.preventDefault();
  dropZone.classList.add('hover');
});

dropZone.addEventListener('dragleave', () => {
  dropZone.classList.remove('hover');
});

dropZone.addEventListener('drop', (e) => {
  e.preventDefault();
  dropZone.classList.remove('hover');
  const files = e.dataTransfer.files;
  if (files.length > 0) {
    handleSelectedFile(files[0]);
  }
});

dropZone.addEventListener('click', () => {
  fileInput.click();
});

fileInput.addEventListener('change', (e) => {
  const files = e.target.files;
  if (files.length > 0) {
    handleSelectedFile(files[0]);
  }
});

function handleSelectedFile(file) {
  if (!file.type.startsWith('image/')) {
    alert('请选择有效的图片文件！');
    return;
  }
  
  const reader = new FileReader();
  reader.onload = (e) => {
    cropImageSrc = e.target.result;
    initCropper();
  };
  reader.readAsDataURL(file);
}

function initCropper() {
  dropZone.style.display = 'none';
  cropArea.style.display = 'flex';
  btnCancelCrop.style.display = 'inline-block';
  btnSaveCrop.style.display = 'inline-block';
  
  cropImage.src = cropImageSrc;
  
  cropImageObj = new Image();
  cropImageObj.src = cropImageSrc;
  cropImageObj.onload = () => {
    // Reset positions and zoom slider
    zoomSlider.value = 100;
    imageScale = 1.0;
    
    // Center image inside container
    const cropContainerNode = document.querySelector('.crop-container');
    const containerWidth = cropContainerNode.clientWidth || 630;
    const containerHeight = cropContainerNode.clientHeight || 350;
    
    const imgWidth = cropImageObj.width;
    const imgHeight = cropImageObj.height;
    
    // Scale image to fit container initially
    const scaleX = containerWidth / imgWidth;
    const scaleY = containerHeight / imgHeight;
    imageScale = Math.max(scaleX, scaleY);
    if (imageScale > 1) imageScale = 1.0;
    
    zoomSlider.min = Math.floor(imageScale * 50);
    zoomSlider.max = Math.floor(imageScale * 300);
    zoomSlider.value = Math.floor(imageScale * 100);
    
    imageX = 0;
    imageY = 0;
    
    updateImageStyle();
  };
}

// Mouse dragging inside crop viewport
cropArea.addEventListener('mousedown', (e) => {
  if (e.target === cropImage || e.target.id === 'crop-area' || e.target.className === 'crop-container') {
    isDragging = true;
    startX = e.clientX - imageX;
    startY = e.clientY - imageY;
    e.preventDefault();
  }
});

window.addEventListener('mousemove', (e) => {
  if (!isDragging) return;
  const newX = e.clientX - startX;
  const newY = e.clientY - startY;
  // Clamp so image center stays within the crop container
  const container = cropContainer.getBoundingClientRect();
  const imgW = (cropImage.naturalWidth || cropImage.width) * imageScale;
  const imgH = (cropImage.naturalHeight || cropImage.height) * imageScale;
  const maxX = Math.max(0, (imgW - container.width) / 2);
  const maxY = Math.max(0, (imgH - container.height) / 2);
  imageX = Math.max(-maxX, Math.min(maxX, newX));
  imageY = Math.max(-maxY, Math.min(maxY, newY));
  updateImageStyle();
});

window.addEventListener('mouseup', () => {
  isDragging = false;
});

// Slider Zoom handler
zoomSlider.addEventListener('input', (e) => {
  imageScale = parseFloat(e.target.value) / 100;
  updateImageStyle();
});

function updateImageStyle() {
  if (cropImage) {
    cropImage.style.transform = `translate(${imageX}px, ${imageY}px) scale(${imageScale})`;
  }
}

// Perform client-side 1:1 cropping using Canvas
function performCrop() {
  if (!cropImageObj) return null;
  
  // Dimensions of final outputs
  const bgSize = 2048;
  const previewSize = 640;
  
  // Crop window is centered in a container (630x350)
  // The crop box is 300x300 centered in that container.
  // We compute the bounding box relative to the image's coordinate space.
  // Crop window is centered in a container
  const cropContainerNode = document.querySelector('.crop-container');
  const cropBoxNode = document.getElementById('crop-box');
  const containerRect = cropContainerNode.getBoundingClientRect();
  const boxRect = cropBoxNode.getBoundingClientRect();

  const containerW = containerRect.width || 630;
  const containerH = containerRect.height || 350;
  const boxSize = boxRect.width || 300;
  
  // Center coordinates of the crop box in container space
  const boxLeft = boxRect.left - containerRect.left;
  const boxTop = boxRect.top - containerRect.top;
  
  // Transform crop box coordinate to image coordinate space
  // Equation: ContainerCoord = ImageCoord * Scale + ImageOffset + CenterAdjustment
  // CenterAdjustment is because CSS transform scales from the center of the image
  const imgW = cropImageObj.width;
  const imgH = cropImageObj.height;
  
  // The image is initially centered in container space.
  const imgCenterX = containerW / 2;
  const imgCenterY = containerH / 2;
  
  // Top-left of unscaled centered image
  const initialLeft = imgCenterX - imgW / 2;
  const initialTop = imgCenterY - imgH / 2;
  
  // Active top-left of image including offset and scaling
  // Transform origin is center: 50% 50%
  const currentCenterX = initialLeft + imgW / 2 + imageX;
  const currentCenterY = initialTop + imgH / 2 + imageY;
  
  const currentLeft = currentCenterX - (imgW * imageScale) / 2;
  const currentTop = currentCenterY - (imgH * imageScale) / 2;
  
  // Relative position of crop-box inside the scaled image
  const relX = (boxLeft - currentLeft) / imageScale;
  const relY = (boxTop - currentTop) / imageScale;
  const relSize = boxSize / imageScale;
  
  // Draw Background Image (2048x2048)
  const bgCanvas = document.createElement('canvas');
  bgCanvas.width = bgSize;
  bgCanvas.height = bgSize;
  const bgCtx = bgCanvas.getContext('2d');
  
  // Fill background with black just in case of empty margins
  bgCtx.fillStyle = '#050202';
  bgCtx.fillRect(0, 0, bgSize, bgSize);
  
  bgCtx.drawImage(
    cropImageObj,
    relX, relY, relSize, relSize, // Source
    0, 0, bgSize, bgSize          // Destination
  );
  
  // Draw Preview Image (640x640)
  const previewCanvas = document.createElement('canvas');
  previewCanvas.width = previewSize;
  previewCanvas.height = previewSize;
  const previewCtx = previewCanvas.getContext('2d');
  
  previewCtx.drawImage(
    bgCanvas,
    0, 0, bgSize, bgSize,
    0, 0, previewSize, previewSize
  );
  
  // Export as JPEGs
  const bgData = bgCanvas.toDataURL('image/jpeg', 0.9);
  const previewData = previewCanvas.toDataURL('image/jpeg', 0.9);
  
  return {
    bgImage: bgData,
    previewImage: previewData
  };
}

// Upload cropped theme to server
btnSaveCrop.addEventListener('click', async () => {
  const cropped = performCrop();
  if (!cropped) {
    alert('裁剪出错，请重试！');
    return;
  }
  
  notify('正在保存并应用自定义背景...', 'info');
  closeUploadModal();
  
  try {
    await invoke('upload_custom_theme', {
      bgBase64: cropped.bgImage,
      previewBase64: cropped.previewImage
    });
    notify('自定义背景保存并应用成功！', 'info');
    await refreshStatus();
    loadThemes();
    await applyTheme('custom');
  } catch (err) {
    notify(`上传请求失败: ${err}`, 'error');
    console.error(err);
  }
});

// Event Listeners for DOM Toggles & Actions
autoLaunchToggle.addEventListener('change', (e) => {
  updateConfig({ autoLaunchAgent: e.target.checked });
});

themeEnabledToggle.addEventListener('change', async (e) => {
  themeEnabledToggle.disabled = true;
  if (e.target.checked) {
    if (appConfig && appConfig.selectedThemeId) {
      await applyTheme(appConfig.selectedThemeId);
    } else {
      await applyTheme('carton');
    }
  } else {
    await clearActiveTheme();
  }
  themeEnabledToggle.disabled = false;
});

btnStartAgent.addEventListener('click', () => startAgent(false));
btnRestartAgent.addEventListener('click', () => restartAgent());

btnCloseModal.addEventListener('click', closeUploadModal);
btnCancelCrop.addEventListener('click', openUploadModal);

// Init on Load
async function init() {
  await refreshStatus();
  await loadThemes();
  
  // Start polling status
  setInterval(refreshStatus, 15000);
}

window.addEventListener('DOMContentLoaded', init);
