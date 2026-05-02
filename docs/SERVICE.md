# Run at boot with systemd

Run `mount-luks` automatically at boot as a systemd system service.

## Quick start

```shell
sudo ./docs/scripts/install-service.sh
```

Verify:

```shell
systemctl status mount-luks.service
journalctl -u mount-luks.service
```

## What the script does

1. Adds SELinux file context rules to label the Homebrew binary as `bin_t`
2. Applies the labels with `restorecon`
3. Creates a systemd service at `/etc/systemd/system/mount-luks.service`
4. Enables and starts the service

## Why SELinux rules are needed

Homebrew installs binaries under `/home` which SELinux labels as `user_home_t`. System services run in the `init_t` domain which cannot execute `user_home_t` files. The script uses `semanage fcontext` to persistently relabel the binary as `bin_t`.

On Fedora Atomic (Bluefin, Bazzite, Aurora), `/home` is a symlink to `/var/home`. The script resolves this automatically.

## Brew upgrades

The systemd service includes `ExecStartPre` commands that run `restorecon` before each start. When Homebrew installs a new version, the new binary initially gets `user_home_t`, but the next boot relabels it automatically using the persistent `semanage` rules.

## Manual removal

```shell
sudo systemctl disable --now mount-luks.service
sudo rm /etc/systemd/system/mount-luks.service
sudo systemctl daemon-reload
```

To also remove the SELinux rules:

```shell
BREW_PREFIX_REAL="$(readlink -f /home/linuxbrew/.linuxbrew)"
sudo semanage fcontext -d "${BREW_PREFIX_REAL}/bin/mount-luks"
sudo semanage fcontext -d "${BREW_PREFIX_REAL}/opt/mount-luks(/.*)?"
sudo semanage fcontext -d "${BREW_PREFIX_REAL}/Cellar/mount-luks/.*/bin/mount-luks"
```
