import { PresetColor, FanPreset } from "../types";

export const PRESET_COLORS: PresetColor[] = [
  { name: "🔴 Vermelho", rgb: [255, 0, 0], color: "rgb(255,0,0)" },
  { name: "🟢 Verde", rgb: [0, 255, 0], color: "rgb(0,255,0)" },
  { name: "🔵 Azul", rgb: [0, 0, 255], color: "rgb(0,0,255)" },
  { name: "🟡 Amarelo", rgb: [255, 255, 0], color: "rgb(255,255,0)" },
  { name: "🔷 Ciano", rgb: [0, 255, 255], color: "rgb(0,255,255)" },
  { name: "🟣 Magenta", rgb: [255, 0, 255], color: "rgb(255,0,255)" },
  { name: "⚪ Branco", rgb: [255, 255, 255], color: "rgb(255,255,255)" },
  { name: "🟠 Laranja", rgb: [255, 128, 0], color: "rgb(255,128,0)" },
  { name: "🟪 Roxo", rgb: [128, 0, 255], color: "rgb(128,0,255)" },
  { name: "🩷 Rosa", rgb: [255, 128, 192], color: "rgb(255,128,192)" },
];

export const FAN_PRESETS: FanPreset[] = [
  { name: "Silencioso", icon: "🔇", cpu: 0, gpu: 0 },
  { name: "Normal", icon: "💨", cpu: 50, gpu: 50 },
  { name: "Turbo", icon: "🌪️", cpu: 85, gpu: 85 },
  { name: "Máximo", icon: "🔥", cpu: 100, gpu: 100 },
];
