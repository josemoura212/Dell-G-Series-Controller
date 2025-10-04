import { invoke } from "@tauri-apps/api/core";
import { PRESET_COLORS } from "../constants/colors";
import { DeviceInfo, LedMode } from "../types";
import { ColorPicker } from "./ColorPicker";
import { LedModeSelector } from "./LedModeSelector";
import { usePersistedSettings } from "../hooks/usePersistedSettings";

interface KeyboardControlProps {
  deviceInfo: DeviceInfo | null;
  showStatus: (message: string, isError?: boolean) => void;
  onCheckSetup: () => void;
}

export function KeyboardControl({
  deviceInfo,
  showStatus,
  onCheckSetup,
}: KeyboardControlProps) {
  const { settings, updateSetting } = usePersistedSettings();

  // Use persisted settings or defaults
  const red = settings?.red ?? 122;
  const green = settings?.green ?? 122;
  const blue = settings?.blue ?? 122;
  const duration = settings?.duration ?? 128;
  const currentMode = (settings?.currentLedMode as LedMode) ?? "static";

  const applyPresetColor = async (rgb: [number, number, number]) => {
    updateSetting("red", rgb[0]);
    updateSetting("green", rgb[1]);
    updateSetting("blue", rgb[2]);
    try {
      const result: string = await invoke("set_static_color", {
        red: rgb[0],
        green: rgb[1],
        blue: rgb[2],
      });
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

  const handleRedChange = (value: number) => updateSetting("red", value);
  const handleGreenChange = (value: number) => updateSetting("green", value);
  const handleBlueChange = (value: number) => updateSetting("blue", value);
  const handleModeChange = (mode: LedMode) =>
    updateSetting("currentLedMode", mode);
  const handleDurationChange = (value: number) =>
    updateSetting("duration", value);

  if (!deviceInfo?.keyboard_supported) {
    return (
      <section className="card">
        <h2>🎹 Controle do Teclado RGB</h2>
        <div className="warning-box">
          <p>⚠️ Teclado RGB não disponível</p>
          <p className="info-text">
            Verifique a conexão USB e execute a configuração do sistema
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
      <h2>🎹 Controle do Teclado RGB</h2>

      <div className="section">
        <h3>Cores Rápidas</h3>
        <div className="color-grid">
          {PRESET_COLORS.map((preset) => (
            <button
              key={preset.name}
              className="color-btn"
              style={{ background: preset.color }}
              onClick={() => applyPresetColor(preset.rgb)}
            >
              {preset.name}
            </button>
          ))}
        </div>
      </div>

      <ColorPicker
        red={red}
        green={green}
        blue={blue}
        onRedChange={handleRedChange}
        onGreenChange={handleGreenChange}
        onBlueChange={handleBlueChange}
      />

      <LedModeSelector
        currentMode={currentMode}
        duration={duration}
        onModeChange={handleModeChange}
        onDurationChange={handleDurationChange}
        onApply={applyColor}
      />
    </section>
  );
}
