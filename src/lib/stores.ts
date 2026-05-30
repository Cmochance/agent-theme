import { writable } from 'svelte/store';
import type { AppConfig, Theme, AgentStatus } from './types';

export const configStore = writable<AppConfig | null>(null);
export const themesStore = writable<Theme[]>([]);
export const statusStore = writable<AgentStatus | null>(null);
export const showUploadModal = writable(false);
export const lastErrorStore = writable<string | null>(null);
export const isRestartingStore = writable(false);
