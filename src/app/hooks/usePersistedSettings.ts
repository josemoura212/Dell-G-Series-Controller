import { useState, useEffect } from "react";

export interface PersistedSettings {
  currentMode: string;
  selectedPreset: string | null;
  cpuFan: number;
  gpuFan: number;
  red: number;
  green: number;
  blue: number;
  duration: number;
  currentLedMode: string;
  isTurbo?: boolean;
  zone0?: [number, number, number];
  zone1?: [number, number, number];
  zone2?: [number, number, number];
  zone3?: [number, number, number];
}

const SETTINGS_KEY = "dell-controller-settings";

export function usePersistedSettings() {
  const [settings, setSettings] = useState<PersistedSettings | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = () => {
    try {
      const saved = localStorage.getItem(SETTINGS_KEY);
      if (saved) {
        const parsedSettings = JSON.parse(saved);
        setSettings(parsedSettings);
      } else {
        // Default settings
        const defaultSettings: PersistedSettings = {
          currentMode: "USTT_Balanced",
          selectedPreset: null,
          cpuFan: 50,
          gpuFan: 50,
          red: 255,
          green: 0,
          blue: 0,
          duration: 1000,
          currentLedMode: "static",
          isTurbo: false,
        };
        setSettings(defaultSettings);
        saveSettings(defaultSettings);
      }
    } catch (error) {
      console.error("Failed to load settings:", error);
      // Use defaults on error
      const defaultSettings: PersistedSettings = {
        currentMode: "balanced",
        selectedPreset: null,
        cpuFan: 50,
        gpuFan: 50,
        red: 255,
        green: 0,
        blue: 0,
        duration: 1000,
        currentLedMode: "static",
      };
      setSettings(defaultSettings);
    } finally {
      setIsLoading(false);
    }
  };

  const saveSettings = (newSettings: Partial<PersistedSettings>) => {
    try {
      const currentSettings = settings || {
        currentMode: "USTT_Balanced",
        selectedPreset: null,
        cpuFan: 50,
        gpuFan: 50,
        red: 255,
        green: 0,
        blue: 0,
        duration: 1000,
        currentLedMode: "static",
        isTurbo: false,
      };
      const updatedSettings = { ...currentSettings, ...newSettings };
      localStorage.setItem(SETTINGS_KEY, JSON.stringify(updatedSettings));
      setSettings(updatedSettings);
      console.log("Settings saved:", updatedSettings);
    } catch (error) {
      console.error("Failed to save settings:", error);
    }
  };

  const updateSetting = <K extends keyof PersistedSettings>(
    key: K,
    value: PersistedSettings[K]
  ) => {
    saveSettings({ [key]: value });
  };

  return {
    settings,
    isLoading,
    saveSettings,
    updateSetting,
    loadSettings,
  };
}
