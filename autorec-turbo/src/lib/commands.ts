import { invoke } from "@tauri-apps/api/core";

export interface LicenseResponse {
  valid: boolean;
  activated: boolean;
  email?: string;
  error?: string;
}

export interface InjectPayload {
  name: string;
  instagram: string;
  youtube: string;
  accentColor: string;
}

// ── OBS ───────────────────────────────────────────────────────────────────────

export const checkObsInstalled = (): Promise<boolean> =>
  invoke("check_obs_installed");

export const checkObsRunning = (): Promise<boolean> =>
  invoke("check_obs_running");

export const installObs = (): Promise<void> => invoke("install_obs");

export const backupObsConfig = (): Promise<string> =>
  invoke("backup_obs_config");

export const injectScenes = (payload: InjectPayload): Promise<void> =>
  invoke("inject_scenes", { payload });

export const launchObs = (): Promise<void> => invoke("launch_obs");

// ── Licença ───────────────────────────────────────────────────────────────────

export const validateLicense = (key: string): Promise<LicenseResponse> =>
  invoke("validate_license", { key });

export const activateLicense = (key: string): Promise<LicenseResponse> =>
  invoke("activate_license", { key });
