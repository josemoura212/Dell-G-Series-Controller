import { LedMode } from "../types";

interface LedModeSelectorProps {
  currentMode: LedMode;
  duration: number;
  onModeChange: (mode: LedMode) => void;
  onDurationChange: (duration: number) => void;
  onApply: () => void;
}

export function LedModeSelector({
  currentMode,
  duration,
  onModeChange,
  onDurationChange,
  onApply,
}: LedModeSelectorProps) {
  return (
    <div className="section">
      <h3>Modos de LED</h3>

      <div style={{ marginBottom: "16px" }}>
        <select
          value={currentMode}
          onChange={(e) => onModeChange(e.target.value as LedMode)}
          className="dark-select"
        >
          <option value="static">Cor Estática</option>
          <option value="breathing">Respiração (Pulse)</option>
          <option value="morph">Transição (Morph)</option>
          <option value="spectrum">Spectrum Cycle</option>
          <option value="rainbow">Rainbow Wave</option>
          <option value="zone">Zonas (4 Regiões)</option>
          <option value="off">Desligado</option>
        </select>
      </div>

      {(currentMode === "morph" ||
        currentMode === "breathing" ||
        currentMode === "spectrum" ||
        currentMode === "rainbow") && (
        <div className="slider-group">
          <label>
            ⏱️ Velocidade: {duration} ({((duration / 1000) * 100).toFixed(0)}%)
          </label>
          <input
            type="range"
            min="255"
            max="1000"
            value={duration}
            onChange={(e) => onDurationChange(Number(e.target.value))}
          />
          <p className="info-text">Mais rápido (255) ← → Mais lento (1000)</p>
        </div>
      )}

      <button className="apply-btn" onClick={onApply}>
        ✓ Aplicar Configuração
      </button>
    </div>
  );
}
