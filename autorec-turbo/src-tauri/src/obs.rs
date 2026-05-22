use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};
use tauri::Emitter;

// ── HTML Overlay Templates ──────────────────────────────────────────────────
// Placeholders substituídos em runtime: {{NAME}} {{INSTAGRAM}} {{YOUTUBE}} {{ACCENT}}

static TMPL_INICIANDO: &str = r####"<!DOCTYPE html><html><head><meta charset="UTF-8"><style>*{margin:0;padding:0;box-sizing:border-box}html,body{width:1920px;height:1080px;overflow:hidden;font-family:'Segoe UI',system-ui,sans-serif;background:#080810;color:#fff}.o{position:absolute;border-radius:50%;filter:blur(100px)}.o1{width:800px;height:800px;background:rgba(99,102,241,.3);top:-200px;left:-200px;animation:f1 10s ease-in-out infinite}.o2{width:500px;height:500px;background:rgba(129,140,248,.2);bottom:-150px;right:-100px;animation:f2 12s ease-in-out infinite}@keyframes f1{0%,100%{transform:translate(0,0)}50%{transform:translate(100px,80px)}}@keyframes f2{0%,100%{transform:translate(0,0)}50%{transform:translate(-80px,-60px)}}.glow{position:absolute;inset:0;background:radial-gradient(ellipse 55% 50% at 50% 40%,rgba(99,102,241,.18) 0%,transparent 65%);animation:br 5s ease-in-out infinite}@keyframes br{0%,100%{opacity:.6}50%{opacity:1}}.grid{position:absolute;inset:0;background-image:linear-gradient(rgba(99,102,241,.05) 1px,transparent 1px),linear-gradient(90deg,rgba(99,102,241,.05) 1px,transparent 1px);background-size:70px 70px}.c{position:absolute;inset:0;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:28px}.badge{display:inline-flex;align-items:center;gap:10px;background:rgba(99,102,241,.15);border:1px solid rgba(99,102,241,.45);border-radius:100px;padding:10px 28px;font-size:15px;font-weight:600;letter-spacing:.25em;text-transform:uppercase;color:#818cf8}.dot{width:10px;height:10px;border-radius:50%;background:{{ACCENT}};animation:bk 1.4s ease-in-out infinite}@keyframes bk{0%,100%{opacity:1}50%{opacity:.1}}h1{font-size:108px;font-weight:900;letter-spacing:-3px;line-height:1.05;text-align:center;background:linear-gradient(135deg,#fff 0%,#a5b4fc 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}.sub{font-size:26px;color:rgba(255,255,255,.4);letter-spacing:.1em}.ln{width:150px;height:2px;background:linear-gradient(90deg,transparent,{{ACCENT}},transparent)}.bot{position:absolute;bottom:0;left:0;right:0;height:130px;display:flex;align-items:center;justify-content:center;gap:48px;background:linear-gradient(transparent,rgba(0,0,0,.85))}.h{display:flex;align-items:center;gap:10px;font-size:22px;font-weight:500;color:rgba(255,255,255,.65)}.hi{color:{{ACCENT}}}.dv{width:4px;height:4px;border-radius:50%;background:rgba(255,255,255,.25)}</style></head><body><div class="o o1"></div><div class="o o2"></div><div class="grid"></div><div class="glow"></div><div class="c"><div class="badge"><span class="dot"></span>AO VIVO EM BREVE</div><h1>Come&ccedil;ando<br>em breve</h1><div class="ln"></div><p class="sub">Aguarde um momento&hellip;</p></div><div class="bot"><div class="h"><span class="hi">&#9670;</span>&nbsp;@{{INSTAGRAM}}</div><div class="dv"></div><div class="h"><span class="hi">&#9670;</span>&nbsp;{{YOUTUBE}}</div><div class="dv"></div><div class="h" style="color:#818cf8;font-weight:700">{{NAME}}</div></div></body></html>"####;

static TMPL_BRB: &str = r####"<!DOCTYPE html><html><head><meta charset="UTF-8"><style>*{margin:0;padding:0;box-sizing:border-box}html,body{width:1920px;height:1080px;overflow:hidden;font-family:'Segoe UI',system-ui,sans-serif;background:#080810;color:#fff}.orb{position:absolute;border-radius:50%;filter:blur(100px);width:700px;height:700px;background:rgba(99,102,241,.28);top:50%;left:50%;transform:translate(-50%,-50%);animation:pulse 5s ease-in-out infinite}@keyframes pulse{0%,100%{opacity:.6;transform:translate(-50%,-50%) scale(1)}50%{opacity:1;transform:translate(-50%,-50%) scale(1.12)}}.grid{position:absolute;inset:0;background-image:linear-gradient(rgba(99,102,241,.05) 1px,transparent 1px),linear-gradient(90deg,rgba(99,102,241,.05) 1px,transparent 1px);background-size:70px 70px}.c{position:absolute;inset:0;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:24px}.icon{font-size:80px;animation:bob 2s ease-in-out infinite}@keyframes bob{0%,100%{transform:translateY(0)}50%{transform:translateY(-14px)}}h1{font-size:112px;font-weight:900;letter-spacing:-3px;background:linear-gradient(135deg,#fff 0%,#a5b4fc 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}.sub{font-size:27px;color:rgba(255,255,255,.4);letter-spacing:.08em}.dots{display:flex;gap:12px;margin-top:6px}.d{width:12px;height:12px;border-radius:50%;background:{{ACCENT}};animation:sp 1.4s ease-in-out infinite}.d:nth-child(2){animation-delay:.2s}.d:nth-child(3){animation-delay:.4s}@keyframes sp{0%,80%,100%{transform:scale(.5);opacity:.3}40%{transform:scale(1);opacity:1}}.bot{position:absolute;bottom:0;left:0;right:0;height:100px;display:flex;align-items:center;justify-content:center;gap:40px;background:linear-gradient(transparent,rgba(0,0,0,.8));font-size:21px;color:rgba(255,255,255,.5);font-weight:500}.sep{color:rgba(255,255,255,.2)}</style></head><body><div class="orb"></div><div class="grid"></div><div class="c"><div class="icon">&#9749;</div><h1>Voltando j&aacute;!</h1><p class="sub">Pequena pausa&hellip;</p><div class="dots"><div class="d"></div><div class="d"></div><div class="d"></div></div></div><div class="bot"><span>@{{INSTAGRAM}}</span><span class="sep">&nbsp;/&nbsp;</span><span>{{YOUTUBE}}</span></div></body></html>"####;

static TMPL_ENCERRAMENTO: &str = r####"<!DOCTYPE html><html><head><meta charset="UTF-8"><style>*{margin:0;padding:0;box-sizing:border-box}html,body{width:1920px;height:1080px;overflow:hidden;font-family:'Segoe UI',system-ui,sans-serif;background:#080810;color:#fff}.o1{position:absolute;border-radius:50%;filter:blur(100px);width:900px;height:900px;background:rgba(99,102,241,.22);top:50%;left:50%;transform:translate(-50%,-55%);animation:br 6s ease-in-out infinite}.o2{position:absolute;border-radius:50%;filter:blur(100px);width:400px;height:400px;background:rgba(168,85,247,.18);bottom:-100px;left:200px}@keyframes br{0%,100%{opacity:.55}50%{opacity:1}}.grid{position:absolute;inset:0;background-image:linear-gradient(rgba(99,102,241,.05) 1px,transparent 1px),linear-gradient(90deg,rgba(99,102,241,.05) 1px,transparent 1px);background-size:70px 70px}.c{position:absolute;inset:0;display:flex;flex-direction:column;align-items:center;justify-content:center;gap:20px}.wave{font-size:78px;animation:wv 2.5s ease-in-out infinite}@keyframes wv{0%,100%{transform:rotate(0)}25%{transform:rotate(22deg)}75%{transform:rotate(-16deg)}}h1{font-size:102px;font-weight:900;letter-spacing:-3px;background:linear-gradient(135deg,#fff 30%,#a5b4fc 100%);-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text}.sub{font-size:27px;color:rgba(255,255,255,.45);max-width:800px;text-align:center;line-height:1.5}.cards{display:flex;gap:24px;margin-top:12px}.card{background:rgba(99,102,241,.15);border:1px solid rgba(99,102,241,.4);border-radius:16px;padding:16px 36px;text-align:center}.plat{font-size:13px;letter-spacing:.15em;text-transform:uppercase;color:#818cf8;margin-bottom:4px}.handle{font-size:25px;font-weight:700}.cta{font-size:16px;letter-spacing:.1em;text-transform:uppercase;color:rgba(255,255,255,.3);margin-top:6px}</style></head><body><div class="o1"></div><div class="o2"></div><div class="grid"></div><div class="c"><div class="wave">&#128075;</div><h1>Obrigado!</h1><p class="sub">Foi &oacute;timo ter voc&ecirc;s aqui. Sigam para mais conte&uacute;do!</p><div class="cards"><div class="card"><div class="plat">Instagram</div><div class="handle">@{{INSTAGRAM}}</div></div><div class="card"><div class="plat">YouTube</div><div class="handle">{{YOUTUBE}}</div></div></div><p class="cta">At&eacute; a pr&oacute;xima &#10024;</p></div></body></html>"####;

static TMPL_LOWER_THIRD: &str = r####"<!DOCTYPE html><html><head><meta charset="UTF-8"><style>*{margin:0;padding:0;box-sizing:border-box}html,body{width:1920px;height:1080px;overflow:hidden;background:transparent}.lt{position:absolute;bottom:0;left:0;right:0;height:104px;display:flex;align-items:center;padding:0 64px;background:linear-gradient(90deg,rgba(5,5,15,.96) 0%,rgba(5,5,15,.92) 55%,transparent 100%);border-top:3px solid {{ACCENT}};animation:si .7s cubic-bezier(.16,1,.3,1) both}@keyframes si{from{transform:translateY(100%);opacity:0}to{transform:translateY(0);opacity:1}}.bar{width:5px;height:64px;background:{{ACCENT}};border-radius:3px;margin-right:22px;flex-shrink:0}.info{display:flex;flex-direction:column;gap:5px;flex:1;font-family:'Segoe UI',system-ui,sans-serif}.name{font-size:32px;font-weight:700;color:#fff;letter-spacing:-.5px}.social{font-size:19px;color:rgba(255,255,255,.55)}.logo{width:110px;height:62px;border:2px dashed rgba(255,255,255,.15);border-radius:10px;display:flex;align-items:center;justify-content:center;color:rgba(255,255,255,.2);font-size:10px;letter-spacing:.1em;text-align:center;line-height:1.5;font-family:'Segoe UI',system-ui,sans-serif}</style></head><body><div class="lt"><div class="bar"></div><div class="info"><div class="name">{{NAME}}</div><div class="social">@{{INSTAGRAM}}&nbsp;&middot;&nbsp;{{YOUTUBE}}</div></div><div class="logo">SEU<br>LOGO</div></div></body></html>"####;

fn render_tmpl(tmpl: &str, payload: &InjectPayload) -> String {
    let instagram = payload.instagram.trim().trim_start_matches('@');
    tmpl.replace("{{NAME}}", payload.name.trim())
        .replace("{{INSTAGRAM}}", instagram)
        .replace("{{YOUTUBE}}", payload.youtube.trim())
        .replace("{{ACCENT}}", &payload.accent_color)
}

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

    let overlay_dir = config_dir.join("autorec-overlays");
    fs::create_dir_all(&overlay_dir).map_err(|e| e.to_string())?;

    let _ = window.emit("install_log", "Gerando overlays personalizados...");
    write_overlay_files(&overlay_dir, &payload)?;
    let _ = window.emit("install_log", "Construindo coleção de cenas...");
    let scene_json = build_scene_collection(&payload, &overlay_dir);
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

fn make_webcam_source(obs_id: &str) -> serde_json::Value {
    #[cfg(target_os = "macos")]
    let settings = serde_json::json!({ "device": "", "device_name": "", "frame_rate": { "denominator": 1, "numerator": 30 }, "input_format": -1, "preset": -1, "resolution": "" });
    #[cfg(target_os = "windows")]
    let settings = serde_json::json!({ "video_device_id": "", "last_resolution": "", "fps": 0 });
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let settings = serde_json::json!({ "device_id": "", "pixelformat": 0, "resolution": "" });
    serde_json::json!({
        "deinterlace_field_order": 0, "deinterlace_mode": 0, "enabled": true, "flags": 0,
        "hotkeys": {}, "id": obs_id, "mixers": 0, "monitoring_type": 0, "muted": false,
        "name": "Webcam", "prev_ver": null, "private_settings": {},
        "push-to-mute-delay": 0, "push-to-talk-delay": 0, "settings": settings,
        "sync": 0, "versioned_id": obs_id, "volume": 1.0
    })
}

fn make_display_source(obs_id: &str) -> serde_json::Value {
    #[cfg(target_os = "macos")]
    let settings = serde_json::json!({ "display": 0, "show_cursor": true });
    #[cfg(target_os = "windows")]
    let settings = serde_json::json!({ "monitor": 0, "show_cursor": true });
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let settings = serde_json::json!({ "show_cursor": true });
    serde_json::json!({
        "deinterlace_field_order": 0, "deinterlace_mode": 0, "enabled": true, "flags": 0,
        "hotkeys": {}, "id": obs_id, "mixers": 0, "monitoring_type": 0, "muted": false,
        "name": "Captura de Tela", "prev_ver": null, "private_settings": {},
        "push-to-mute-delay": 0, "push-to-talk-delay": 0, "settings": settings,
        "sync": 0, "versioned_id": obs_id, "volume": 1.0
    })
}

#[allow(dead_code)]
fn make_window_source(obs_id: &str) -> serde_json::Value {
    #[cfg(target_os = "windows")]
    let settings = serde_json::json!({ "window": "", "show_cursor": true });
    #[cfg(not(target_os = "windows"))]
    let settings = serde_json::json!({ "owner_name": "", "show_cursor": true });
    serde_json::json!({
        "deinterlace_field_order": 0, "deinterlace_mode": 0, "enabled": true, "flags": 0,
        "hotkeys": {}, "id": obs_id, "mixers": 0, "monitoring_type": 0, "muted": false,
        "name": "Captura de Janela", "prev_ver": null, "private_settings": {},
        "push-to-mute-delay": 0, "push-to-talk-delay": 0, "settings": settings,
        "sync": 0, "versioned_id": obs_id, "volume": 1.0
    })
}

#[allow(dead_code)]
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

fn write_overlay_files(dir: &std::path::Path, payload: &InjectPayload) -> Result<(), String> {
    let files: &[(&str, &str)] = &[
        ("iniciando.html",    TMPL_INICIANDO),
        ("brb.html",          TMPL_BRB),
        ("encerramento.html", TMPL_ENCERRAMENTO),
        ("lower-third.html",  TMPL_LOWER_THIRD),
    ];
    for (name, tmpl) in files {
        fs::write(dir.join(name), render_tmpl(tmpl, payload))
            .map_err(|e| format!("Erro ao gravar overlay {}: {}", name, e))?;
    }
    Ok(())
}

/// Monta o JSON completo do Scene Collection do OBS.
fn build_scene_collection(payload: &InjectPayload, overlay_dir: &std::path::Path) -> String {
    let ov = |f: &str| overlay_dir.join(f).to_string_lossy().into_owned();

    let browser = |name: &str, file: &str| -> serde_json::Value {
        serde_json::json!({
            "deinterlace_field_order": 0, "deinterlace_mode": 0, "enabled": true, "flags": 0,
            "hotkeys": {}, "id": "browser_source", "mixers": 0, "monitoring_type": 0, "muted": false,
            "name": name, "prev_ver": null, "private_settings": {},
            "push-to-mute-delay": 0, "push-to-talk-delay": 0,
            "settings": {
                "css": "body{background-color:rgba(0,0,0,0);margin:0;overflow:hidden}",
                "fps": 30, "fps_custom": false, "height": 1080, "is_local_file": true,
                "local_file": ov(file), "reroute_audio": false, "restart_when_active": true,
                "shutdown": false, "url": "", "webpage_control_level": 1, "width": 1920
            },
            "sync": 0, "versioned_id": "browser_source", "volume": 1.0
        })
    };

    #[cfg(target_os = "macos")]
    let (audio_in_id, audio_out_id, webcam_id, screen_id) = (
        "coreaudio_input_capture", "coreaudio_output_capture",
        "av_capture_input_v2", "display_capture",
    );
    #[cfg(target_os = "windows")]
    let (audio_in_id, audio_out_id, webcam_id, screen_id) = (
        "wasapi_input_capture", "wasapi_output_capture",
        "dshow_input", "monitor_capture",
    );
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let (audio_in_id, audio_out_id, webcam_id, screen_id) = (
        "pulse_input_capture", "pulse_output_capture",
        "v4l2_input", "xshm_input",
    );

    let webcam = make_webcam_source(webcam_id);
    let screen = make_display_source(screen_id);

    // ── 6 Cenas profissionais ──────────────────────────────────────────────

    let sc_start = make_scene(
        "\u{1F3AC} Iniciando",
        vec![scene_item("Overlay: Iniciando", 1, 0.0, 0.0, 1.0, 1.0, true)],
    );
    let sc_cam = make_scene(
        "\u{1F4F8} C\u{E2}mera",
        vec![
            scene_item("Webcam",      1, 0.0, 0.0, 1.0, 1.0, true),
            scene_item("Lower Third", 2, 0.0, 0.0, 1.0, 1.0, true),
        ],
    );
    let sc_screen = make_scene(
        "\u{1F5A5}\u{FE0F} Tela",
        vec![
            scene_item("Captura de Tela", 1, 0.0, 0.0, 1.0, 1.0, true),
            scene_item("Lower Third",     2, 0.0, 0.0, 1.0, 1.0, true),
        ],
    );
    let sc_pip = make_scene(
        "\u{1F3A5} C\u{E2}mera + Tela",
        vec![
            scene_item("Captura de Tela", 1,    0.0,   0.0, 1.0,  1.0,  true),
            scene_item("Webcam",          2, 1556.0, 862.0, 0.25, 0.25, true),
            scene_item("Lower Third",     3,    0.0,   0.0, 1.0,  1.0,  true),
        ],
    );
    let sc_brb = make_scene(
        "\u{2615} BRB",
        vec![scene_item("Overlay: BRB", 1, 0.0, 0.0, 1.0, 1.0, true)],
    );
    let sc_end = make_scene(
        "\u{1F44B} Encerramento",
        vec![scene_item("Overlay: Encerramento", 1, 0.0, 0.0, 1.0, 1.0, true)],
    );

    let collection = serde_json::json!({
        "AuxAudioDevice1": make_audio_source(0, "Microfone", audio_in_id),
        "AuxAudioDevice2": null,
        "AuxAudioDevice3": null,
        "AuxAudioDevice4": null,
        "AuxAudioDevice5": null,
        "DesktopAudioDevice1": make_audio_source(1, "\u{C1}udio do Sistema", audio_out_id),
        "DesktopAudioDevice2": null,
        "current_scene": "\u{1F3AC} Iniciando",
        "current_program_scene": "\u{1F3AC} Iniciando",
        "modules": {},
        "name": "AutoREC Turbo",
        "scene_order": [
            {"name": "\u{1F3AC} Iniciando"},
            {"name": "\u{1F4F8} C\u{E2}mera"},
            {"name": "\u{1F5A5}\u{FE0F} Tela"},
            {"name": "\u{1F3A5} C\u{E2}mera + Tela"},
            {"name": "\u{2615} BRB"},
            {"name": "\u{1F44B} Encerramento"}
        ],
        "sources": [
            browser("Overlay: Iniciando",    "iniciando.html"),
            browser("Overlay: BRB",          "brb.html"),
            browser("Overlay: Encerramento", "encerramento.html"),
            browser("Lower Third",           "lower-third.html"),
            webcam,
            screen,
            sc_start,
            sc_cam,
            sc_screen,
            sc_pip,
            sc_brb,
            sc_end
        ],
        "transitions": [{
            "duration": 300,
            "hotkeys": {},
            "id": "fade_transition",
            "name": "Fade",
            "settings": {}
        }]
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
