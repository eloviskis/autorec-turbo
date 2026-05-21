mod license;
mod obs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // OBS
            obs::check_obs_installed,
            obs::check_obs_running,
            obs::install_obs,
            obs::backup_obs_config,
            obs::inject_scenes,
            obs::launch_obs,
            // Licença
            license::validate_license,
            license::activate_license,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
