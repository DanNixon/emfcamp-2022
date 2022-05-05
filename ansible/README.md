# Software deployment

## Setup

- `ansible-galaxy install -r requirements.yml`

## Modem deployment

### Alpine install: prepare SD card

- `wget https://dl-cdn.alpinelinux.org/alpine/v3.15/releases/aarch64/alpine-rpi-3.15.0-aarch64.tar.gz`
- `sudo fdisk /dev/mmcblk0`
  - (for anything unspecified, accept the default value presented by fdisk)
  - Create new partition table: `o`
  - Create new primary partition: `n`, `p`
  - Set end sector: `+200M`
  - Select `W95 FAT32 (LBA)` type: `t`, `c`
  - Enable boot flat: `a`
  - Create new primary partition: `n`, `p`
  - Write partitions to disk: `w`
- `sudo mkfs.fat -F32 /dev/mmcblk0p1`
- `sudo mkfs.ext4 /dev/mmcblk0p2`
- `mkdir boot`
- `sudo mount /dev/mmcblk0p1 boot`
- `sudo tar -zxvf alpine-rpi-3.15.0-aarch64.tar.gz -C boot`
- `echo "dtparam=audio=on" > boot/usercfg.txt`
- `sync`
- `sudo umount boot`
- `rmdir boot`

### Alpine install: configuration

- Put SD card in Pi and boot
- `mkdir /media/mmcblk0p2`
- `mount /dev/mmcblk0p2 /media/mmcblk0p/`
- `setup-alpine`
  - (follow instructions for obvious things)
  - (do not configure WiFi)
  - Ensure `mmcblk0p2` is set for APK cache and LBU
- `mkdir .ssh`
- `curl https://dan-nixon.com/ssh_pubkey.txt > .ssh/authorized_keys`
- `lbu include /root/.ssh/authorized_keys`
- `apk add python3`
- `lbu commit`

### Build projects

- Build [UniPager-v1](https://github.com/DanNixon/UniPager-v1) and [remote-closedown](https://github.com/DanNixon/remote-closedown) using [cross-rs](https://github.com/cross-rs/cross): `cross build --release --target aarch64-unknown-linux-musl`
- Set the path to the root of each project in `modem.yml`

### Deployment via Ansible

- `ansible-playbook modem.yml`
- `tailscale up --accept-dns=false --advertise-tags=tag:radio`
- `lbu include /var/lib/tailscale`
- `lbu commit`

## Backend/controller deployment

- Create a K8s cluster somewhere (Linode is always good)
- `ansible-playbook controller.yml`
