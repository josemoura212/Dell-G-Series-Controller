import { useState, useEffect } from "react";

export function useAutostart() {
  const [autostartEnabled, setAutostartEnabled] = useState(false);

  useEffect(() => {
    checkAutostartStatus();
  }, []);

  const checkAutostartStatus = async () => {
    try {
      setAutostartEnabled(false);
    } catch (error) {
      console.error("Erro ao verificar autostart:", error);
    }
  };

  const toggleAutostart = async () => {
    try {
      setAutostartEnabled(!autostartEnabled);
    } catch (error) {
      console.error("Erro ao configurar autostart:", error);
    }
  };

  return {
    autostartEnabled,
    toggleAutostart,
  };
}
