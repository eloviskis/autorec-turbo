use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};
use tauri::Emitter;

// ── Payload recebido do frontend ─────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InjectPayload {
    pub name: String,
    pub instagram: String,
    pub youtube: String,
    pub accent_color: String,
}

// ── Helpers de path ───────────────────────────────────────────────────────────

fn obs_config_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    return dirs::home_dir()
        .expect("home dir não encontrado")
        .join("Library/Application Support/obs-studio");

    #[cfg(target_os = "windows")]
    return dirs::data_dir()
        .expect("AppData não encontrado")
        .join("obs-studio");

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return dirs::home_dir()
        .expect("home dir não encontrado")
        .join(".config/obs-studio");
}

// ── Comandos públicos ─────────────────────────────────────────────────────────

#[tauri::command]
pub fn check_obs_installed() -> bool {
    #[cfg(target_os = "macos")]
    return std::path::Path::new("/Applications/OBS.app").exists();

    #[cfg(target_os = "windows")]
    return std::path::Path::new(r"C:\Program Files\obs-studio\bin\64bit\obs64.exe").exists();

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return Command::new("which").arg("obs").output().map(|o| o.status.success()).unwrap_or(false);
}

#[tauri::command]
pub fn check_obs_running() -> bool {
    #[cfg(target_os = "macos")]
    return Command::new("pgrep")
        .args(["-x", "OBS"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    #[cfg(target_os = "windows")]
    return Command::new("tasklist")
        .args(["/FI", "IMAGENAME eq obs64.exe", "/NH"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("obs64.exe"))
        .unwrap_or(false);

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return Command::new("pgrep").arg("obs").output().map(|o| o.status.success()).unwrap_or(false);
}

/// Instala o OBS via Homebrew (se disponível) ou baixa o DMG diretamente.
/// Emite eventos "install_log" com progresso.
#[tauri::command]
pub async fn install_obs(window: tauri::Window) -> Result<(), String> {
    if check_obs_installed() {
        let _ = window.emit("install_log", "OBS já está instalado.");
        return Ok(());
    }
    install_obs_platform(window).await
}

#[cfg(target_os = "macos")]
async fn install_obs_platform(window: tauri::Window) -> Result<(), String> {
    // Prefere Homebrew (mais rápido e seguro)
    if homebrew_available() {
        let _ = window.emit("install_log", "Instalando OBS via Homebrew...");
        let status = Command::new("brew")
            .args(["install", "--cask", "obs"])
            .status()
            .map_err(|e| e.to_string())?;

        if status.success() {
            let _ = window.emit("install_log", "OBS instalado com sucesso via Homebrew!");
            return Ok(());
        }
        let _ = window.emit("install_log", "Homebrew falhou. Tentando download direto...");
    }

    let obs_url =
        "https://github.com/obsproject/obs-studio/releases/download/31.0.3/OBS-31.0.3-macOS-Universal.dmg";
    let tmp_dmg = "/tmp/obs-installer.dmg";

    let _ = window.emit("install_log", "Baixando OBS Studio (isso pode levar alguns minutos)...");

    let status = Command::new("curl")
        .args(["-L", "-o", tmp_dmg, obs_url])
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err("Falha ao baixar OBS. Verifique sua conexão e tente novamente.".into());
    }

    let _ = window.emit("install_log", "Montando imagem do instalador...");

    let output = Command::new("hdiutil")
        .args(["attach", tmp_dmg, "-nobrowse"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err("Falha ao montar o DMG do OBS.".into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mount_point = stdout
        .lines()
        .find(|l| l.contains("/Volumes/"))
        .and_then(|l| l.split_whitespace().find(|s| s.starts_with("/Volumes/")))
        .ok_or("Não foi possível localizar o volume montado do OBS.")?
        .to_string();

    let app_src = fs::read_dir(&mount_point)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .find(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                == Some("app")
        })
        .map(|e| e.path())
        .ok_or("OBS.app não encontrado dentro do DMG.")?;

    let _ = window.emit("install_log", "Copiando OBS para /Applications...");

    let status = Command::new("cp")
        .args(["-R", app_src.to_str().unwrap(), "/Applications/OBS.app"])
        .status()
        .map_err(|e| e.to_string())?;

    let _ = Command::new("hdiutil")
        .args(["detach", &mount_point, "-quiet"])
        .output();
    let _ = fs::remove_file(tmp_dmg);

    if !status.success() {
        return Err("Falha ao copiar OBS para /Applications. Tente instalar manualmente.".into());
    }

    let _ = window.emit("install_log", "OBS instalado com sucesso!");
    Ok(())
}

#[cfg(target_os = "windows")]
async fn install_obs_platform(window: tauri::Window) -> Result<(), String> {
    let obs_url =
        "https://github.com/obsproject/obs-studio/releases/download/31.0.3/OBS-31.0.3-Windows-Installer.exe";
    let tmp_exe = std::env::temp_dir().join("obs-installer.exe");
    let tmp_exe_str = tmp_exe.to_str().unwrap().to_string();

    let _ = window.emit("install_log", "Baixando OBS Studio...");

    let status = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!("Invoke-WebRequest -Uri '{}' -OutFile '{}'", obs_url, tmp_exe_str),
        ])
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err("Falha ao baixar OBS. Verifique sua conexão.".into());
    }

    let _ = window.emit("install_log", "Instalando OBS Studio (aguarde)...");

    let status = Command::new(&tmp_exe)
        .arg("/S") // silent install NSIS
        .status()
        .map_err(|e| e.to_string())?;

    let _ = fs::remove_file(&tmp_exe);

    if !status.success() {
        return Err("Falha ao instalar OBS. Tente instalar manualmente.".into());
    }

    let _ = window.emit("install_log", "OBS instalado com sucesso!");
    Ok(())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
async fn install_obs_platform(window: tauri::Window) -> Result<(), String> {
    // Tenta apt-get (Ubuntu / Debian)
    if Command::new("which").arg("apt-get").output().map(|o| o.status.success()).unwrap_or(false) {
        let _ = window.emit("install_log", "Instalando OBS via apt-get...");
        let status = Command::new("pkexec")
            .args(["apt-get", "install", "-y", "obs-studio"])
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            let _ = window.emit("install_log", "OBS instalado com sucesso!");
            return Ok(());
        }
        let _ = window.emit("install_log", "apt-get falhou. Tentando Flatpak...");
    }

    // Tenta flatpak (universal — funciona em qualquer distro com Flatpak)
    if Command::new("which").arg("flatpak").output().map(|o| o.status.success()).unwrap_or(false) {
        let _ = window.emit("install_log", "Instalando OBS via Flatpak (Flathub)...");
        let status = Command::new("flatpak")
            .args(["install", "--noninteractive", "-y", "flathub", "com.obsproject.Studio"])
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            let _ = window.emit("install_log", "OBS instalado via Flatpak com sucesso!");
            return Ok(());
        }
    }

    Err("Não foi possível instalar o OBS automaticamente. Instale manualmente:\n  Ubuntu/Debian: sudo apt-get install obs-studio\n  Flatpak: flatpak install flathub com.obsproject.Studio".into())
}

/// Faz backup da config OBS existente — cross-platform.
/// Retorna o caminho do backup, ou "sem-backup" se não havia config.
#[tauri::command]
pub fn backup_obs_config() -> Result<String, String> {
    let config_dir = obs_config_dir();
    if !config_dir.exists() {
        return Ok("sem-backup".into());
    }

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();

    #[cfg(target_os = "macos")]
    let backup_path = dirs::home_dir()
        .unwrap()
        .join(format!("Library/Application Support/obs-studio-backup-{}", timestamp));

    #[cfg(target_os = "windows")]
    let backup_path = dirs::data_dir()
        .unwrap()
        .join(format!("obs-studio-backup-{}", timestamp));

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let backup_path = dirs::home_dir()
        .unwrap()
        .join(format!(".config/obs-studio-backup-{}", timestamp));

    copy_dir_recursive(&config_dir, &backup_path)?;
    Ok(backup_path.to_str().unwrap().to_string())
}

/// Injeta o Scene Collection + perfil de saída e ativa tudo no global.ini.
#[tauri::command]
pub fn inject_scenes(window: tauri::Window, payload: InjectPayload) -> Result<(), String> {
    let config_dir = obs_config_dir();
    let scenes_dir = config_dir.join("basic/scenes");
    let profile_dir = config_dir.join("basic/profiles/AutoREC Turbo");

    fs::create_dir_all(&scenes_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&profile_dir).map_err(|e| e.to_string())?;

    let _ = window.emit("install_log", "Construindo coleção de cenas...");
    let scene_json = build_scene_collection(&payload);
    fs::write(scenes_dir.join("AutoREC Turbo.json"), scene_json)
        .map_err(|e| e.to_string())?;

    let _ = window.emit("install_log", "Configurando perfil de saída (1080p 60fps)...");
    fs::write(profile_dir.join("basic.ini"), build_profile_ini())
        .map_err(|e| e.to_string())?;

    let _ = window.emit("install_log", "Ativando perfil e coleção no OBS...");
    update_global_ini(&config_dir)?;

    let _ = window.emit("install_log", "Cenas injetadas com sucesso!");
    Ok(())
}

#[tauri::command]
pub fn launch_obs() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    return Command::new("open")
        .arg("/Applications/OBS.app")
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string());

    #[cfg(target_os = "windows")]
    return Command::new(r"C:\Program Files\obs-studio\bin\64bit\obs64.exe")
        .spawn()
        .map(|_| ())
        .map_err(|e| e.to_string());

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return Command::new("obs").spawn().map(|_| ()).map_err(|e| e.to_string());
}

// ── Construtores de JSON do OBS ───────────────────────────────────────────────

fn make_audio_source(id: usize, name: &str, obs_id: &str) -> serde_json::Value {
    serde_json::json!({
        "balance": 0.5,
        "deinterlace_field_order": 0,
        "deinterlace_mode": 0,
        "enabled": true,
        "flags": 0,
        "hotkeys": {},
        "id": obs_id,
        "mixers": 255,
        "monitoring_type": 0,
        "muted": false,
        "name": name,
        "prev_ver": 503316481,
        "private_settings": {},
        "push-to-mute-delay": 0,
        "push-to-talk-delay": 0,
        "settings": { "device_id": "default" },
        "sync": 0,
        "versioned_id": obs_id,
        "volume": 1.0,
        "_id": id
    })
}

fn make_webcam_source() -> serde_json::Value {
    serde_json::json!({
        "balance": 0.5,
        "deinterlace_field_order": 0,
        "deinterlace_mode": 0,
        "enabled": true,
        "flags": 0,
        "hotkeys": {},
        "id": "av_capture_input_v2",
        "mixers": 0,
        "monitoring_type": 0,
        "muted": false,
        "name": "Webcam",
        "prev_ver": 503316481,
        "private_settings": {},
        "push-to-mute-delay": 0,
        "push-to-talk-delay": 0,
        "settings": {
            "device": "",
            "device_name": "",
            "frame_rate": { "denominator": 1, "numerator": 30 },
            "input_format": -1,
            "preset": -1,
            "resolution": ""
        },
        "sync": 0,
        "versioned_id": "av_capture_input_v2",
        "volume": 1.0
    })
}

fn make_display_source() -> serde_json::Value {
    serde_json::json!({
        "balance": 0.5,
        "deinterlace_field_order": 0,
        "deinterlace_mode": 0,
        "enabled": true,
        "flags": 0,
        "hotkeys": {},
        "id": "display_capture",
        "mixers": 0,
        "monitoring_type": 0,
        "muted": false,
        "name": "Captura de Tela",
        "prev_ver": 503316481,
        "private_settings": {},
        "push-to-mute-delay": 0,
        "push-to-talk-delay": 0,
        "settings": { "display": 0, "show_cursor": true },
        "sync": 0,
        "versioned_id": "display_capture",
        "volume": 1.0
    })
}

fn make_window_source() -> serde_json::Value {
    serde_json::json!({
        "balance": 0.5,
        "deinterlace_field_order": 0,
        "deinterlace_mode": 0,
        "enabled": true,
        "flags": 0,
        "hotkeys": {},
        "id": "window_capture",
        "mixers": 0,
        "monitoring_type": 0,
        "muted": false,
        "name": "Captura de Janela",
        "prev_ver": 503316481,
        "private_settings": {},
        "push-to-mute-delay": 0,
        "push-to-talk-delay": 0,
        "settings": { "owner_name": "", "show_cursor": true },
        "sync": 0,
        "versioned_id": "window_capture",
        "volume": 1.0
    })
}

fn make_text_source(name: &str, text: &str, color_hex: &str) -> serde_json::Value {
    // Converte hex "#RRGGBB" → ABGR uint32 que o OBS espera
    let color_int = hex_to_obs_color(color_hex);
    serde_json::json!({
        "balance": 0.5,
        "deinterlace_field_order": 0,
        "deinterlace_mode": 0,
        "enabled": true,
        "flags": 0,
        "hotkeys": {},
        "id": "text_ft2_source_v2",
        "mixers": 0,
        "monitoring_type": 0,
        "muted": false,
        "name": name,
        "prev_ver": 503316481,
        "private_settings": {},
        "push-to-mute-delay": 0,
        "push-to-talk-delay": 0,
        "settings": {
            "color1": color_int,
            "color2": color_int,
            "custom_width": 900,
            "drop_shadow": true,
            "font": { "face": "Helvetica Neue", "flags": 1, "size": 40, "style": "Bold" },
            "from_file": false,
            "outline": false,
            "text": text,
            "word_wrap": false
        },
        "sync": 0,
        "versioned_id": "text_ft2_source_v2",
        "volume": 1.0
    })
}

/// Constrói um item de cena com posição, escala e tamanho de bounds.
fn scene_item(
    source_name: &str,
    item_id: u32,
    pos_x: f64,
    pos_y: f64,
    scale_x: f64,
    scale_y: f64,
    visible: bool,
) -> serde_json::Value {
    serde_json::json!({
        "align": 5,
        "blend_method": 0,
        "blend_type": 0,
        "bounds": { "x": 0.0, "y": 0.0 },
        "bounds_align": 0,
        "bounds_type": 0,
        "crop_bottom": 0,
        "crop_left": 0,
        "crop_right": 0,
        "crop_top": 0,
        "id": item_id,
        "locked": false,
        "name": source_name,
        "pos": { "x": pos_x, "y": pos_y },
        "private_settings": {},
        "rot": 0.0,
        "scale": { "x": scale_x, "y": scale_y },
        "scale_filter": 0,
        "show_transition": {},
        "hide_transition": {},
        "visible": visible
    })
}

fn make_scene(name: &str, items: Vec<serde_json::Value>) -> serde_json::Value {
    serde_json::json!({
        "balance": 0.5,
        "deinterlace_field_order": 0,
        "deinterlace_mode": 0,
        "enabled": true,
        "flags": 0,
        "hotkeys": {},
        "id": "scene",
        "mixers": 0,
        "monitoring_type": 0,
        "muted": false,
        "name": name,
        "prev_ver": 503316481,
        "private_settings": {},
        "push-to-mute-delay": 0,
        "push-to-talk-delay": 0,
        "settings": {
            "custom_size": false,
            "id_counter": items.len() + 1,
            "items": items
        },
        "sync": 0,
        "versioned_id": "scene",
        "volume": 1.0
    })
}

/// Monta o JSON completo do Scene Collection do OBS.
fn build_scene_collection(payload: &InjectPayload) -> String {
    let display_text = format!(
        "{} | @{}",
        payload.name.trim(),
        payload.instagram.trim().trim_start_matches('@')
    );

    // ── Sources compartilhados ─────────────────────────────────────────────
    let webcam = make_webcam_source();
    let screen = make_display_source();
    let window = make_window_source();
    let name_bar = make_text_source("Faixa de Nome", &display_text, &payload.accent_color);

    // ── Cenas ──────────────────────────────────────────────────────────────

    // 1. Você + Slides: tela cheia + webcam 320×180 canto inferior direito + faixa
    let scene_slides = make_scene(
        "Você + Slides",
        vec![
            scene_item("Captura de Tela", 1, 0.0, 0.0, 1.0, 1.0, true),
            scene_item("Webcam", 2, 1600.0, 900.0, 0.1667, 0.1667, true),
            scene_item("Faixa de Nome", 3, 30.0, 980.0, 1.0, 1.0, true),
        ],
    );

    // 2. Tela Cheia: display capture + webcam menor
    let scene_fullscreen = make_scene(
        "Tela Cheia",
        vec![
            scene_item("Captura de Tela", 1, 0.0, 0.0, 1.0, 1.0, true),
            scene_item("Webcam", 2, 1580.0, 20.0, 0.1667, 0.1667, true),
        ],
    );

    // 3. Tablet / Escrita: captura de janela + webcam
    let scene_tablet = make_scene(
        "Tablet / Escrita",
        vec![
            scene_item("Captura de Janela", 1, 0.0, 0.0, 1.0, 1.0, true),
            scene_item("Webcam", 2, 1600.0, 900.0, 0.1667, 0.1667, true),
            scene_item("Faixa de Nome", 3, 30.0, 980.0, 1.0, 1.0, true),
        ],
    );

    // 4. Pré-Evento: só a faixa de nome centralizada
    let scene_pre = make_scene(
        "Pré-Evento",
        vec![scene_item("Faixa de Nome", 1, 510.0, 502.0, 1.0, 1.0, true)],
    );

    // 5. Live / Reunião: webcam centralizada ocupando o canvas
    let scene_live = make_scene(
        "Live / Reunião",
        vec![
            scene_item("Webcam", 1, 0.0, 0.0, 1.0, 1.0, true),
            scene_item("Faixa de Nome", 2, 30.0, 980.0, 1.0, 1.0, true),
        ],
    );

    let collection = serde_json::json!({
        "AuxAudioDevice1": make_audio_source(0, "Microfone", "coreaudio_input_capture"),
        "AuxAudioDevice2": null,
        "AuxAudioDevice3": null,
        "AuxAudioDevice4": null,
        "AuxAudioDevice5": null,
        "CurrentPreviewScene": "Você + Slides",
        "CurrentProgramScene": "Você + Slides",
        "DesktopAudioDevice1": make_audio_source(1, "Áudio do Sistema", "coreaudio_output_capture"),
        "DesktopAudioDevice2": null,
        "Modules": {},
        "SceneOrder": [
            { "name": "Você + Slides" },
            { "name": "Tela Cheia" },
            { "name": "Tablet / Escrita" },
            { "name": "Pré-Evento" },
            { "name": "Live / Reunião" }
        ],
        "Sources": [
            webcam,
            screen,
            window,
            name_bar,
            scene_slides,
            scene_fullscreen,
            scene_tablet,
            scene_pre,
            scene_live
        ],
        "Transitions": [{
            "duration": 300,
            "hotkeys": {},
            "id": "fade_transition",
            "name": "Fade",
            "settings": {}
        }],
        "name": "AutoREC Turbo"
    });

    serde_json::to_string_pretty(&collection).unwrap_or_default()
}

fn build_profile_ini() -> String {
    // Cria a pasta Videos se não existir
    let videos_path = dirs::video_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap().join("Videos"))
        .to_str()
        .unwrap_or("~/Videos")
        .to_string();

    format!(
        "[General]\n\
         Name=AutoREC Turbo\n\n\
         [Video]\n\
         BaseCX=1920\n\
         BaseCY=1080\n\
         OutputCX=1920\n\
         OutputCY=1080\n\
         FPSType=1\n\
         FPSNum=60\n\
         FPSDen=1\n\
         ColorFormat=NV12\n\
         ColorSpace=709\n\
         ColorRange=Partial\n\n\
         [Output]\n\
         Mode=Simple\n\n\
         [SimpleOutput]\n\
         FilePath={videos}\n\
         FileFormat=%CCYY-%MM-%DD %hh-%mm-%ss\n\
         RecFormat2=mkv\n\
         VBitrate=6000\n\
         RecQuality=Small\n\
         RecEncoder=obs_x264\n\
         RecEncoderID=obs_x264\n\n\
         [Audio]\n\
         SampleRate=44100\n\
         ChannelSetup=Stereo\n",
        videos = videos_path
    )
}

fn update_global_ini(config_dir: &std::path::Path) -> Result<(), String> {
    let ini_path = config_dir.join("global.ini");

    // Lê existente ou começa do zero
    let existing = fs::read_to_string(&ini_path).unwrap_or_default();

    let updated = set_ini_value(
        set_ini_value(
            set_ini_value(
                set_ini_value(
                    existing,
                    "Basic",
                    "Profile",
                    "AutoREC Turbo",
                ),
                "Basic",
                "ProfileDir",
                "AutoREC Turbo",
            ),
            "Basic",
            "SceneCollection",
            "AutoREC Turbo",
        ),
        "Basic",
        "SceneCollectionFile",
        "AutoREC Turbo",
    );

    fs::write(ini_path, updated).map_err(|e| e.to_string())
}

/// Atualiza (ou insere) uma chave dentro de uma seção de um INI simples.
fn set_ini_value(content: String, section: &str, key: &str, value: &str) -> String {
    let section_header = format!("[{}]", section);
    let key_line = format!("{}={}", key, value);
    let key_prefix = format!("{}=", key);

    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let mut in_section = false;
    let mut key_updated = false;
    let mut section_idx: Option<usize> = None;

    for (i, line) in lines.iter_mut().enumerate() {
        if line.trim() == section_header {
            in_section = true;
            section_idx = Some(i);
            continue;
        }
        if in_section {
            if line.starts_with('[') {
                in_section = false;
                continue;
            }
            if line.starts_with(&key_prefix) {
                *line = key_line.clone();
                key_updated = true;
                break;
            }
        }
    }

    if !key_updated {
        if let Some(idx) = section_idx {
            lines.insert(idx + 1, key_line);
        } else {
            lines.push(String::new());
            lines.push(section_header);
            lines.push(key_line);
        }
    }

    lines.join("\n")
}

// ── Utils ─────────────────────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
fn homebrew_available() -> bool {
    Command::new("brew")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Copia um diretório recursivamente (cross-platform, sem dependência de shell).
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// Converte "#RRGGBB" → inteiro ABGR uint32 que o OBS espera para cores de texto.
fn hex_to_obs_color(hex: &str) -> u32 {
    let hex = hex.trim_start_matches('#');
    if hex.len() < 6 {
        return 0xFFFFFFFF;
    }
    let r = u32::from_str_radix(&hex[0..2], 16).unwrap_or(255);
    let g = u32::from_str_radix(&hex[2..4], 16).unwrap_or(255);
    let b = u32::from_str_radix(&hex[4..6], 16).unwrap_or(255);
    // OBS usa ABGR (alpha=FF, então fica 0xFF_BB_GG_RR)
    0xFF000000 | (b << 16) | (g << 8) | r
}
