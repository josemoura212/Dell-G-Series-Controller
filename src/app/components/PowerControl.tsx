import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { FAN_PRESETS } from "../constants/colors";
import { DeviceInfo, SensorData, FanPreset } from "../types";
import { FanControl } from "./FanControl";
import { SensorDisplay } from "./SensorDisplay";

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
  const [cpuFan, setCpuFan] = useState(50);
  const [gpuFan, setGpuFan] = useState(50);
  const [currentMode, setCurrentMode] = useState<string>("USTT_Balanced");
  const [selectedPreset, setSelectedPreset] = useState<FanPreset | null>(null);

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
      setCurrentMode(mode);
      // Limpar preset selecionado quando mudar de modo
      if (mode !== "Manual") {
        setSelectedPreset(null);
      }
      showStatus(result);
    } catch (error) {
      showStatus("Erro: " + String(error), true);
    }
  };

  const applyFanPreset = async (preset: FanPreset) => {
    setCpuFan(preset.cpu);
    setGpuFan(preset.gpu);
    setSelectedPreset(preset);
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
          onFan1Change={setCpuFan}
          onFan2Change={setGpuFan}
          onApply={applyFanSpeeds}
          fanControlLimited={deviceInfo?.fan_control_limited}
        />
      )}
    </section>
  );
}
