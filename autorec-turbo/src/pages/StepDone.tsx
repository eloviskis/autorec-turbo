import { useSetupStore } from "../store/setup";
import { launchObs } from "../lib/commands";

export default function StepDone() {
  const { name, reset } = useSetupStore();

  async function handleOpen() {
    await launchObs().catch(() => {});
  }

  return (
    <div className="flex flex-col items-center justify-center h-full text-center px-10 gap-6">
      {/* Ícone de sucesso */}
      <div className="w-20 h-20 rounded-full bg-emerald-600/20 border-2 border-emerald-500/40 flex items-center justify-center">
        <svg
          className="w-10 h-10 text-emerald-400"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          strokeWidth={2}
        >
          <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
        </svg>
      </div>

      <div className="space-y-2">
        <h2 className="text-3xl font-bold text-white">
          {name ? `Pronto, ${name.split(" ")[0]}!` : "Pronto!"}
        </h2>
        <p className="text-zinc-400 text-sm max-w-xs">
          O OBS está configurado com 5 cenas prontas para gravar. Clique em
          Abrir OBS para começar.
        </p>
      </div>

      {/* O que foi criado */}
      <ul className="text-left space-y-1.5 w-full max-w-xs">
        {[
          "Você + Slides",
          "Tela Cheia",
          "Tablet / Escrita",
          "Pré-Evento",
          "Live / Reunião",
        ].map((scene) => (
          <li key={scene} className="flex items-center gap-2 text-sm text-zinc-300">
            <span className="text-emerald-400 text-xs">✓</span>
            {scene}
          </li>
        ))}
      </ul>

      <div className="flex flex-col gap-2 w-full max-w-xs">
        <button
          onClick={handleOpen}
          className="w-full bg-emerald-600 hover:bg-emerald-500 active:bg-emerald-700 text-white font-semibold py-3 rounded-xl transition-colors"
        >
          Abrir OBS →
        </button>
        <button
          onClick={reset}
          className="w-full text-zinc-500 hover:text-zinc-300 text-sm py-2 transition-colors"
        >
          Configurar outro computador
        </button>
      </div>
    </div>
  );
}
