# `mount_luks`

A simple CLI tool to unlock and mount a LUKS encrypted disk.

## Getting Started

### Prerequisites

It is assumed you have already created a LUKS encrypted disk.

### Install

Download the latest binary from [GitHub Releases](https://github.com/StudioLE/mount_luks/releases).

### Create an options file

Create an options file a `.yaml` or `.yml` extension in `/root/.config/mount_luks/` structured as follows:

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
key_path: /root/.config/mount_luks/.key
# Optional
# TPM persistent handle address
tpm_handle: 0x81000000
# Optional
# Should an interactive key be required?
key_prompt: false
```

Check which persistent handles are already in use:

```shell
sudo tpm2_getcap handles-persistent
```

### Set the TPM component of the key

```shell
mount_luks set-tpm
```

### Set the LUKS key

You will need to enter an existing LUKS passphrase.

```shell
mount_luks set-luks
```

### Unlock and mount the LUKS partition

```shell
mount_luks
```
