import { invoke } from '@tauri-apps/api/core';
import type { AppConfig, Theme, AgentStatus } from './types';

export async function getConfig(): Promise<AppConfig> {
  return invoke<AppConfig>('get_config');
}

export async function setEnabled(enabled: boolean): Promise<AppConfig> {
  return invoke<AppConfig>('set_enabled', { enabled });
}

export async function setSelectedAgent(agent: 'codex' | 'antigravity'): Promise<AppConfig> {
  return invoke<AppConfig>('set_selected_agent', { agent });
}

export async function getAgentStatus(): Promise<AgentStatus> {
  return invoke<AgentStatus>('get_agent_status');
}

export async function getAllThemes(): Promise<Theme[]> {
  return invoke<Theme[]>('get_all_themes');
}

export async function applyTheme(themeId: string): Promise<void> {
  return invoke('apply_theme', { themeId });
}

export async function restartAgent(): Promise<void> {
  return invoke('restart_agent');
}

export async function clearTheme(): Promise<void> {
  return invoke('clear_theme');
}

export async function uploadCustomTheme(bgBase64: string, previewBase64: string): Promise<void> {
  return invoke('upload_custom_theme', { bgBase64, previewBase64 });
}

export async function deleteCustomTheme(): Promise<void> {
  return invoke('delete_custom_theme_cmd');
}
