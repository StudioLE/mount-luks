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
# Optional path to a file containing the LUKS key
key_path: /root/.config/mount_luks/.key
```
