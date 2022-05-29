# Software deployment

## Setup

- `ansible-galaxy install -r requirements.yml`

## Backend/controller deployment

- Create a K8s cluster somewhere (Linode is always good)
- `ansible-playbook cloud.yml`

## MB7PMF modem deployment

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

### Deployment via Ansible

- `ansible-playbook dapnet_modem.yml`
- `tailscale up --accept-dns=false --advertise-tags=tag:radio`
- `lbu include /var/lib/tailscale`
- `lbu commit`

## Thermal printer controller deployment

### Alpine install: prepare SD card

Using [alpine-raspberry-pi-install](https://github.com/DanNixon/alpine-raspberry-pi-install).

### Alpine install: configuration

As per above.

### Deployment via Ansible

- `ansible-playbook dapnet_modem.yml`
- `tailscale up --accept-dns=false --advertise-tags=tag:emfcamp`
- `lbu include /var/lib/tailscale`
- `lbu commit`
