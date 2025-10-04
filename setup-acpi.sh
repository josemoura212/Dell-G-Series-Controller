#!/bin/bash
# Configure passwordless ACPI access for Dell G Series Controller

set -e

echo "======================================"
echo "Dell G Series - Configure ACPI Access"
echo "======================================"
echo ""

if [ "$EUID" -ne 0 ]; then 
    echo "⚠️  Execute with sudo: sudo ./setup-acpi.sh"
    exit 1
fi

echo "📋 Installing polkit rules..."
echo ""

# Install polkit rule
if [ -f "50-dell-acpi-nopasswd.rules" ]; then
    cp 50-dell-acpi-nopasswd.rules /etc/polkit-1/rules.d/
    chmod 644 /etc/polkit-1/rules.d/50-dell-acpi-nopasswd.rules
    echo "✓ Polkit rule installed in /etc/polkit-1/rules.d/"
else
    echo "❌ File 50-dell-acpi-nopasswd.rules not found"
    exit 1
fi

echo ""
echo "📋 Installing udev rules for USB keyboard..."
echo ""

# Install udev rule
if [ -f "99-dell-g-series.rules" ]; then
    cp 99-dell-g-series.rules /etc/udev/rules.d/
    chmod 644 /etc/udev/rules.d/99-dell-g-series.rules
    echo "✓ Udev rule installed in /etc/udev/rules.d/"
else
    echo "❌ File 99-dell-g-series.rules not found"
    exit 1
fi

# Reload udev rules
echo ""
echo "🔄 Reloading udev rules..."
udevadm control --reload-rules
udevadm trigger
echo "✓ Udev rules reloaded"

# Restart polkit
echo ""
echo "🔄 Restarting polkit service..."
if systemctl is-active --quiet polkit.service; then
    systemctl restart polkit.service
    echo "✓ Polkit service restarted"
fi

echo ""
echo "======================================"
echo "✅ Configuration complete!"
echo "======================================"
echo ""
echo "What was done:"
echo "  - Users in 'wheel' group can use pkexec without password for ACPI"
echo ""
echo "⚠️  IMPORTANT:"
echo "  - LOGOUT and LOGIN again to apply changes"
echo "  - Or restart the system: sudo reboot"
echo ""
echo "After that, run the program:"
echo "  ./target/release/dell-g-controller"
echo ""
