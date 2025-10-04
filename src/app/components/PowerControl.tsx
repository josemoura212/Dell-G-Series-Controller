import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { sendNotification } from "@tauri-apps/plugin-notification";
import { FAN_PRESETS } from "../constants/colors";
import { DeviceInfo, SensorData, FanPreset } from "../types";
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
  const { settings, updateSetting } = usePersistedSettings();

  // Use persisted settings or defaults
  const cpuFan = settings?.cpuFan ?? 50;
  const gpuFan = settings?.gpuFan ?? 50;
  const currentMode = settings?.currentMode ?? "USTT_Balanced";
  const selectedPreset = settings?.selectedPreset
    ? FAN_PRESETS.find((p) => p.name === settings.selectedPreset)
    : null;

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

    // Set up automatic sensor reading every 3 seconds
    const interval = setInterval(() => {
      readSensors();
    }, 3000);

    // Cleanup interval on unmount
    return () => clearInterval(interval);
  }, [deviceInfo]);

  const setPowerMode = async (mode: string) => {
    try {
      const result: string = await invoke("set_power_mode", { mode });
      updateSetting("currentMode", mode);
      // Limpar preset selecionado quando mudar de modo
      if (mode !== "Manual") {
        updateSetting("selectedPreset", null);
      }
      showStatus(result);
    } catch (error) {
      showStatus("Erro: " + String(error), true);
    }
  };

  const activateTurboMode = async () => {
    try {
      const result: string = await invoke("set_turbo_mode");
      showStatus(result);

      // Send notification
      await sendNotification({
        title: "🚀 Modo Turbo Ativado",
        body: "Ventiladores configurados para velocidade máxima",
      });
    } catch (error) {
      showStatus("Erro: " + String(error), true);
    }
  };

  const deactivateTurboMode = async () => {
    try {
      // Return to balanced mode
      const result: string = await invoke("set_power_mode", {
        mode: "USTT_Balanced",
      });
      updateSetting("currentMode", "USTT_Balanced");
      updateSetting("selectedPreset", null);
      showStatus(result);

      // Send notification
      await sendNotification({
        title: "⚖️ Modo Balanceado Restaurado",
        body: "Ventiladores retornaram ao modo automático balanceado",
      });
    } catch (error) {
      showStatus("Erro: " + String(error), true);
    }
  };

  const applyFanPreset = async (preset: FanPreset) => {
    updateSetting("cpuFan", preset.cpu);
    updateSetting("gpuFan", preset.gpu);
    updateSetting("selectedPreset", preset.name);
    try {
      const result: string = await invoke("set_fan_boost", {
        cpu_rpm: preset.cpu,
        gpu_rpm: preset.gpu,
      });
      showStatus(result);
    } catch (error) {
      showStatus("Erro: " + String(error), true);
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
      showStatus(result);
    } catch (error) {
      showStatus("Erro: " + String(error), true);
    }
  };

  if (!deviceInfo?.power_supported) {
    return (
      <section className="card">
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
    <section className="card">
      <h2>⚡ Controle de Energia</h2>

      <div className="section">
        <h3>Modo de Energia</h3>
        <div className="button-row">
          <button
            className={`mode-btn ${
              currentMode === "USTT_Quiet" ? "active" : ""
            }`}
            onClick={() => setPowerMode("USTT_Quiet")}
          >
            🌙 Silencioso
          </button>
          <button
            className={`mode-btn ${
              currentMode === "USTT_Balanced" ? "active" : ""
            }`}
            onClick={() => setPowerMode("USTT_Balanced")}
          >
            ⚖️ Balanceado
          </button>
          <button
            className={`mode-btn ${
              currentMode === "USTT_Performance" ? "active" : ""
            }`}
            onClick={() => setPowerMode("USTT_Performance")}
          >
            🚀 Performance
          </button>
          <button
            className={`mode-btn ${currentMode === "Manual" ? "active" : ""}`}
            onClick={() => setPowerMode("Manual")}
          >
            🎛️ Manual
          </button>
        </div>
      </div>

      <SensorDisplay sensors={sensorData} />

      <div className="section">
        <h3>Modo Turbo</h3>
        <div className="button-row">
          <button className="mode-btn turbo-btn" onClick={activateTurboMode}>
            🚀 TURBO (F12)
          </button>
          <button
            className="mode-btn balanced-btn"
            onClick={deactivateTurboMode}
          >
            ⚖️ Balanceado
          </button>
        </div>
        <p className="info-text">
          💡 Pressione F12 ou clique em TURBO para ativar ventiladores em
          velocidade máxima
        </p>
      </div>

      <div className="section">
        <h3>Presets de Ventilação</h3>
        <div className="button-row">
          {FAN_PRESETS.map((preset) => (
            <button
              key={preset.name}
              className={`preset-btn ${
                selectedPreset?.name === preset.name ? "active" : ""
              }`}
              onClick={() => applyFanPreset(preset)}
            >
              {preset.icon} {preset.name}
            </button>
          ))}
        </div>
      </div>

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
