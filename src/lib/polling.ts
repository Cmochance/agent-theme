import { refreshStatus, loadThemes } from './actions';

let statusInterval: ReturnType<typeof setInterval> | null = null;

export function startPolling(intervalMs = 15000) {
  // Initial load
  refreshStatus();
  loadThemes();

  // Periodic refresh
  statusInterval = setInterval(refreshStatus, intervalMs);
}

export function stopPolling() {
  if (statusInterval) {
    clearInterval(statusInterval);
    statusInterval = null;
  }
}
