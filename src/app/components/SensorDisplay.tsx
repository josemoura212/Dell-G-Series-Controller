import { SensorData } from "../types";

interface SensorDisplayProps {
  sensors: SensorData | null;
}

export function SensorDisplay({ sensors }: SensorDisplayProps) {
  return (
    <div className="section">
      <h3>Sensores</h3>
      {sensors && (
        <div className="sensor-display">
          <div>🌀 Ventilador CPU: {sensors.fan1_rpm} RPM</div>
          <div>💨 Ventilador GPU: {sensors.fan2_rpm} RPM</div>
          <div>🌡️ CPU: {sensors.cpu_temp}°C</div>
          <div>🌡️ GPU: {sensors.gpu_temp}°C</div>
        </div>
      )}
    </div>
  );
}
