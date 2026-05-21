use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::process::Command;

// ── Constantes (substitua pelos valores reais do seu projeto Supabase) ────────
/// URL da Edge Function de validação de licença
const SUPABASE_LICENSE_URL: &str =
    "https://cqedwdzoitxqpwwsjfty.supabase.co/functions/v1/validate-license";
/// Chave anon pública do Supabase (sem risco — só acessa a Edge Function)
const SUPABASE_ANON_KEY: &str = "sb_publishable_ftS7BuY-y-Mbp-0nYoss7g_iNZ2-S4f";

// ── Tipos ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
struct LicenseRequest {
    key: String,
    machine_id: String,
    action: String, // "validate" | "activate"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseResponse {
    pub valid: bool,
    pub activated: bool,
    pub email: Option<String>,
    pub error: Option<String>,
}

// ── Comandos públicos ─────────────────────────────────────────────────────────

/// Valida a chave sem ativar. Retorna { valid, email, error }.
#[tauri::command]
pub async fn validate_license(key: String) -> Result<LicenseResponse, String> {
    let machine_id = get_machine_id();
    call_license_api(&key, &machine_id, "validate").await
}

/// Valida e ativa a licença vinculando esta máquina (máx. 2 ativações por chave).
#[tauri::command]
pub async fn activate_license(key: String) -> Result<LicenseResponse, String> {
    let machine_id = get_machine_id();
    call_license_api(&key, &machine_id, "activate").await
}

// ── Helpers privados ──────────────────────────────────────────────────────────

/// Deriva um ID de máquina determinístico e anônimo a partir do serial do hardware.
/// Usa SHA-256(serial + salt) para não expor o serial diretamente.
pub fn get_machine_id() -> String {
    let serial = read_ioreg_serial();
    let mut hasher = Sha256::new();
    hasher.update(serial.as_bytes());
    hasher.update(b":autorec-turbo:v1"); // salt fixo
    hex::encode(hasher.finalize())
}

fn read_ioreg_serial() -> String {
    let output = match Command::new("ioreg")
        .args(["-rd1", "-c", "IOPlatformExpertDevice"])
        .output()
    {
        Ok(o) => o,
        Err(_) => return "unknown-mac".to_string(),
    };

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Linha de exemplo: "IOPlatformSerialNumber" = "ABCDE12345"
    stdout
        .lines()
        .find(|l| l.contains("IOPlatformSerialNumber"))
        .and_then(|l| l.splitn(2, " = ").nth(1))
        .map(|s| s.trim().trim_matches('"').to_string())
        .unwrap_or_else(|| "unknown-mac".to_string())
}

async fn call_license_api(
    key: &str,
    machine_id: &str,
    action: &str,
) -> Result<LicenseResponse, String> {
    // Modo de desenvolvimento: aceita qualquer chave que comece com "DEV-"
    if key.starts_with("DEV-") {
        return Ok(LicenseResponse {
            valid: true,
            activated: true,
            email: Some("dev@autorec.local".to_string()),
            error: None,
        });
    }

    let client = reqwest::Client::new();
    let body = LicenseRequest {
        key: key.to_string(),
        machine_id: machine_id.to_string(),
        action: action.to_string(),
    };

    let response = client
        .post(SUPABASE_LICENSE_URL)
        .header("apikey", SUPABASE_ANON_KEY)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Erro de rede ao validar licença: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Servidor de licença retornou erro {}.",
            response.status()
        ));
    }

    response
        .json::<LicenseResponse>()
        .await
        .map_err(|e| format!("Resposta inválida do servidor: {}", e))
}
