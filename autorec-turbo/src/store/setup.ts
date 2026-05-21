import { create } from "zustand";

export type Step =
  | "welcome"
  | "license"
  | "personalization"
  | "installing"
  | "done";

export interface SetupState {
  step: Step;
  licenseKey: string;
  email: string;
  name: string;
  instagram: string;
  youtube: string;
  accentColor: string;
  logs: string[];
  error: string | null;

  setStep: (s: Step) => void;
  setLicenseKey: (k: string) => void;
  setEmail: (e: string) => void;
  setPersonalization: (p: {
    name: string;
    instagram: string;
    youtube: string;
    accentColor: string;
  }) => void;
  addLog: (msg: string) => void;
  setError: (e: string | null) => void;
  reset: () => void;
}

const initialState = {
  step: "welcome" as Step,
  licenseKey: "",
  email: "",
  name: "",
  instagram: "",
  youtube: "",
  accentColor: "#6366f1",
  logs: [],
  error: null,
};

export const useSetupStore = create<SetupState>((set) => ({
  ...initialState,
  setStep: (step) => set({ step, error: null }),
  setLicenseKey: (licenseKey) => set({ licenseKey }),
  setEmail: (email) => set({ email }),
  setPersonalization: (p) => set(p),
  addLog: (msg) => set((s) => ({ logs: [...s.logs, msg] })),
  setError: (error) => set({ error }),
  reset: () => set(initialState),
}));
