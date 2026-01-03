import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";
import { DeviceInfo, SensorData } from "../types";
// ... imports

// ... inside component

// ... imports

// ... inside component
import { FanControl } from "./FanControl";
import { SensorDisplay } from "./SensorDisplay";
import { usePersistedSettings } from "../hooks/usePersistedSettings";

interface PowerControlProps {
  deviceInfo: DeviceInfo | null;
  showStatus: (message: string, isError?: boolean) => void;
  onCheckSetup: () => void;
}

export function PowerControl({
  deviceInfo,
  showStatus,
  onCheckSetup,
}: PowerControlProps) {
  const [sensorData, setSensorData] = useState<SensorData | null>(null);
  const [isTurbo, setIsTurbo] = useState(false);
  const { settings, updateSetting } = usePersistedSettings();

  // Use persisted settings or defaults
  const cpuFan = settings?.cpuFan ?? 50;
  const gpuFan = settings?.gpuFan ?? 50;
  const currentMode = settings?.currentMode ?? "USTT_Balanced";

  const readSensors = async () => {
    if (!deviceInfo?.power_supported) return;
    try {
      const data: SensorData = await invoke("get_sensors");
      setSensorData(data);
    } catch (error) {
      showStatus("Erro ao ler sensores: " + String(error), true);
    }
  };

  useEffect(() => {
    if (!deviceInfo?.power_supported) return;

    // Read sensors immediately when component mounts
    readSensors();

    // Sync turbo state
    if (deviceInfo.turbo_enabled !== undefined) {
      setIsTurbo(deviceInfo.turbo_enabled);
    }

    // Set up automatic sensor reading every 3 seconds
    const interval = setInterval(() => {
      readSensors();
    }, 3000);

    // Listen for turbo toggle events from backend (Fn+F9)
    const unlisten = listen("turbo-toggled", () => {
      setIsTurbo((prev) => !prev);
      // Maybe refresh other states if needed
    });

    // Cleanup interval on unmount
    return () => {
      clearInterval(interval);
      unlisten.then((f) => f());
    };
  }, [deviceInfo]);

  const setPowerMode = async (mode: string) => {
    try {
      const result: string = await invoke("set_power_mode", { mode });
      updateSetting("currentMode", mode);
      // Limpar preset selecionado quando mudar de modo
      if (mode !== "Manual") {
        updateSetting("selectedPreset", null);
      }
      setIsTurbo(false); // Manually setting mode disables turbo
      showStatus(result);
    } catch (error) {
      showStatus("Erro: " + String(error), true);
    }
  };

  const toggleTurboPromise = async () => {
    try {
      const result: string = await invoke("toggle_turbo");
      setIsTurbo((prev) => !prev);

      let permissionGranted = await isPermissionGranted();
      if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === "granted";
      }

      if (permissionGranted) {
        sendNotification({
          title: "Modo Turbo",
          body: result,
        });
      }
      showStatus(result);
    } catch (error) {
      showStatus("Erro: " + String(error), true);
      let permissionGranted = await isPermissionGranted();
      if (permissionGranted) {
        sendNotification({
          title: "Erro no Modo Turbo",
          body: String(error),
        });
      }
    }
  };

  const handleCpuFanChange = (value: number) => {
    updateSetting("cpuFan", value);
  };

  const handleGpuFanChange = (value: number) => {
    updateSetting("gpuFan", value);
  };

  const applyFanSpeeds = async () => {
    try {
      // First ensure we're in Manual mode
      if (currentMode !== "Manual") {
        await setPowerMode("Manual");
        // Small delay to let mode change take effect
        await new Promise((resolve) => setTimeout(resolve, 300));
      }

      const params = {
        cpu_rpm: Math.round(cpuFan),
        gpu_rpm: Math.round(gpuFan),
      };

      // Try the imported invoke first
      const result: string = await invoke("set_fan_boost", params);
      setIsTurbo(false);
      showStatus(result);
    } catch (error) {
      showStatus("Erro: " + String(error), true);
    }
  };

  if (!deviceInfo?.power_supported) {
    return (
      <section className="card glass-panel">
        <h2>⚡ Controle de Energia</h2>
        <div className="warning-box">
          <p>⚠️ ACPI não disponível</p>
          <p className="info-text">
            Execute o script de configuração do sistema
          </p>
          <button className="setup-btn" onClick={onCheckSetup}>
            🔄 Verificar Novamente
          </button>
        </div>
      </section>
    );
  }

  return (
    <section className="card glass-panel">
      <div className="card-header">
        <h2>⚡ Controle de Energia</h2>
        <button
          className={`turbo-toggle-btn ${isTurbo ? "active" : ""}`}
          onClick={toggleTurboPromise}
          title="Alternar Modo Turbo (Fn+F9)"
        >
          {isTurbo ? "🚀 TURBO ON" : "🚀 TURBO OFF"}
        </button>
      </div>

      <div className="section">
        <h3>Modo de Energia</h3>
        <div className="button-group power-modes">
          <button
            className={`mode-btn ${
              !isTurbo && currentMode === "USTT_Quiet" ? "active" : ""
            }`}
            onClick={() => setPowerMode("USTT_Quiet")}
          >
            <span className="icon">🌙</span> Silencioso
          </button>
          <button
            className={`mode-btn ${
              !isTurbo && currentMode === "USTT_Balanced" ? "active" : ""
            }`}
            onClick={() => setPowerMode("USTT_Balanced")}
          >
            <span className="icon">⚖️</span> Balanceado
          </button>
          <button
            className={`mode-btn ${
              !isTurbo && currentMode === "USTT_Performance" ? "active" : ""
            }`}
            onClick={() => setPowerMode("USTT_Performance")}
          >
            <span className="icon">🔥</span> Performance
          </button>
        </div>
      </div>

      <SensorDisplay sensors={sensorData} />

      {/* Hidden manual control for safety, can be re-enabled if needed */}
      {currentMode === "Manual" && (
        <FanControl
          fan1={cpuFan}
          fan2={gpuFan}
          onFan1Change={handleCpuFanChange}
          onFan2Change={handleGpuFanChange}
          onApply={applyFanSpeeds}
          fanControlLimited={deviceInfo?.fan_control_limited}
        />
      )}
    </section>
  );
}
