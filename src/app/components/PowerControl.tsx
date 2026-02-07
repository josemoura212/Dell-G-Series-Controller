import { useState, useEffect, useRef } from "react";
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
  const isTurboRef = useRef(isTurbo); // Track state in ref for event listener
  const { settings, updateSetting } = usePersistedSettings();

  useEffect(() => {
    isTurboRef.current = isTurbo;
  }, [isTurbo]);

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

  // Sync turbo state from persisted settings on mount
  useEffect(() => {
    if (!deviceInfo?.power_supported) return;
    const persistedTurbo =
      settings?.isTurbo ?? deviceInfo.turbo_enabled ?? false;
    setIsTurbo(persistedTurbo);
  }, [deviceInfo?.power_supported, deviceInfo?.turbo_enabled, settings?.isTurbo]);

  // Sensor reading interval
  useEffect(() => {
    if (!deviceInfo?.power_supported) return;
    readSensors();
    const interval = setInterval(readSensors, 3000);
    return () => clearInterval(interval);
  }, [deviceInfo?.power_supported]);

  // Listen for turbo toggle events from backend (special key / F9)
  useEffect(() => {
    const unlisten = listen("turbo-toggled", async () => {
      const newStatus = !isTurboRef.current;
      setIsTurbo(newStatus);
      updateSetting("isTurbo", newStatus);

      let permissionGranted = await isPermissionGranted();
      if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === "granted";
      }

      if (permissionGranted) {
        sendNotification({
          title: "Modo Turbo",
          body: newStatus ? "ATIVADO" : "DESATIVADO",
        });
      }
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, [updateSetting]);

  const setPowerMode = async (mode: string) => {
    try {
      const result: string = await invoke("set_power_mode", { mode });
      updateSetting("currentMode", mode);
      // Limpar preset selecionado quando mudar de modo
      if (mode !== "Manual") {
        updateSetting("selectedPreset", null);
      }
      const wasInTurbo = isTurbo;
      setIsTurbo(false); // Manually setting mode disables turbo
      updateSetting("isTurbo", false);

      // Provide better feedback
      if (wasInTurbo) {
        showStatus(`${result} (Turbo desativado)`);
      } else {
        showStatus(result);
      }
    } catch (error) {
      const errorMsg = String(error);
      showStatus("Erro ao definir modo de energia: " + errorMsg, true);
      console.error("Power mode error:", error);
    }
  };

  const toggleTurboPromise = async () => {
    try {
      const result: string = await invoke("toggle_turbo");
      const newTurboState = !isTurbo;
      setIsTurbo(newTurboState);
      updateSetting("isTurbo", newTurboState);

      let permissionGranted = await isPermissionGranted();
      if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === "granted";
      }

      if (permissionGranted) {
        sendNotification({
          title: "🎮 Modo Turbo",
          body: newTurboState
            ? "🚀 ATIVADO - Ventiladores em 100%"
            : "🐜 Silencioso - DESATIVADO",
        });
      }
      showStatus(result);
    } catch (error) {
      const errorMsg = String(error);
      showStatus("Erro ao alternar modo turbo: " + errorMsg, true);
      console.error("Turbo toggle error:", error);

      let permissionGranted = await isPermissionGranted();
      if (permissionGranted) {
        sendNotification({
          title: "⚠️ Erro no Modo Turbo",
          body: errorMsg,
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

      const result: string = await invoke("set_fan_boost", {
        params: {
          cpu_rpm: Math.round(cpuFan),
          gpu_rpm: Math.round(gpuFan),
        },
      });
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
          title="Alternar Modo Turbo (Tecla F9 ou Fn+F9)"
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
