import { DeviceInfo } from "../types";

interface HeaderProps {
  deviceInfo: DeviceInfo | null;
  autostartEnabled: boolean;
  onToggleAutostart: () => void;
}

export function Header({
  deviceInfo,
  autostartEnabled,
  onToggleAutostart,
}: HeaderProps) {
  return (
    <header>
      <h1>🎮 Dell G Series Controller</h1>
      <p className="device-model">{deviceInfo?.model || "Detectando..."}</p>
      <div className="header-controls">
        <button
          className={"autostart-btn " + (autostartEnabled ? "active" : "")}
          onClick={onToggleAutostart}
        >
          {autostartEnabled ? "🚀 Autostart: ON" : "⭕ Autostart: OFF"}
        </button>
      </div>
    </header>
  );
}
