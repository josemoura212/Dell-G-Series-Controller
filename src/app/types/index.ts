export interface DeviceInfo {
  model: string;
  keyboard_supported: boolean;
  power_supported: boolean;
  power_modes: string[];
  fan_control_limited?: boolean;
  turbo_enabled?: boolean;
}

export interface SensorData {
  fan1_rpm: number;
  fan2_rpm: number;
  cpu_temp: number;
  gpu_temp: number;
}

export interface PresetColor {
  name: string;
  rgb: [number, number, number];
  color: string;
}

export interface FanPreset {
  name: string;
  icon: string;
  cpu: number;
  gpu: number;
}

export type LedMode = "static" | "morph" | "breathing" | "zone" | "off";
