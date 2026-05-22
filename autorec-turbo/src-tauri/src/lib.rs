mod license;
mod obs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Corrige glitch de renderização do WebKit no Linux (GPU compositing)
    #[cfg(target_os = "linux")]
    {
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }

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
