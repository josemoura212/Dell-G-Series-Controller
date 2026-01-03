import { invoke } from "@tauri-apps/api/core";
import { PRESET_COLORS } from "../constants/colors";
import { DeviceInfo, LedMode } from "../types";
import { ColorPicker } from "./ColorPicker";
import { LedModeSelector } from "./LedModeSelector";
import {
  usePersistedSettings,
  PersistedSettings,
} from "../hooks/usePersistedSettings";

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
      } else if (currentMode === "breathing") {
        result = await invoke("set_pulse_effect", {
          red,
          green,
          blue,
          speed: duration,
        });
      } else if (currentMode === "zone") {
        // Should be handled by individual zone updates or a separate apply button
        // For now, let's create a bulk apply for zones
        const colors = [
          settings?.zone0 ?? [255, 0, 0],
          settings?.zone1 ?? [0, 255, 0],
          settings?.zone2 ?? [0, 0, 255],
          settings?.zone3 ?? [255, 255, 255],
        ];
        result = await invoke("set_zone_colors", { colors });
      } else {
        result = await invoke("turn_off_leds");
      }
      showStatus(result);
    } catch (error) {
      showStatus("Erro: " + String(error), true);
    }
  };

  const activeZoneColors = [
    settings?.zone0 ?? [255, 0, 0],
    settings?.zone1 ?? [0, 255, 0],
    settings?.zone2 ?? [0, 0, 255],
    settings?.zone3 ?? [255, 255, 255],
  ];

  const handleZoneColorChange = (
    zoneIndex: number,
    color: [number, number, number]
  ) => {
    const key = `zone${zoneIndex}` as keyof PersistedSettings;
    updateSetting(key, color);
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
    <section className="card glass-panel">
      <div className="card-header">
        <h2>🎹 Controle do Teclado RGB</h2>
      </div>

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

      {currentMode === "zone" ? (
        <div className="section">
          <h3>Configuração de Zonas</h3>
          <div
            style={{
              display: "grid",
              gridTemplateColumns: "1fr 1fr",
              gap: "12px",
            }}
          >
            {[0, 1, 2, 3].map((zoneId) => (
              <div
                key={zoneId}
                className="zone-picker"
                style={{
                  background: "rgba(0,0,0,0.2)",
                  padding: "10px",
                  borderRadius: "8px",
                }}
              >
                <label
                  style={{
                    display: "block",
                    marginBottom: "8px",
                    fontSize: "12px",
                  }}
                >
                  Zona {zoneId + 1}
                </label>
                <input
                  type="color"
                  style={{
                    width: "100%",
                    height: "40px",
                    border: "none",
                    borderRadius: "4px",
                    cursor: "pointer",
                    background: "transparent",
                  }}
                  value={`#${(
                    (1 << 24) +
                    ((activeZoneColors[zoneId]?.[0] ?? 0) << 16) +
                    ((activeZoneColors[zoneId]?.[1] ?? 0) << 8) +
                    (activeZoneColors[zoneId]?.[2] ?? 0)
                  )
                    .toString(16)
                    .slice(1)}`}
                  onChange={(e) => {
                    const hex = e.target.value;
                    const r = parseInt(hex.substr(1, 2), 16);
                    const g = parseInt(hex.substr(3, 2), 16);
                    const b = parseInt(hex.substr(5, 2), 16);
                    handleZoneColorChange(zoneId, [r, g, b]);
                  }}
                />
              </div>
            ))}
          </div>
          <button
            className="apply-btn"
            onClick={applyColor}
            style={{ marginTop: "16px" }}
          >
            Aplicar Zonas
          </button>
        </div>
      ) : (
        <ColorPicker
          red={red}
          green={green}
          blue={blue}
          onRedChange={handleRedChange}
          onGreenChange={handleGreenChange}
          onBlueChange={handleBlueChange}
        />
      )}

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
