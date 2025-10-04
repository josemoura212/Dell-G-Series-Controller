interface FooterProps {
  statusMessage: string;
  statusError: boolean;
}

export function Footer({ statusMessage, statusError }: FooterProps) {
  return (
    <footer>
      <div className={"status-message " + (statusError ? "error" : "")}>
        {statusMessage}
      </div>
      <p className="info-text">
        💡 Ao fechar a janela, o programa continua em execução na bandeja do
        sistema
      </p>
    </footer>
  );
}
