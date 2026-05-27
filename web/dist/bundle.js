var __getOwnPropNames = Object.getOwnPropertyNames;
var __esm = (fn, res) => function __init() {
  return fn && (res = (0, fn[__getOwnPropNames(fn)[0]])(fn = 0)), res;
};
var __commonJS = (cb, mod) => function __require() {
  return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
};

// node_modules/@tauri-apps/api/external/tslib/tslib.es6.js
function __classPrivateFieldGet(receiver, state, kind, f) {
  if (kind === "a" && !f) throw new TypeError("Private accessor was defined without a getter");
  if (typeof state === "function" ? receiver !== state || !f : !state.has(receiver)) throw new TypeError("Cannot read private member from an object whose class did not declare it");
  return kind === "m" ? f : kind === "a" ? f.call(receiver) : f ? f.value : state.get(receiver);
}
function __classPrivateFieldSet(receiver, state, value, kind, f) {
  if (kind === "m") throw new TypeError("Private method is not writable");
  if (kind === "a" && !f) throw new TypeError("Private accessor was defined without a setter");
  if (typeof state === "function" ? receiver !== state || !f : !state.has(receiver)) throw new TypeError("Cannot write private member to an object whose class did not declare it");
  return kind === "a" ? f.call(receiver, value) : f ? f.value = value : state.set(receiver, value), value;
}
var init_tslib_es6 = __esm({
  "node_modules/@tauri-apps/api/external/tslib/tslib.es6.js"() {
  }
});

// node_modules/@tauri-apps/api/core.js
function transformCallback(callback, once = false) {
  return window.__TAURI_INTERNALS__.transformCallback(callback, once);
}
async function invoke(cmd, args = {}, options) {
  return window.__TAURI_INTERNALS__.invoke(cmd, args, options);
}
function convertFileSrc(filePath, protocol = "asset") {
  return window.__TAURI_INTERNALS__.convertFileSrc(filePath, protocol);
}
var _Channel_onmessage, _Channel_nextMessageIndex, _Channel_pendingMessages, _Channel_messageEndIndex, _Resource_rid, SERIALIZE_TO_IPC_FN, Channel;
var init_core = __esm({
  "node_modules/@tauri-apps/api/core.js"() {
    init_tslib_es6();
    SERIALIZE_TO_IPC_FN = "__TAURI_TO_IPC_KEY__";
    Channel = class {
      constructor(onmessage) {
        _Channel_onmessage.set(this, void 0);
        _Channel_nextMessageIndex.set(this, 0);
        _Channel_pendingMessages.set(this, []);
        _Channel_messageEndIndex.set(this, void 0);
        __classPrivateFieldSet(this, _Channel_onmessage, onmessage || (() => {
        }), "f");
        this.id = transformCallback((rawMessage) => {
          const index = rawMessage.index;
          if ("end" in rawMessage) {
            if (index == __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")) {
              this.cleanupCallback();
            } else {
              __classPrivateFieldSet(this, _Channel_messageEndIndex, index, "f");
            }
            return;
          }
          const message = rawMessage.message;
          if (index == __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")) {
            __classPrivateFieldGet(this, _Channel_onmessage, "f").call(this, message);
            __classPrivateFieldSet(this, _Channel_nextMessageIndex, __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") + 1, "f");
            while (__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") in __classPrivateFieldGet(this, _Channel_pendingMessages, "f")) {
              const message2 = __classPrivateFieldGet(this, _Channel_pendingMessages, "f")[__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")];
              __classPrivateFieldGet(this, _Channel_onmessage, "f").call(this, message2);
              delete __classPrivateFieldGet(this, _Channel_pendingMessages, "f")[__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f")];
              __classPrivateFieldSet(this, _Channel_nextMessageIndex, __classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") + 1, "f");
            }
            if (__classPrivateFieldGet(this, _Channel_nextMessageIndex, "f") === __classPrivateFieldGet(this, _Channel_messageEndIndex, "f")) {
              this.cleanupCallback();
            }
          } else {
            __classPrivateFieldGet(this, _Channel_pendingMessages, "f")[index] = message;
          }
        });
      }
      cleanupCallback() {
        window.__TAURI_INTERNALS__.unregisterCallback(this.id);
      }
      set onmessage(handler) {
        __classPrivateFieldSet(this, _Channel_onmessage, handler, "f");
      }
      get onmessage() {
        return __classPrivateFieldGet(this, _Channel_onmessage, "f");
      }
      [(_Channel_onmessage = /* @__PURE__ */ new WeakMap(), _Channel_nextMessageIndex = /* @__PURE__ */ new WeakMap(), _Channel_pendingMessages = /* @__PURE__ */ new WeakMap(), _Channel_messageEndIndex = /* @__PURE__ */ new WeakMap(), SERIALIZE_TO_IPC_FN)]() {
        return `__CHANNEL__:${this.id}`;
      }
      toJSON() {
        return this[SERIALIZE_TO_IPC_FN]();
      }
    };
    _Resource_rid = /* @__PURE__ */ new WeakMap();
  }
});

// app.js
var require_app = __commonJS({
  "app.js"() {
    init_core();
    var appConfig = null;
    var currentThemes = [];
    var cropImageSrc = null;
    var cropImageObj = null;
    var imageX = 0;
    var imageY = 0;
    var imageScale = 1;
    var isDragging = false;
    var startX = 0;
    var startY = 0;
    var statusDot = document.getElementById("agent-status-dot");
    var statusText = document.getElementById("agent-status-text");
    var cdpPortText = document.getElementById("cdp-port-text");
    var autoLaunchToggle = document.getElementById("auto-launch-toggle");
    var themeEnabledToggle = document.getElementById("theme-enabled-toggle");
    var btnStartAgent = document.getElementById("btn-start-agent");
    var btnRestartAgent = document.getElementById("btn-restart-agent");
    var themesGrid = document.getElementById("themes-grid");
    var notificationText = document.getElementById("notification-text");
    var btnAgentCodex = document.getElementById("btn-agent-codex");
    var btnAgentAntigravity = document.getElementById("btn-agent-antigravity");
    var uploadModal = document.getElementById("upload-modal");
    var btnCloseModal = document.getElementById("btn-close-modal");
    var dropZone = document.getElementById("drop-zone");
    var fileInput = document.getElementById("file-input");
    var cropArea = document.getElementById("crop-area");
    var cropImage = document.getElementById("crop-image");
    var cropBox = document.getElementById("crop-box");
    var zoomSlider = document.getElementById("zoom-slider");
    var btnCancelCrop = document.getElementById("btn-cancel-crop");
    var btnSaveCrop = document.getElementById("btn-save-crop");
    function notify(text, type = "info") {
      notificationText.textContent = text;
      notificationText.className = `notification-info ${type}`;
    }
    async function loadThemes() {
      try {
        currentThemes = await invoke("get_all_themes");
        renderThemesGrid();
      } catch (err) {
        notify("\u65E0\u6CD5\u83B7\u53D6\u4E3B\u9898\u5217\u8868", "error");
        console.error(err);
      }
    }
    function renderThemesGrid() {
      themesGrid.innerHTML = "";
      if (currentThemes.length === 0) {
        themesGrid.innerHTML = '<div class="loading-placeholder">\u6682\u65E0\u53EF\u7528\u4E3B\u9898</div>';
        return;
      }
      currentThemes.forEach((theme) => {
        const isActive = appConfig && appConfig.selectedThemeId === theme.id;
        const card = document.createElement("div");
        card.className = `card theme-card${isActive ? " active" : ""}`;
        card.dataset.id = theme.id;
        const previewDiv = document.createElement("div");
        previewDiv.className = "theme-preview";
        const previewPath = theme.dir + "/" + theme.preview;
        previewDiv.style.backgroundImage = `url(${convertFileSrc(previewPath)})`;
        card.appendChild(previewDiv);
        const overlay = document.createElement("div");
        overlay.className = "theme-overlay";
        card.appendChild(overlay);
        const info = document.createElement("div");
        info.className = "theme-info";
        const details = document.createElement("div");
        details.className = "theme-details";
        const name = document.createElement("h3");
        name.textContent = theme.displayName.zh;
        details.appendChild(name);
        const description = document.createElement("p");
        description.textContent = theme.isCustom ? "\u7528\u6237\u4E0A\u4F20\u80CC\u666F" : "\u5185\u7F6E\u58C1\u7EB8";
        details.appendChild(description);
        const badges = document.createElement("div");
        badges.className = "theme-meta-badges";
        if (theme.isCustom) {
          const customBadge = document.createElement("span");
          customBadge.className = "badge badge-custom";
          customBadge.textContent = "Custom";
          badges.appendChild(customBadge);
        }
        details.appendChild(badges);
        info.appendChild(details);
        const actionText = document.createElement("span");
        actionText.className = "theme-action";
        actionText.textContent = isActive ? "\u5E94\u7528\u4E2D" : "\u4F7F\u7528\u4E3B\u9898";
        info.appendChild(actionText);
        card.appendChild(info);
        if (theme.isCustom) {
          const deleteBtn = document.createElement("button");
          deleteBtn.className = "delete-theme-btn";
          deleteBtn.innerHTML = "&times;";
          deleteBtn.title = "\u5220\u9664\u81EA\u5B9A\u4E49\u4E3B\u9898";
          deleteBtn.addEventListener("click", async (e) => {
            e.stopPropagation();
            if (confirm("\u786E\u8BA4\u5220\u9664\u81EA\u5B9A\u4E49\u80CC\u666F\u4E3B\u9898\uFF1F")) {
              await deleteCustomTheme();
            }
          });
          card.appendChild(deleteBtn);
        }
        card.addEventListener("click", () => applyTheme(theme.id));
        themesGrid.appendChild(card);
      });
      const uploadCard = document.createElement("div");
      uploadCard.className = "upload-card";
      const uploadIcon = document.createElement("div");
      uploadIcon.className = "upload-icon";
      uploadIcon.textContent = "\u2795";
      uploadCard.appendChild(uploadIcon);
      const uploadText = document.createElement("span");
      uploadText.textContent = "\u81EA\u5B9A\u4E49\u80CC\u666F";
      uploadCard.appendChild(uploadText);
      uploadCard.addEventListener("click", () => {
        openUploadModal();
      });
      themesGrid.appendChild(uploadCard);
    }
    async function applyTheme(themeId) {
      notify(`\u6B63\u5728\u5E94\u7528\u4E3B\u9898 "${themeId}"...`, "info");
      try {
        await invoke("apply_theme", { themeId });
        notify("\u4E3B\u9898\u5E94\u7528\u6210\u529F\uFF01", "info");
        await refreshStatus();
        loadThemes();
      } catch (err) {
        notify(`\u5E94\u7528\u4E3B\u9898\u5931\u8D25: ${err}`, "error");
        console.error(err);
      }
    }
    async function clearActiveTheme() {
      notify("\u6B63\u5728\u6E05\u9664\u5F53\u524D\u4E3B\u9898...", "info");
      try {
        await invoke("clear_theme");
        notify("\u4E3B\u9898\u5DF2\u6210\u529F\u6E05\u9664\u3002", "info");
        await refreshStatus();
        loadThemes();
      } catch (err) {
        notify(`\u6E05\u9664\u4E3B\u9898\u5931\u8D25: ${err}`, "error");
        themeEnabledToggle.checked = true;
        console.error(err);
      }
    }
    async function deleteCustomTheme() {
      notify("\u6B63\u5728\u5220\u9664\u81EA\u5B9A\u4E49\u4E3B\u9898...", "info");
      try {
        await invoke("delete_custom_theme_cmd");
        notify("\u81EA\u5B9A\u4E49\u4E3B\u9898\u5DF2\u5220\u9664\u3002", "info");
        await refreshStatus();
        loadThemes();
      } catch (err) {
        notify(`\u5220\u9664\u5931\u8D25: ${err}`, "error");
        console.error(err);
      }
    }
    async function startAgent(forceClean = false) {
      notify("\u6B63\u5728\u542F\u52A8 Agent App...", "info");
      try {
        await invoke("restart_agent");
        notify("Agent \u5DF2\u542F\u52A8\uFF01", "info");
        setTimeout(refreshStatus, 3e3);
      } catch (err) {
        notify(`\u542F\u52A8\u5931\u8D25: ${err}`, "error");
        console.error(err);
      }
    }
    async function restartAgent() {
      notify("\u6B63\u5728\u91CD\u542F Agent App...", "info");
      try {
        await invoke("restart_agent");
        notify("Agent \u5DF2\u6210\u529F\u91CD\u542F\uFF01", "info");
        setTimeout(refreshStatus, 3e3);
      } catch (err) {
        notify(`\u91CD\u542F\u5931\u8D25: ${err}`, "error");
        console.error(err);
      }
    }
    function openUploadModal() {
      dropZone.style.display = "flex";
      cropArea.style.display = "none";
      btnCancelCrop.style.display = "none";
      btnSaveCrop.style.display = "none";
      fileInput.value = "";
      uploadModal.classList.add("show");
    }
    function closeUploadModal() {
      uploadModal.classList.remove("show");
      cropImageSrc = null;
      cropImageObj = null;
    }
    dropZone.addEventListener("dragover", (e) => {
      e.preventDefault();
      dropZone.classList.add("hover");
    });
    dropZone.addEventListener("dragleave", () => {
      dropZone.classList.remove("hover");
    });
    dropZone.addEventListener("drop", (e) => {
      e.preventDefault();
      dropZone.classList.remove("hover");
      const files = e.dataTransfer.files;
      if (files.length > 0) {
        handleSelectedFile(files[0]);
      }
    });
    dropZone.addEventListener("click", () => {
      fileInput.click();
    });
    fileInput.addEventListener("change", (e) => {
      const files = e.target.files;
      if (files.length > 0) {
        handleSelectedFile(files[0]);
      }
    });
    function handleSelectedFile(file) {
      if (!file.type.startsWith("image/")) {
        alert("\u8BF7\u9009\u62E9\u6709\u6548\u7684\u56FE\u7247\u6587\u4EF6\uFF01");
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
      dropZone.style.display = "none";
      cropArea.style.display = "flex";
      btnCancelCrop.style.display = "inline-block";
      btnSaveCrop.style.display = "inline-block";
      cropImage.src = cropImageSrc;
      cropImageObj = new Image();
      cropImageObj.src = cropImageSrc;
      cropImageObj.onload = () => {
        zoomSlider.value = 100;
        imageScale = 1;
        const cropContainerNode = document.querySelector(".crop-container");
        const containerWidth = cropContainerNode.clientWidth || 630;
        const containerHeight = cropContainerNode.clientHeight || 350;
        const imgWidth = cropImageObj.width;
        const imgHeight = cropImageObj.height;
        const scaleX = containerWidth / imgWidth;
        const scaleY = containerHeight / imgHeight;
        imageScale = Math.max(scaleX, scaleY);
        if (imageScale > 1) imageScale = 1;
        zoomSlider.min = Math.floor(imageScale * 50);
        zoomSlider.max = Math.floor(imageScale * 300);
        zoomSlider.value = Math.floor(imageScale * 100);
        imageX = 0;
        imageY = 0;
        updateImageStyle();
      };
    }
    cropArea.addEventListener("mousedown", (e) => {
      if (e.target === cropImage || e.target.id === "crop-area" || e.target.className === "crop-container") {
        isDragging = true;
        startX = e.clientX - imageX;
        startY = e.clientY - imageY;
        e.preventDefault();
      }
    });
    window.addEventListener("mousemove", (e) => {
      if (!isDragging) return;
      const newX = e.clientX - startX;
      const newY = e.clientY - startY;
      const container = cropContainer.getBoundingClientRect();
      const imgW = (cropImage.naturalWidth || cropImage.width) * imageScale;
      const imgH = (cropImage.naturalHeight || cropImage.height) * imageScale;
      const maxX = Math.max(0, (imgW - container.width) / 2);
      const maxY = Math.max(0, (imgH - container.height) / 2);
      imageX = Math.max(-maxX, Math.min(maxX, newX));
      imageY = Math.max(-maxY, Math.min(maxY, newY));
      updateImageStyle();
    });
    window.addEventListener("mouseup", () => {
      isDragging = false;
    });
    zoomSlider.addEventListener("input", (e) => {
      imageScale = parseFloat(e.target.value) / 100;
      updateImageStyle();
    });
    function updateImageStyle() {
      if (cropImage) {
        cropImage.style.transform = `translate(${imageX}px, ${imageY}px) scale(${imageScale})`;
      }
    }
    function performCrop() {
      if (!cropImageObj) return null;
      const bgSize = 2048;
      const previewSize = 640;
      const cropContainerNode = document.querySelector(".crop-container");
      const cropBoxNode = document.getElementById("crop-box");
      const containerRect = cropContainerNode.getBoundingClientRect();
      const boxRect = cropBoxNode.getBoundingClientRect();
      const containerW = containerRect.width || 630;
      const containerH = containerRect.height || 350;
      const boxSize = boxRect.width || 300;
      const boxLeft = boxRect.left - containerRect.left;
      const boxTop = boxRect.top - containerRect.top;
      const imgW = cropImageObj.width;
      const imgH = cropImageObj.height;
      const imgCenterX = containerW / 2;
      const imgCenterY = containerH / 2;
      const initialLeft = imgCenterX - imgW / 2;
      const initialTop = imgCenterY - imgH / 2;
      const currentCenterX = initialLeft + imgW / 2 + imageX;
      const currentCenterY = initialTop + imgH / 2 + imageY;
      const currentLeft = currentCenterX - imgW * imageScale / 2;
      const currentTop = currentCenterY - imgH * imageScale / 2;
      const relX = (boxLeft - currentLeft) / imageScale;
      const relY = (boxTop - currentTop) / imageScale;
      const relSize = boxSize / imageScale;
      const bgCanvas = document.createElement("canvas");
      bgCanvas.width = bgSize;
      bgCanvas.height = bgSize;
      const bgCtx = bgCanvas.getContext("2d");
      bgCtx.fillStyle = "#050202";
      bgCtx.fillRect(0, 0, bgSize, bgSize);
      bgCtx.drawImage(
        cropImageObj,
        relX,
        relY,
        relSize,
        relSize,
        // Source
        0,
        0,
        bgSize,
        bgSize
        // Destination
      );
      const previewCanvas = document.createElement("canvas");
      previewCanvas.width = previewSize;
      previewCanvas.height = previewSize;
      const previewCtx = previewCanvas.getContext("2d");
      previewCtx.drawImage(
        bgCanvas,
        0,
        0,
        bgSize,
        bgSize,
        0,
        0,
        previewSize,
        previewSize
      );
      const bgData = bgCanvas.toDataURL("image/jpeg", 0.9);
      const previewData = previewCanvas.toDataURL("image/jpeg", 0.9);
      return {
        bgImage: bgData,
        previewImage: previewData
      };
    }
    btnSaveCrop.addEventListener("click", async () => {
      const cropped = performCrop();
      if (!cropped) {
        alert("\u88C1\u526A\u51FA\u9519\uFF0C\u8BF7\u91CD\u8BD5\uFF01");
        return;
      }
      notify("\u6B63\u5728\u4FDD\u5B58\u5E76\u5E94\u7528\u81EA\u5B9A\u4E49\u80CC\u666F...", "info");
      closeUploadModal();
      try {
        await invoke("upload_custom_theme", {
          bgBase64: cropped.bgImage,
          previewBase64: cropped.previewImage
        });
        notify("\u81EA\u5B9A\u4E49\u80CC\u666F\u4FDD\u5B58\u5E76\u5E94\u7528\u6210\u529F\uFF01", "info");
        await refreshStatus();
        loadThemes();
        await applyTheme("custom");
      } catch (err) {
        notify(`\u4E0A\u4F20\u8BF7\u6C42\u5931\u8D25: ${err}`, "error");
        console.error(err);
      }
    });
    function setupAgentSelector() {
      const btns = document.querySelectorAll(".agent-btn");
      btns.forEach((btn) => {
        btn.addEventListener("click", async () => {
          const agent = btn.dataset.agent;
          if (appConfig && appConfig.selectedAgent === agent) return;
          notify(`\u5207\u6362\u5230 ${agent === "codex" ? "Codex" : "Antigravity"}...`, "info");
          try {
            await invoke("set_selected_agent", { agent });
            await refreshStatus();
            await loadThemes();
            if (appConfig && appConfig.enabled && appConfig.selectedThemeId) {
              await applyTheme(appConfig.selectedThemeId);
            }
            notify(`\u5DF2\u5207\u6362\u5230 ${agent === "codex" ? "Codex" : "Antigravity"}`, "info");
          } catch (err) {
            notify(`\u5207\u6362\u5931\u8D25: ${err}`, "error");
            console.error(err);
          }
        });
      });
    }
    autoLaunchToggle.addEventListener("change", async (e) => {
      try {
        await invoke("set_auto_launch", { enabled: e.target.checked });
        await refreshStatus();
      } catch (err) {
        console.error("Failed to update auto launch:", err);
      }
    });
    themeEnabledToggle.addEventListener("change", async (e) => {
      themeEnabledToggle.disabled = true;
      if (e.target.checked) {
        if (appConfig && appConfig.selectedThemeId) {
          await applyTheme(appConfig.selectedThemeId);
        } else {
          await applyTheme("carton");
        }
      } else {
        await clearActiveTheme();
      }
      themeEnabledToggle.disabled = false;
    });
    btnStartAgent.addEventListener("click", () => startAgent(false));
    btnRestartAgent.addEventListener("click", () => restartAgent());
    btnCloseModal.addEventListener("click", closeUploadModal);
    btnCancelCrop.addEventListener("click", openUploadModal);
    async function init() {
      setupAgentSelector();
      await refreshStatus();
      await loadThemes();
      setInterval(refreshStatus, 15e3);
    }
    window.addEventListener("DOMContentLoaded", init);
    async function refreshStatus() {
      try {
        const status = await invoke("get_agent_status");
        const config = await invoke("get_config");
        appConfig = config;
        if (status.running) {
          statusDot.className = "dot online";
          statusText.textContent = "\u6B63\u5728\u8FD0\u884C";
          btnStartAgent.disabled = true;
        } else {
          statusDot.className = "dot offline";
          statusText.textContent = "\u672A\u8FD0\u884C";
          btnStartAgent.disabled = false;
        }
        cdpPortText.textContent = status.cdpPort || "\u672A\u7ED1\u5B9A";
        autoLaunchToggle.checked = appConfig.autoLaunchAgent;
        themeEnabledToggle.checked = appConfig.enabled;
        updateAgentSelector(appConfig.selectedAgent || "codex");
        return status;
      } catch (err) {
        notify("\u65E0\u6CD5\u83B7\u53D6\u540E\u7AEF\u72B6\u6001", "error");
        console.error(err);
      }
    }
    function updateAgentSelector(agent) {
      const btns = document.querySelectorAll(".agent-btn");
      btns.forEach((btn) => {
        if (btn.dataset.agent === agent) {
          btn.classList.add("active");
        } else {
          btn.classList.remove("active");
        }
      });
    }
  }
});
export default require_app();
