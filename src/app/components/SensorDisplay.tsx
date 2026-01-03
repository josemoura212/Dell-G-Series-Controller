import { SensorData } from "../types";

interface SensorDisplayProps {
  sensors: SensorData | null;
}

export function SensorDisplay({ sensors }: SensorDisplayProps) {
  if (!sensors) return null;

  return (
    <div className="section">
      <h3>Sensores</h3>
      <div className="sensor-display">
        <div className="sensor-item">
          <span className="sensor-label">Ventilador CPU</span>
          <span className="sensor-value">{sensors.fan1_rpm} RPM</span>
        </div>
        <div className="sensor-item">
          <span className="sensor-label">Ventilador GPU</span>
          <span className="sensor-value">{sensors.fan2_rpm} RPM</span>
        </div>
        <div className="sensor-item">
          <span className="sensor-label">CPU Temp</span>
          <span className="sensor-value">{sensors.cpu_temp}°C</span>
        </div>
        <div className="sensor-item">
          <span className="sensor-label">GPU Temp</span>
          <span className="sensor-value">{sensors.gpu_temp}°C</span>
        </div>
      </div>
    </div>
  );
}
