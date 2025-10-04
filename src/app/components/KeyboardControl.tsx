import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { PRESET_COLORS } from "../constants/colors";
import { DeviceInfo, LedMode } from "../types";
import { ColorPicker } from "./ColorPicker";
import { LedModeSelector } from "./LedModeSelector";

interface KeyboardControlProps {
  deviceInfo: DeviceInfo | null;
  showStatus: (message: string, isError?: boolean) => void;
  onCheckSetup: () => void;
}

export function KeyboardControl({ deviceInfo, showStatus, onCheckSetup }: KeyboardControlProps) {
  const [red, setRed] = useState(122);
  const [green, setGreen] = useState(122);
  const [blue, setBlue] = useState(122);
  const [duration, setDuration] = useState(128);
  const [currentMode, setCurrentMode] = useState<LedMode>("static");

  const applyPresetColor = async (rgb: [number, number, number]) => {
    setRed(rgb[0]);
    setGreen(rgb[1]);
    setBlue(rgb[2]);
    try {
      const result: string = await invoke("set_static_color", { red: rgb[0], green: rgb[1], blue: rgb[2] });
      showStatus(result);
    } catch (error) {
      showStatus("Erro: " + String(error), true);
    }
  };

  const applyColor = async () => {
    try {
      let result: string;
      if (currentMode === "static") {
        result = await invoke("set_static_color", { red, green, blue });
      } else if (currentMode === "morph") {
        result = await invoke("set_morph", { red, green, blue, duration });
      } else {
        result = await invoke("turn_off_leds");
      }
      showStatus(result);
    } catch (error) {
      showStatus("Erro: " + String(error), true);
    }
  };

  if (!deviceInfo?.keyboard_supported) {
    return (
      <section className="card">
        <h2>🎹 Controle do Teclado RGB</h2>
        <div className="warning-box">
          <p>⚠️ Teclado RGB não disponível</p>
          <p className="info-text">Verifique a conexão USB e execute a configuração do sistema</p>
          <button className="setup-btn" onClick={onCheckSetup}>🔄 Verificar Novamente</button>
        </div>
      </section>
    );
  }

  return (
    <section className="card">
      <h2>🎹 Controle do Teclado RGB</h2>
      
      <div className="section">
        <h3>Cores Rápidas</h3>
        <div className="color-grid">
          {PRESET_COLORS.map((preset) => (
            <button key={preset.name} className="color-btn" style={{ background: preset.color }} onClick={() => applyPresetColor(preset.rgb)}>
              {preset.name}
            </button>
          ))}
        </div>
      </div>

      <ColorPicker red={red} green={green} blue={blue} onRedChange={setRed} onGreenChange={setGreen} onBlueChange={setBlue} />

      <LedModeSelector currentMode={currentMode} duration={duration} onModeChange={setCurrentMode} onDurationChange={setDuration} onApply={applyColor} />
    </section>
  );
}
