import { useState, useEffect } from "react";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";

export function useAutostart() {
  const [autostartEnabled, setAutostartEnabled] = useState(false);

  useEffect(() => {
    checkAutostartStatus();
  }, []);

  const checkAutostartStatus = async () => {
    try {
      const enabled = await isEnabled();
      setAutostartEnabled(enabled);
    } catch (error) {
      console.error("Erro ao verificar autostart:", error);
    }
  };

  const toggleAutostart = async () => {
    try {
      if (autostartEnabled) {
        await disable();
        setAutostartEnabled(false);
      } else {
        await enable();
        setAutostartEnabled(true);
      }
    } catch (error) {
      console.error("Erro ao configurar autostart:", error);
    }
  };

  return {
    autostartEnabled,
    toggleAutostart,
  };
}
