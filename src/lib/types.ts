export interface DisplayName {
  en: string;
  zh: string;
}

/** Optional per-theme styling for the modular Codex injection (see theme.rs). */
export interface ThemeStyle {
  ink?: string;
  ink2?: string;
  ink3?: string;
  ink4?: string;
  accent?: string;
  accentSoft?: string;
  glass?: string;
  glassStrong?: string;
  glassSoft?: string;
  border?: string;
  borderSoft?: string;
  borderStrong?: string;
  blur?: string;
  scrimTop?: string;
  scrimMid?: string;
  scrimBot?: string;
  baseColor?: string;
}

export interface Theme {
  id: string;
  displayName: DisplayName;
  isCustom: boolean;
  background: string;
  preview: string;
  previewDataUri: string;
  backgroundFit?: string | null;
  backgroundPosition?: string | null;
  style?: ThemeStyle | null;
  dir: string;
}

/** Supported agents (matches src-tauri AgentKind, serialized lowercase). */
export type AgentKind = 'codex' | 'antigravity' | 'linear';

export interface AppConfig {
  enabled: boolean;
  selectedThemeId: string;
  autoLaunchAgent: boolean;
  activeIdentifier: string | null;
  selectedAgent: AgentKind;
}

export interface AgentStatus {
  running: boolean;
  cdpPort: number | null;
  agent: string;
}
