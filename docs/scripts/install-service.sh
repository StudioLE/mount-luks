#!/usr/bin/env bash
# Install mount-luks as a systemd system service on SELinux systems (e.g., Bluefin/Fedora Atomic)
#
# Homebrew installs binaries under /home, which SELinux leaves labelled default_t.
# System services (init_t) cannot execute default_t files.
# This script relabels the mount-luks binary as bin_t and creates a systemd service.
#
# Usage: sudo ./install-service.sh
set -euo pipefail

BREW_PREFIX_DEFAULT="/home/linuxbrew/.linuxbrew"
BREW_PREFIX="${HOMEBREW_PREFIX:-$BREW_PREFIX_DEFAULT}"
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
add_selinux_rules() {
  local prefix
  prefix="$(fcontext_prefix)"
  echo "Adding SELinux file context rules…"
  add_fcontext "${prefix}/bin/mount-luks"
  add_fcontext "${prefix}/opt/mount-luks(/.*)?"
  add_fcontext "${prefix}/Cellar/mount-luks/.*/bin/mount-luks"
}

# Build the file spec prefix as a regex matching both /home and /var/home.
# Fedora Atomic ships a /var/home equivalency in file_contexts.subs_dist and has
# changed its direction between releases. Lookups are rewritten to whichever form
# is canonical before matching, and semanage rejects a spec starting with the
# other one, so an optional /var segment stays valid either way.
# Output: regex prefix, with '.' escaped
fcontext_prefix() {
  local escaped="${BREW_PREFIX//./\\.}"
  case "$escaped" in
    /var/home/*) printf '/(var/)?%s' "${escaped#/var/}" ;;
    /home/*) printf '/(var/)?%s' "${escaped#/}" ;;
    *) printf '%s' "$escaped" ;;
  esac
}

# Add a single fcontext rule. semanage falls back to modify if it already exists.
add_fcontext() {
  local pattern="$1"
  semanage fcontext -a -t bin_t "$pattern"
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
