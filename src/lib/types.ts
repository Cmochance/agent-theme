export interface DisplayName {
  en: string;
  zh: string;
}

export interface Theme {
  id: string;
  displayName: DisplayName;
  isCustom: boolean;
  background: string;
  preview: string;
  previewDataUri: string;
  dir: string;
}

export interface AppConfig {
  enabled: boolean;
  selectedThemeId: string;
  autoLaunchAgent: boolean;
  activeIdentifier: string | null;
  selectedAgent: 'codex' | 'antigravity';
}

export interface AgentStatus {
  running: boolean;
  cdpPort: number | null;
  agent: string;
}
