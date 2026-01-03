import { useDeviceSetup } from "./app/hooks/useDeviceSetup";
import { useAutostart } from "./app/hooks/useAutostart";
import { Header } from "./app/components/Header";
import { SetupBanner } from "./app/components/SetupBanner";
import { KeyboardControl } from "./app/components/KeyboardControl";
import { PowerControl } from "./app/components/PowerControl";
import { Footer } from "./app/components/Footer";
import "./App.css";

function App() {
  const {
    deviceInfo,
    setupNeeded,
    statusMessage,
    statusError,
    checkSystemSetup,
    runSetup,
  } = useDeviceSetup();
  const { autostartEnabled, toggleAutostart } = useAutostart();

  function showStatus(_message: string, _error = false) {
    // Status display function
  }

  return (
    <div style={{ minHeight: "100vh", padding: "20px" }}>
      <Header
        deviceInfo={deviceInfo}
        autostartEnabled={autostartEnabled}
        onToggleAutostart={toggleAutostart}
      />

      {setupNeeded && <SetupBanner onRunSetup={runSetup} />}

      <main
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fit, minmax(500px, 1fr))",
          gap: "20px",
          maxWidth: "1400px",
          margin: "0 auto 20px",
          alignItems: "start",
        }}
      >
        <KeyboardControl
          deviceInfo={deviceInfo}
          showStatus={showStatus}
          onCheckSetup={checkSystemSetup}
        />
        <PowerControl
          deviceInfo={deviceInfo}
          showStatus={showStatus}
          onCheckSetup={checkSystemSetup}
        />
      </main>

      <Footer statusMessage={statusMessage} statusError={statusError} />
    </div>
  );
}

export default App;
