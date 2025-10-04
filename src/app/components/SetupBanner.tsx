interface SetupBannerProps {
  onRunSetup: () => void;
}

export function SetupBanner({ onRunSetup }: SetupBannerProps) {
  return (
    <div className="setup-banner">
      <p>⚠️ Configuração necessária para funcionar corretamente</p>
      <button className="setup-btn" onClick={onRunSetup}>
        🔧 Configurar Sistema
      </button>
    </div>
  );
}
