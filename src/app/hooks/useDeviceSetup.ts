import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DeviceInfo } from "../types";

export function useDeviceSetup() {
  const [deviceInfo, setDeviceInfo] = useState<DeviceInfo | null>(null);
  const [setupNeeded, setSetupNeeded] = useState(false);
  const [statusMessage, setStatusMessage] = useState("Inicializando...");
  const [statusError, setStatusError] = useState(false);

  useEffect(() => {
    checkSystemSetup();
  }, []);

  function showStatus(message: string, isError: boolean = false) {
    setStatusMessage(message);
    setStatusError(isError);
  }

  async function checkSystemSetup() {
    // Sempre tenta inicializar o dispositivo primeiro
    try {
      const info: DeviceInfo = await invoke("init_device");
      setDeviceInfo(info);

      // Verificar imediatamente se ACPI está disponível usando o info retornado
      if (info.power_supported) {
        try {
          const devices: string[] = await invoke("check_usb_devices");
          showStatus(
            `✓ Sistema configurado. Dispositivos: ${devices.join(", ")}`
          );
          setSetupNeeded(false);
          return;
        } catch (usbError) {
          showStatus(
            `⚠️ ${usbError}. Conecte o dispositivo Dell ou verifique as permissões.`,
            true
          );
          setSetupNeeded(true);
          return;
        }
      }
    } catch (initError) {
      showStatus(`❌ Erro na inicialização: ${initError}`, true);
      setSetupNeeded(true);
      return;
    }

    // Se ACPI não está disponível, verificar permissões
    try {
      const permResult: string = await invoke("check_permissions");

      if (permResult === "configured") {
        try {
          const devices: string[] = await invoke("check_usb_devices");
          showStatus(
            `✓ Sistema configurado. Dispositivos: ${devices.join(", ")}`
          );
        } catch (usbError) {
          showStatus(
            `⚠️ ${usbError}. Conecte o dispositivo Dell ou verifique as permissões.`,
            true
          );
          setSetupNeeded(true);
        }
      } else {
        const missing = permResult.replace("missing:", "");
        setSetupNeeded(true);
        showStatus(
          `⚠️ Permissões não configuradas (${missing}). Execute a configuração do sistema.`,
          true
        );
      }
    } catch (error) {
      showStatus(`❌ Erro na verificação: ${error}`, true);
      setSetupNeeded(true);
    }
  }

  async function runSetup() {
    try {
      showStatus("⚙️ Executando configuração...");
      const result: string = await invoke("run_setup_script");
      showStatus(result);

      const shouldRestart = confirm(
        "Configuração concluída! É necessário reiniciar o sistema. Reiniciar agora?"
      );

      if (shouldRestart) {
        showStatus("🔄 Reiniciando o sistema...");
      }
    } catch (error) {
      showStatus(`❌ Erro na configuração: ${error}`, true);
    }
  }

  return {
    deviceInfo,
    setupNeeded,
    statusMessage,
    statusError,
    showStatus,
    checkSystemSetup,
    runSetup,
  };
}
