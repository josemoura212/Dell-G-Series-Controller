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
      <div className="mode-buttons">
        <button
          className={"mode-btn " + (currentMode === "static" ? "active" : "")}
          onClick={() => onModeChange("static")}
        >
          Cor Estática
        </button>
        <button
          className={"mode-btn " + (currentMode === "morph" ? "active" : "")}
          onClick={() => onModeChange("morph")}
        >
          Transição
        </button>
        <button
          className={
            "mode-btn danger " + (currentMode === "off" ? "active" : "")
          }
          onClick={() => onModeChange("off")}
        >
          Desligar
        </button>
      </div>

      {currentMode === "morph" && (
        <div className="slider-group">
          <label>
            ⏱️ Duração da Transição: {duration} (
            {((duration / 255) * 100).toFixed(0)}%)
          </label>
          <input
            type="range"
            min="1"
            max="255"
            value={duration}
            onChange={(e) => onDurationChange(Number(e.target.value))}
          />
          <p className="info-text">Mais rápido (1) ← → Mais lento (255)</p>
        </div>
      )}

      <button className="apply-btn" onClick={onApply}>
        ✓ Aplicar Configuração
      </button>
    </div>
  );
}
