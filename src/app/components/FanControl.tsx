interface FanControlProps {
  fan1: number;
  fan2: number;
  onFan1Change: (value: number) => void;
  onFan2Change: (value: number) => void;
  onApply: () => void;
  fanControlLimited?: boolean;
}

export function FanControl({
  fan1,
  fan2,
  onFan1Change,
  onFan2Change,
  onApply,
  fanControlLimited = false,
}: FanControlProps) {
  return (
    <div className="section">
      <h3>Controle Manual</h3>

      {fanControlLimited && (
        <div className="warning-banner">
          <div className="warning-icon">⚠️</div>
          <div className="warning-content">
            <strong>Limitação do Hardware</strong>
            <p>
              O controle manual de velocidade das ventoinhas não está disponível
              neste modelo. O BIOS/EC firmware substitui configurações manuais
              por proteção de hardware.
            </p>
            <p>
              <strong>Alternativas:</strong>
            </p>
            <ul>
              <li>
                Use os modos de energia (Equilibrado, Performance, Silencioso)
                para controle indireto
              </li>
              <li>Verifique atualizações do BIOS no site da Dell</li>
              <li>
                Considere usar software proprietário da Dell se disponível
              </li>
            </ul>
          </div>
        </div>
      )}

      <div className="slider-group">
        <label>🌀 Ventilador CPU: {fan1}%</label>
        <input
          type="range"
          min="0"
          max="100"
          value={fan1}
          onChange={(e) => onFan1Change(Number(e.target.value))}
          disabled={fanControlLimited}
        />
      </div>
      <div className="slider-group">
        <label>💨 Ventilador GPU: {fan2}%</label>
        <input
          type="range"
          min="0"
          max="100"
          value={fan2}
          onChange={(e) => onFan2Change(Number(e.target.value))}
          disabled={fanControlLimited}
        />
      </div>
      <button
        className="apply-btn"
        onClick={onApply}
        disabled={fanControlLimited}
      >
        {fanControlLimited ? "Controle Indisponível" : "Aplicar Velocidades"}
      </button>
    </div>
  );
}
