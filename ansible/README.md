# Software deployment

## Setup

- `ansible-galaxy install -r requirements.yml`

## Modem deployment

### Alpine install: prepare SD card

Using [alpine-raspberry-pi-install](https://github.com/DanNixon/alpine-raspberry-pi-install).

- `./make_boot_media.sh /dev/sdX`
- `echo "dtparam=audio=on" | sudo tee boot/usercfg.txt`
- `./umount_boot_media.sh`

### Alpine install: configuration

- Put SD card in Pi and boot
- `mkdir /media/sdX2`
- `mount /dev/sdX2 /media/sdX2/`
- `setup-alpine`
  - (follow instructions for obvious things)
  - (do not configure WiFi)
  - Ensure `sdX2` is set for APK cache and LBU
- `apk add curl python3`
- `mkdir .ssh`
- `curl https://dan-nixon.com/ssh_pubkey.txt > .ssh/authorized_keys`
- `lbu include /root/.ssh/authorized_keys`
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
