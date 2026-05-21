import { useEffect, useRef, useState } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { useSetupStore } from "../store/setup";
import {
  checkObsInstalled,
  checkObsRunning,
  installObs,
  backupObsConfig,
  injectScenes,
} from "../lib/commands";

type PhaseStatus = "pending" | "running" | "done" | "error";

interface Phase {
  id: string;
  label: string;
  status: PhaseStatus;
}

const PHASES: Omit<Phase, "status">[] = [
  { id: "check_obs", label: "Verificar / instalar OBS" },
  { id: "backup", label: "Backup da configuração atual" },
  { id: "inject", label: "Injetar cenas e perfil" },
];

export default function StepInstalling() {
  const { name, instagram, youtube, accentColor, addLog, setStep, setError } =
    useSetupStore();

  const [phases, setPhases] = useState<Phase[]>(
    PHASES.map((p) => ({ ...p, status: "pending" }))
  );
  const [globalError, setGlobalError] = useState<string | null>(null);
  const logsRef = useRef<HTMLDivElement>(null);
  const [logs, setLogs] = useState<string[]>([]);

  function appendLog(msg: string) {
    setLogs((prev) => [...prev, msg]);
    addLog(msg);
  }

  function setPhaseStatus(id: string, status: PhaseStatus) {
    setPhases((prev) =>
      prev.map((p) => (p.id === id ? { ...p, status } : p))
    );
  }

  useEffect(() => {
    let unlisten: UnlistenFn | null = null;
    let cancelled = false;

    async function run() {
      // Escuta logs do Rust
      unlisten = await listen<string>("install_log", (e) => {
        if (!cancelled) appendLog(e.payload);
      });

      try {
        // ── Fase 1: OBS ────────────────────────────────────────────────────
        setPhaseStatus("check_obs", "running");
        appendLog("Verificando OBS...");

        const obsRunning = await checkObsRunning();
        if (obsRunning) {
          throw new Error(
            "O OBS está aberto. Feche-o e tente novamente."
          );
        }

        const obsInstalled = await checkObsInstalled();
        if (!obsInstalled) {
          appendLog("OBS não encontrado. Iniciando instalação...");
          await installObs();
        } else {
          appendLog("OBS já instalado. Pulando download.");
        }
        setPhaseStatus("check_obs", "done");

        // ── Fase 2: Backup ─────────────────────────────────────────────────
        setPhaseStatus("backup", "running");
        appendLog("Fazendo backup da configuração existente...");
        const backupPath = await backupObsConfig();
        if (backupPath === "sem-backup") {
          appendLog("Nenhuma config anterior. Sem backup necessário.");
        } else {
          appendLog(`Backup salvo em: ${backupPath}`);
        }
        setPhaseStatus("backup", "done");

        // ── Fase 3: Injeção ────────────────────────────────────────────────
        setPhaseStatus("inject", "running");
        appendLog("Injetando coleção de cenas...");
        await injectScenes({ name, instagram, youtube, accentColor });
        setPhaseStatus("inject", "done");

        appendLog("✓ Tudo pronto!");
        setTimeout(() => setStep("done"), 800);
      } catch (err: unknown) {
        const msg = String(err);
        setGlobalError(msg);
        setError(msg);
        appendLog(`ERRO: ${msg}`);
        // Marca a fase em andamento como erro
        setPhases((prev) =>
          prev.map((p) => (p.status === "running" ? { ...p, status: "error" } : p))
        );
      }
    }

    run();

    return () => {
      cancelled = true;
      unlisten?.();
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Auto-scroll nos logs
  useEffect(() => {
    if (logsRef.current) {
      logsRef.current.scrollTop = logsRef.current.scrollHeight;
    }
  }, [logs]);

  return (
    <div className="flex flex-col h-full px-8 py-6 gap-4">
      <div className="text-center space-y-1">
        <h2 className="text-2xl font-bold text-white">Instalando…</h2>
        <p className="text-zinc-400 text-sm">Não feche esta janela.</p>
      </div>

      {/* Fases */}
      <div className="space-y-2 w-full max-w-sm mx-auto">
        {phases.map((phase) => (
          <div
            key={phase.id}
            className="flex items-center gap-3 px-4 py-2.5 rounded-xl bg-zinc-800/60 border border-zinc-700/50"
          >
            <PhaseIcon status={phase.status} />
            <span
              className={`text-sm font-medium ${
                phase.status === "done"
                  ? "text-emerald-400"
                  : phase.status === "error"
                  ? "text-red-400"
                  : phase.status === "running"
                  ? "text-white"
                  : "text-zinc-500"
              }`}
            >
              {phase.label}
            </span>
          </div>
        ))}
      </div>

      {/* Log terminal */}
      <div
        ref={logsRef}
        className="flex-1 bg-zinc-950 border border-zinc-800 rounded-xl p-3 overflow-y-auto font-mono text-xs text-zinc-400 space-y-0.5"
      >
        {logs.map((l, i) => (
          <div key={i} className="leading-relaxed">
            <span className="text-zinc-600 select-none">▸ </span>
            {l}
          </div>
        ))}
      </div>

      {globalError && (
        <div className="bg-red-950/60 border border-red-800/60 rounded-xl px-4 py-3 text-red-300 text-sm text-center">
          {globalError}
          <br />
          <button
            onClick={() => setStep("personalization")}
            className="mt-2 text-red-400 underline text-xs"
          >
            Voltar e tentar novamente
          </button>
        </div>
      )}
    </div>
  );
}

function PhaseIcon({ status }: { status: PhaseStatus }) {
  if (status === "done")
    return <span className="text-emerald-400 text-base shrink-0">✓</span>;
  if (status === "error")
    return <span className="text-red-400 text-base shrink-0">✗</span>;
  if (status === "running")
    return (
      <svg
        className="w-4 h-4 animate-spin text-indigo-400 shrink-0"
        fill="none"
        viewBox="0 0 24 24"
      >
        <circle
          className="opacity-25"
          cx="12"
          cy="12"
          r="10"
          stroke="currentColor"
          strokeWidth="4"
        />
        <path
          className="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8v8H4z"
        />
      </svg>
    );
  return <span className="w-4 h-4 rounded-full border border-zinc-600 shrink-0 inline-block" />;
}
