#!/usr/bin/env bash
# Install mount-luks as a systemd system service on SELinux systems (e.g., Bluefin/Fedora Atomic)
#
# Homebrew installs binaries under /home which SELinux labels as user_home_t.
# System services (init_t) cannot execute user_home_t files.
# This script relabels the mount-luks binary as bin_t and creates a systemd service.
#
# Usage: sudo ./install-service.sh
set -euo pipefail

BREW_PREFIX_DEFAULT="/home/linuxbrew/.linuxbrew"
BREW_PREFIX="${HOMEBREW_PREFIX:-$BREW_PREFIX_DEFAULT}"
BREW_PREFIX_REAL="$(readlink -f "$BREW_PREFIX")"
SERVICE_FILE="/etc/systemd/system/mount-luks.service"

main() {
  validate
  add_selinux_rules
  apply_selinux_labels
  create_service
  enable_service
  echo "Done. Verify with: systemctl status mount-luks.service"
}

validate() {
  if [[ $EUID -ne 0 ]]; then
    echo "Error: must run as root (use sudo)"
    exit 1
  fi
  if [[ ! -f "${BREW_PREFIX}/opt/mount-luks/bin/mount-luks" ]]; then
    echo "Error: mount-luks not found at ${BREW_PREFIX}/opt/mount-luks/bin/mount-luks"
    echo "Install with: brew install studiole/tap/mount-luks"
    exit 1
  fi
}

# Add persistent SELinux file context rules for the brew binary paths.
# Uses the real path (e.g., /var/home) because semanage rejects /home on Fedora Atomic
# where /home is a symlink to /var/home.
add_selinux_rules() {
  echo "Adding SELinux file context rules…"
  add_fcontext "${BREW_PREFIX_REAL}/bin/mount-luks"
  add_fcontext "${BREW_PREFIX_REAL}/opt/mount-luks(/.*)?"
  add_fcontext "${BREW_PREFIX_REAL}/Cellar/mount-luks/.*/bin/mount-luks"
}

# Add a single fcontext rule, updating if it already exists.
add_fcontext() {
  local pattern="$1"
  if ! semanage fcontext -a -t bin_t "$pattern" 2>/dev/null; then
    semanage fcontext -m -t bin_t "$pattern"
  fi
}

apply_selinux_labels() {
  echo "Applying SELinux labels…"
  restorecon -v "${BREW_PREFIX}/bin/mount-luks"
  restorecon -Rv "${BREW_PREFIX}/opt/mount-luks/"
  restorecon -Rv "${BREW_PREFIX}/Cellar/mount-luks/"
}

create_service() {
  echo "Creating ${SERVICE_FILE}…"
  cat > "$SERVICE_FILE" << EOF
[Unit]
Description=Unlock and mount LUKS encrypted disks
After=tpm2.target

[Service]
Type=oneshot
ExecStartPre=/sbin/restorecon ${BREW_PREFIX}/bin/mount-luks
ExecStartPre=/sbin/restorecon -R ${BREW_PREFIX}/opt/mount-luks/
ExecStartPre=/sbin/restorecon -R ${BREW_PREFIX}/Cellar/mount-luks/
ExecStart=${BREW_PREFIX}/opt/mount-luks/bin/mount-luks
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
EOF
}

enable_service() {
  echo "Enabling and starting service…"
  systemctl daemon-reload
  systemctl enable mount-luks.service
  systemctl start mount-luks.service
}

main
