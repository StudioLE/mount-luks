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

Homebrew installs binaries under `/home`, which SELinux leaves labelled `default_t`. System services run in the `init_t` domain, which cannot execute `default_t` files, so the service fails with `203/EXEC` and `Permission denied`. The script uses `semanage fcontext` to persistently relabel the binary as `bin_t`.

## Fedora Atomic and `/var/home`

On Fedora Atomic (Bluefin, Bazzite, Aurora), `/home` is a symlink to `/var/home`, and `file_contexts.subs_dist` carries an equivalency between the two. Lookups are rewritten to whichever form is canonical before any matching happens, and `semanage` refuses a spec starting with the other one.

Releases have shipped that equivalency in both directions. A flip silently invalidates rules written for the previous direction: they stay in `file_contexts.local` looking correct, match nothing, and `restorecon` stamps `default_t` back onto the binary at every service start.

The script writes `/(var/)?home/…` so one rule matches either form. The leading `(` also keeps the spec clear of the literal prefix check `semanage` uses to reject the aliased-away form.

## Brew upgrades

The systemd service includes `ExecStartPre` commands that run `restorecon` before each start. When Homebrew installs a new version, the new binary initially gets `default_t`, but the next boot relabels it automatically using the persistent `semanage` rules.

## Manual removal

```shell
sudo systemctl disable --now mount-luks.service
sudo rm /etc/systemd/system/mount-luks.service
sudo systemctl daemon-reload
```

To also remove the SELinux rules:

```shell
sudo semanage fcontext -d '/(var/)?home/linuxbrew/\.linuxbrew/bin/mount-luks'
sudo semanage fcontext -d '/(var/)?home/linuxbrew/\.linuxbrew/opt/mount-luks(/.*)?'
sudo semanage fcontext -d '/(var/)?home/linuxbrew/\.linuxbrew/Cellar/mount-luks/.*/bin/mount-luks'
```
