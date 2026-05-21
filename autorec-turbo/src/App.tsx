import { useSetupStore } from "./store/setup";
import StepWelcome from "./pages/StepWelcome";
import StepLicense from "./pages/StepLicense";
import StepPersonalization from "./pages/StepPersonalization";
import StepInstalling from "./pages/StepInstalling";
import StepDone from "./pages/StepDone";

const STEP_LABELS = ["Boas-vindas", "Licença", "Personalização", "Instalação", "Concluído"];
const STEP_ORDER = ["welcome", "license", "personalization", "installing", "done"] as const;

function ProgressBar({ currentStep }: { currentStep: string }) {
  const idx = STEP_ORDER.indexOf(currentStep as (typeof STEP_ORDER)[number]);
  return (
    <div className="px-8 pt-6 pb-2 shrink-0">
      <div className="flex items-center gap-1">
        {STEP_ORDER.map((s, i) => (
          <div key={s} className="flex items-center gap-1 flex-1">
            <div
              className={`h-1 flex-1 rounded-full transition-all duration-500 ${
                i <= idx ? "bg-indigo-500" : "bg-zinc-700"
              }`}
            />
          </div>
        ))}
      </div>
      <p className="text-xs text-zinc-500 mt-1.5 text-center">
        {STEP_LABELS[idx] ?? ""}
      </p>
    </div>
  );
}

function App() {
  const step = useSetupStore((s) => s.step);

  return (
    <div className="h-screen bg-zinc-900 flex flex-col select-none">
      {step !== "welcome" && step !== "done" && (
        <ProgressBar currentStep={step} />
      )}
      <main className="flex-1 overflow-hidden">
        {step === "welcome" && <StepWelcome />}
        {step === "license" && <StepLicense />}
        {step === "personalization" && <StepPersonalization />}
        {step === "installing" && <StepInstalling />}
        {step === "done" && <StepDone />}
      </main>
    </div>
  );
}

export default App;
