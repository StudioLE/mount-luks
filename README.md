# `mount-luks`

## Overview

A simple CLI tool to unlock and mount a LUKS encrypted disk.

The passphrase is concatenated from multiple optional sources ensuring the highest level of security:

### TPM (Trusted Platform Module) 2.0

- Hardware integrity check
- Prevents unlock if bootloader/firmware is tampered with
- Protects against evil maid attacks and key extraction

### File on removable media (USB)

- Possession factor – something you have
- Prevents unlock without physical access to the USB
- Protects against remote attacks and theft (if USB stored separately)
- Kept physically secure when not required

### Interactive PIN or password

- Knowledge factor – something you know
- Prevents unlock by unauthorized users with physical access
- Protects against simple theft (even if they have your laptop + USB)

### Combined defense:

- Thief steals laptop only → blocked by USB and PIN
- Thief steals laptop and USB → blocked by PIN
- Attacker tampers with bootloader → blocked by TPM
- Someone finds your USB → blocked by needing laptop and PIN
- Remote/software attack → blocked by all three

## Getting Started

### Requirements

- It is assumed you have already created a LUKS encrypted disk.
- [tpm2-tools](https://tpm2-tools.readthedocs.io/en/latest/)
- Root access

### Install

Install the latest version with homebrew:

```shell
brew install studiole/tap/mount-luks
```

Or download the binary from [GitHub Releases](https://github.com/StudioLE/mount-luks/releases).

### Create an options file

Create an options file with a `.yaml` or `.yml` extension in `/root/.config/mount-luks/` structured as follows:

```yaml
# Path of the LUKS partition
partition_path: /dev/nvme0n1p9
# Name to use for the mapper device
mapper_name: e
# Path to mount the unlocked LUKS partition
mount_path: /mnt/e
# Optional
# Path to a file containing the LUKS key
# Ideally this is stored on an external USB device which is removed when not required
key_path: /root/.config/mount-luks/.key
# Optional
# TPM persistent handle address
tpm_handle: 0x81000000
# Optional
# Should an interactive key be required?
key_prompt: false
```

The `tpm_handle` must be unique and is ideally sequentially, so check which persistent handles are already in use:

```shell
sudo tpm2_getcap handles-persistent
```

### Save the file component of the key

Generate a random key using your preferred method and save it to the `key_path` file.

```shell
tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 128 | sudo tee /root/.config/mount-luks/.key > /dev/null
```

### Save the TPM component of the key

Generate a random key using your preferred method.

```shell
tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 128
```

Copy the key to the clipboard.

Run the `set-tpm` sub command and paste the key when prompted.

```shell
sudo mount-luks set-tpm
```

### Save the concatenated key to LUKS

LUKS has multiple keyslots so your existing passphrase will not be replaced or overwritten.

To check the existing key slots:

```shell
sudo cryptsetup luksDump /dev/nvme0n1p9
```

Once you've saved the new key with `mount-luks` you should **NOT** remove your existing passphrase as it will be needed to
update the TPM data if your secure boot configuration changes.

Run the `set-luks` sub command to save the concatenated key to LUKS.

Enter your **existing** LUKS passphrase when prompted.

```shell
sudo mount-luks set-luks
```

### Validate the key works

To check the key works you can run the `validate` sub command:

```shell
sudo mount-luks validate
```

### Unlock and mount the LUKS partition

You can now unlock and mount the LUKS partition using the `mount-luks` command:

```shell
sudo mount-luks
```
