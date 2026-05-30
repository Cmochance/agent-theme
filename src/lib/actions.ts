import {
  configStore,
  themesStore,
  statusStore,
  lastErrorStore,
  isRestartingStore,
} from './stores';
import * as cmd from './tauri-commands';

function messageFromError(err: unknown) {
  if (typeof err === 'string') return err;
  if (err instanceof Error) return err.message;
  return 'The operation failed.';
}

export async function refreshStatus() {
  try {
    const status = await cmd.getAgentStatus();
    const config = await cmd.getConfig();
    statusStore.set(status);
    configStore.set(config);
  } catch (err) {
    console.error('Failed to refresh status:', err);
  }
}

export async function loadThemes() {
  try {
    const themes = await cmd.getAllThemes();
    themesStore.set(themes);
  } catch (err) {
    console.error('Failed to load themes:', err);
  }
}

export async function applyTheme(themeId: string) {
  lastErrorStore.set(null);
  try {
    await cmd.applyTheme(themeId);
    await refreshStatus();
    await loadThemes();
  } catch (err) {
    lastErrorStore.set(messageFromError(err));
    await refreshStatus();
  }
}

export async function restartAgent() {
  lastErrorStore.set(null);
  isRestartingStore.set(true);
  try {
    await cmd.restartAgent();
    await refreshStatus();
    await loadThemes();
  } catch (err) {
    lastErrorStore.set(messageFromError(err));
    await refreshStatus();
  } finally {
    isRestartingStore.set(false);
  }
}

export async function clearTheme() {
  lastErrorStore.set(null);
  try {
    await cmd.clearTheme();
    await refreshStatus();
    await loadThemes();
  } catch (err) {
    lastErrorStore.set(messageFromError(err));
    await refreshStatus();
  }
}

export async function deleteTheme() {
  await cmd.deleteCustomTheme();
  await refreshStatus();
  await loadThemes();
}

export async function setEnabled(enabled: boolean) {
  lastErrorStore.set(null);
  try {
    const config = await cmd.setEnabled(enabled);
    configStore.set(config);
  } catch (err) {
    lastErrorStore.set(messageFromError(err));
    await refreshStatus();
  }
}

export async function switchAgent(agent: 'codex' | 'antigravity') {
  lastErrorStore.set(null);
  try {
    const config = await cmd.setSelectedAgent(agent);
    configStore.set(config);
    await refreshStatus();
    await loadThemes();
  } catch (err) {
    lastErrorStore.set(messageFromError(err));
    await refreshStatus();
  }
}

export async function uploadTheme(bgBase64: string, previewBase64: string) {
  await cmd.uploadCustomTheme(bgBase64, previewBase64);
  await refreshStatus();
  await loadThemes();
}
