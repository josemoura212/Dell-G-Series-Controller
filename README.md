# Dell G Series Controller

🎮 **Controlador moderno para notebooks Dell G Series** - Interface Tauri 2.0 + React + Rust para controle completo de energia, ventiladores e RGB do teclado.

[![Tauri](https://img.shields.io/badge/Tauri-2.0-24C8DB?style=flat-square)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-18-61DAFB?style=flat-square)](https://reactjs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-000000?style=flat-square)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

## ✨ Funcionalidades

### 🎨 Controle RGB do Teclado
- **Modos de iluminação**: Estático, Morph (transição), Estático + Morph, Desligado
- **Seletor de cores**: RGB personalizado (0-255) com preview visual
- **Controle de brilho**: Ajuste fino de 0-100%
- **Suporte USB HID**: Comunicação direta com controladores ELC

### ⚡ Gestão de Energia e Ventiladores
- **Modos de energia**: Silencioso, Balanceado, Performance, Manual, G-Mode
- **Controle manual**: Boost CPU/GPU individual (0-100%)
- **Presets inteligentes**: 4 perfis otimizados (Silencioso, Normal, Turbo, Máximo)
- **Monitoramento**: RPM e temperatura em tempo real
- **ACPI direto**: Chamadas de sistema sem intermediários

### 🖥️ Interface Moderna
- **Tauri 2.0**: Aplicação nativa cross-platform
- **React + TypeScript**: Frontend responsivo e moderno
- **Atualização automática**: Sensores atualizados a cada 3 segundos
- **Estado visual**: Destaque de modos e presets ativos
- **Bandeja do sistema**: Minimiza para ícone na área de notificação

## 🏗️ Arquitetura

```
dell-manjaro/
├── dell-core/              # Biblioteca Rust core
│   ├── src/
│   │   ├── acpi.rs        # Controle ACPI (energia/ventiladores)
│   │   ├── elc.rs         # Controlador LED (USB HID)
│   │   ├── keyboard.rs    # API RGB de alto nível
│   │   └── lib.rs
│   └── Cargo.toml
├── src/                   # Frontend React
│   ├── App.tsx
│   ├── main.tsx
│   └── app/components/
├── src-tauri/            # Backend Tauri
│   ├── src/main.rs
│   └── tauri.conf.json
├── public/               # Assets estáticos
├── setup-acpi.sh         # Script de configuração ACPI
└── package.json
```

## 📦 Instalação

### Pré-requisitos do Sistema

**Arch Linux / Manjaro:**
```bash
sudo pacman -S nodejs npm webkit2gtk base-devel libusb acpi_call rust
```

**Ubuntu/Debian:**
```bash
sudo apt install nodejs npm libwebkit2gtk-4.0-dev build-essential libusb-1.0-0-dev linux-headers-generic rustc cargo
```

**Fedora:**
```bash
sudo dnf install nodejs npm webkit2gtk3-devel libusb1-devel kernel-devel rust cargo
```

### 1. Clonar e Instalar Dependências

```bash
git clone https://github.com/seu-usuario/dell-manjaro.git
cd dell-manjaro
npm install
```

### 2. Configurar Permissões do Sistema

**Permissões USB (Teclado RGB):**
```bash
# Criar regra udev
sudo tee /etc/udev/rules.d/99-dell-g-keyboard.rules << EOF
SUBSYSTEM=="usb", ATTRS{idVendor}=="187c", ATTRS{idProduct}=="0550", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="187c", ATTRS{idProduct}=="0551", MODE="0666"
EOF

# Recarregar regras
sudo udevadm control --reload-rules && sudo udevadm trigger
```

**Permissões ACPI (Energia/Ventiladores):**
```bash
# Executar script de configuração
sudo ./setup-acpi.sh

# Ou configurar manualmente com Polkit
sudo tee /etc/polkit-1/rules.d/50-dell-acpi-nopasswd.rules << EOF
polkit.addRule(function(action, subject) {
    if (action.id == "org.freedesktop.policykit.exec" &&
        action.lookup("program") == "/usr/bin/pkexec" &&
        subject.isInGroup("wheel")) {
        return polkit.Result.YES;
    }
});
EOF
```

### 3. Compilar e Executar

**Modo Desenvolvimento:**
```bash
npm run tauri dev
```

**Build de Produção:**
```bash
npm run tauri build
```

**Instalar pacotes gerados:**
```bash
# DEB (Ubuntu/Debian)
sudo dpkg -i target/release/bundle/deb/Dell\ Controller_1.0.0_amd64.deb

# RPM (Fedora)
sudo rpm -i target/release/bundle/rpm/Dell\ Controller-1.0.0-1.x86_64.rpm
```

## 🎯 Modelos Suportados

| Modelo | Controle de Energia | RGB Teclado | Status |
|--------|-------------------|-------------|---------|
| Dell G15 5530 | ✅ | ❔ | Testado |
| Dell G15 5525 | ✅ | ✅ | Compatível |
| Dell G15 5520 | ✅ | ✅ | Compatível |
| Dell G15 5515 | ✅ | ✅ | Compatível |
| Dell G15 5511 | ✅ | ✅ | Compatível |
| Dell G16 7630 | ✅ | ✅ | Compatível |
| Dell G16 7620 | ✅ | ✅ | Compatível |
| Alienware M16 R1 | ✅ | ❔ | Compatível |

> ❔ = Não testado, mas deve funcionar

## 🚀 Uso

### Interface Principal

1. **Controle RGB**: Selecione modo e ajuste cores no painel superior
2. **Modos de Energia**: Clique nos botões para alternar perfis de energia
3. **Presets de Ventilação**: Use presets rápidos ou controle manual
4. **Monitoramento**: Visualize RPM e temperaturas em tempo real

### Atalhos

- **Minimizar**: Fecha para bandeja do sistema
- **Manual Mode**: Habilita controles de ventilador individuais
- **Performance Mode**: Define ventiladores em 100% automaticamente

## 🔧 Troubleshooting

| Problema | Solução |
|----------|---------|
| "Keyboard not available" | Verifique USB: `lsusb \| grep 187c` e recarregue udev |
| "ACPI not available" | Execute: `sudo modprobe acpi_call` |
| Pkexec pede senha | Execute `./setup-acpi.sh` como root |
| Modelo não detectado | Verifique DMI: `cat /sys/class/dmi/id/product_name` |
| Interface não carrega | Verifique WebKit: `pacman -S webkit2gtk` |

## 🎨 Stack Tecnológico

- **Frontend**: React 18 + TypeScript + Vite + CSS Modules
- **Backend**: Rust 1.70+ + Tauri 2.0
- **Hardware**: rusb (USB HID) + acpi_call (ACPI)
- **Build**: Cargo + npm + rollup
- **Packaging**: Tauri bundler (DEB, RPM, AppImage)

## 📝 Licença

MIT - Veja [LICENSE](LICENSE) para detalhes.

## 🙏 Créditos

- **Versão Python Original**: [cemkaya-mpi/Dell-G-Series-Controller](https://github.com/cemkaya-mpi/Dell-G-Series-Controller)
- **Pesquisa ACPI**: [trackmastersteve/alienfx#41](https://github.com/trackmastersteve/alienfx/issues/41)
- **Comunidade**: Agradecimentos a @AlexIII, @T-Troll e contribuidores

## ⚠️ Aviso Importante

**USE POR SUA CONTA E RISCO**

Este software interage diretamente com hardware através de chamadas ACPI e USB. Embora testado em vários modelos, não há garantia de funcionamento em todos os laptops Dell G Series. Você pode causar danos ao hardware se usar em modelos não suportados ou com configurações incorretas.

---

**Desenvolvido com ❤️ para a comunidade Dell G Series**
