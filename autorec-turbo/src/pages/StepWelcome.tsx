import { useSetupStore } from "../store/setup";

export default function StepWelcome() {
  const setStep = useSetupStore((s) => s.setStep);

  return (
    <div className="flex flex-col items-center justify-center h-full text-center px-10 gap-6">
      {/* Logo / ícone */}
      <div className="w-20 h-20 rounded-2xl bg-indigo-600 flex items-center justify-center shadow-lg shadow-indigo-900/40">
        <svg
          className="w-10 h-10 text-white"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          strokeWidth={1.5}
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            d="M15.75 10.5l4.72-4.72a.75.75 0 011.28.53v11.38a.75.75 0 01-1.28.53l-4.72-4.72M4.5 18.75h9a2.25 2.25 0 002.25-2.25v-9A2.25 2.25 0 0013.5 5.25h-9A2.25 2.25 0 002.25 7.5v9A2.25 2.25 0 004.5 18.75z"
          />
        </svg>
      </div>

      <div className="space-y-2">
        <h1 className="text-3xl font-bold text-white tracking-tight">
          AutoREC Turbo
        </h1>
        <p className="text-zinc-400 text-sm max-w-sm">
          Em menos de 2 minutos o OBS estará configurado com suas cenas, seu
          nome e pronto para gravar.
        </p>
      </div>

      {/* O que será feito */}
      <ul className="text-left space-y-2 w-full max-w-xs">
        {[
          "Valida sua licença",
          "Instala o OBS (se necessário)",
          "Cria backup da config atual",
          "Injeta 5 cenas prontas",
          "Abre o OBS configurado",
        ].map((item, i) => (
          <li key={i} className="flex items-center gap-2 text-sm text-zinc-300">
            <span className="w-5 h-5 rounded-full bg-indigo-600/20 border border-indigo-500/40 flex items-center justify-center text-indigo-400 text-xs font-bold shrink-0">
              {i + 1}
            </span>
            {item}
          </li>
        ))}
      </ul>

      <button
        onClick={() => setStep("license")}
        className="mt-2 w-full max-w-xs bg-indigo-600 hover:bg-indigo-500 active:bg-indigo-700 text-white font-semibold py-3 rounded-xl transition-colors"
      >
        Começar →
      </button>
    </div>
  );
}
