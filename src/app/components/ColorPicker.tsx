interface ColorPickerProps {
  red: number;
  green: number;
  blue: number;
  onRedChange: (value: number) => void;
  onGreenChange: (value: number) => void;
  onBlueChange: (value: number) => void;
}

export function ColorPicker({
  red,
  green,
  blue,
  onRedChange,
  onGreenChange,
  onBlueChange,
}: ColorPickerProps) {
  const bgColor = "rgb(" + red + ", " + green + ", " + blue + ")";

  return (
    <div className="section">
      <h3>Cor Personalizada</h3>
      <div className="slider-group">
        <label>🔴 Vermelho: {red}</label>
        <input
          type="range"
          min="0"
          max="255"
          value={red}
          onChange={(e) => onRedChange(Number(e.target.value))}
        />
      </div>
      <div className="slider-group">
        <label>🟢 Verde: {green}</label>
        <input
          type="range"
          min="0"
          max="255"
          value={green}
          onChange={(e) => onGreenChange(Number(e.target.value))}
        />
      </div>
      <div className="slider-group">
        <label>🔵 Azul: {blue}</label>
        <input
          type="range"
          min="0"
          max="255"
          value={blue}
          onChange={(e) => onBlueChange(Number(e.target.value))}
        />
      </div>
      <div className="color-preview" style={{ background: bgColor }} />
    </div>
  );
}
