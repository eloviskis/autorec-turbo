import { useState } from "react";
import { useSetupStore } from "../store/setup";
import { activateLicense } from "../lib/commands";

export default function StepLicense() {
  const { licenseKey, setLicenseKey, setEmail, setStep, setError, error } =
    useSetupStore();
  const [loading, setLoading] = useState(false);

  async function handleValidate() {
    const key = licenseKey.trim().toUpperCase();
    if (!key) return;

    setLoading(true);
    setError(null);

    try {
      const result = await activateLicense(key);

      if (!result.valid) {
        setError(result.error ?? "Chave inválida ou não encontrada.");
        return;
      }

      if (!result.activated) {
        setError(
          "Limite de ativações atingido (máx. 2 máquinas por licença). Contate o suporte."
        );
        return;
      }

      if (result.email) setEmail(result.email);
      setStep("personalization");
    } catch (e: unknown) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="flex flex-col items-center justify-center h-full px-10 gap-5">
      <div className="text-center space-y-1">
        <h2 className="text-2xl font-bold text-white">Ativar licença</h2>
        <p className="text-zinc-400 text-sm">
          Digite a chave recebida por e-mail após a compra.
        </p>
      </div>

      <div className="w-full max-w-sm space-y-3">
        <input
          type="text"
          value={licenseKey}
          onChange={(e) => setLicenseKey(e.target.value.toUpperCase())}
          onKeyDown={(e) => e.key === "Enter" && handleValidate()}
          placeholder="XXXX-XXXX-XXXX-XXXX"
          spellCheck={false}
          className="w-full bg-zinc-800 border border-zinc-700 focus:border-indigo-500 focus:outline-none text-white placeholder-zinc-500 rounded-xl px-4 py-3 text-center font-mono tracking-widest transition-colors"
        />

        {error && (
          <div className="bg-red-950/60 border border-red-800/60 rounded-xl px-4 py-3 text-red-300 text-sm text-center">
            {error}
          </div>
        )}

        <button
          onClick={handleValidate}
          disabled={!licenseKey.trim() || loading}
          className="w-full bg-indigo-600 hover:bg-indigo-500 disabled:opacity-40 disabled:cursor-not-allowed text-white font-semibold py-3 rounded-xl transition-colors"
        >
          {loading ? "Validando…" : "Ativar →"}
        </button>
      </div>

      <p className="text-zinc-600 text-xs text-center max-w-xs">
        Cada licença permite ativar em até 2 computadores. Não compartilhe sua
        chave.
      </p>
    </div>
  );
}
