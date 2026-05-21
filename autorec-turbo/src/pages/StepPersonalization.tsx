import { useSetupStore } from "../store/setup";

const ACCENT_PRESETS = [
  { label: "Índigo", value: "#6366f1" },
  { label: "Roxo", value: "#a855f7" },
  { label: "Rosa", value: "#ec4899" },
  { label: "Âmbar", value: "#f59e0b" },
  { label: "Esmeralda", value: "#10b981" },
  { label: "Ciano", value: "#06b6d4" },
];

export default function StepPersonalization() {
  const store = useSetupStore();
  const { name, instagram, youtube, accentColor, setPersonalization, setStep, error } =
    store;

  function canProceed() {
    return name.trim().length > 0;
  }

  return (
    <div className="flex flex-col h-full px-10 py-6 gap-5 overflow-y-auto">
      <div className="text-center space-y-1">
        <h2 className="text-2xl font-bold text-white">Personalização</h2>
        <p className="text-zinc-400 text-sm">
          Suas informações aparecerão na faixa de nome dentro do OBS.
        </p>
      </div>

      <div className="space-y-3 w-full max-w-sm mx-auto">
        {/* Nome */}
        <div className="space-y-1">
          <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider">
            Seu nome *
          </label>
          <input
            type="text"
            value={name}
            onChange={(e) =>
              setPersonalization({ name: e.target.value, instagram, youtube, accentColor })
            }
            placeholder="Ex.: Ana Silva"
            className="w-full bg-zinc-800 border border-zinc-700 focus:border-indigo-500 focus:outline-none text-white placeholder-zinc-500 rounded-xl px-4 py-2.5 transition-colors"
          />
        </div>

        {/* Instagram */}
        <div className="space-y-1">
          <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider">
            Instagram
          </label>
          <div className="flex items-center bg-zinc-800 border border-zinc-700 focus-within:border-indigo-500 rounded-xl px-4 transition-colors">
            <span className="text-zinc-500 mr-1">@</span>
            <input
              type="text"
              value={instagram}
              onChange={(e) =>
                setPersonalization({
                  name,
                  instagram: e.target.value.replace(/^@/, ""),
                  youtube,
                  accentColor,
                })
              }
              placeholder="seuinstagram"
              className="flex-1 bg-transparent py-2.5 text-white placeholder-zinc-500 focus:outline-none"
            />
          </div>
        </div>

        {/* YouTube */}
        <div className="space-y-1">
          <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider">
            YouTube / @handle
          </label>
          <input
            type="text"
            value={youtube}
            onChange={(e) =>
              setPersonalization({ name, instagram, youtube: e.target.value, accentColor })
            }
            placeholder="@seucanal"
            className="w-full bg-zinc-800 border border-zinc-700 focus:border-indigo-500 focus:outline-none text-white placeholder-zinc-500 rounded-xl px-4 py-2.5 transition-colors"
          />
        </div>

        {/* Cor de destaque */}
        <div className="space-y-2">
          <label className="text-xs font-medium text-zinc-400 uppercase tracking-wider">
            Cor de destaque
          </label>
          <div className="flex gap-2 flex-wrap">
            {ACCENT_PRESETS.map((p) => (
              <button
                key={p.value}
                title={p.label}
                onClick={() =>
                  setPersonalization({ name, instagram, youtube, accentColor: p.value })
                }
                className="w-8 h-8 rounded-lg transition-all"
                style={{
                  background: p.value,
                  outline:
                    accentColor === p.value
                      ? `2px solid ${p.value}`
                      : "2px solid transparent",
                  outlineOffset: "3px",
                }}
              />
            ))}
            {/* Cor customizada */}
            <label
              title="Cor personalizada"
              className="w-8 h-8 rounded-lg border border-zinc-600 flex items-center justify-center cursor-pointer hover:border-zinc-400 transition-colors"
            >
              <input
                type="color"
                value={accentColor}
                onChange={(e) =>
                  setPersonalization({ name, instagram, youtube, accentColor: e.target.value })
                }
                className="w-0 h-0 opacity-0 absolute"
              />
              <span className="text-zinc-400 text-xs">+</span>
            </label>
          </div>

          {/* Preview da faixa */}
          <div
            className="rounded-lg px-4 py-2 text-white text-sm font-bold text-center mt-1 transition-colors"
            style={{ background: accentColor + "33", border: `1px solid ${accentColor}66` }}
          >
            <span style={{ color: accentColor }}>
              {name || "Seu Nome"}{instagram ? ` | @${instagram}` : ""}
            </span>
          </div>
        </div>
      </div>

      {error && (
        <div className="bg-red-950/60 border border-red-800/60 rounded-xl px-4 py-3 text-red-300 text-sm text-center max-w-sm mx-auto">
          {error}
        </div>
      )}

      <div className="flex gap-3 w-full max-w-sm mx-auto mt-auto pt-2">
        <button
          onClick={() => setStep("license")}
          className="flex-1 bg-zinc-700 hover:bg-zinc-600 text-white font-semibold py-3 rounded-xl transition-colors text-sm"
        >
          ← Voltar
        </button>
        <button
          onClick={() => setStep("installing")}
          disabled={!canProceed()}
          className="flex-[2] bg-indigo-600 hover:bg-indigo-500 disabled:opacity-40 disabled:cursor-not-allowed text-white font-semibold py-3 rounded-xl transition-colors"
        >
          Instalar →
        </button>
      </div>
    </div>
  );
}
