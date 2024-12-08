# UEFI Application

This repository provides sample UEFI application.

## Setup PXE environment

Install TFTP and DHCP server.

```sh
dnf install -y tftp-server dhcp-server
```

Configure firewall.

```sh
firewall-cmd --permanent --zone=public --add-service=tftp
firewall-cmd --permanent --zone=public --add-service=dhcp
firewall-cmd --permanent --zone=public --add-port=67/tcp
firewall-cmd --reload
```

Configure DHCP server.

```
# /etc/dhcp/dhcpd.conf

option domain-name "home.local";

default-lease-time 60;
max-lease-time 300;

authoritative;

subnet 172.16.1.0 netmask 255.255.255.0 {
  range dynamic-bootp 172.16.1.250 172.16.1.254;
  option broadcast-address 172.16.1.255;
  option routers 172.16.1.1;
  # UEFI Application File Name
  filename "application.efi";
  # PXE Server
  next-server 172.16.1.2;
}
```

Start service.

```sh
systemctl enable --now tftp
systemctl enable --now dhcpd
```

## Create target machine

Create virtual machine without secure boot on KVM .

```xml
<domain type='kvm'>
  <name>uefi</name>
  <memory unit='KiB'>524288</memory>
  <vcpu placement='static'>1</vcpu>
  <os firmware='efi'>
    <type arch='x86_64' machine='pc-q35-rhel9.6.0'>hvm</type>
    <loader readonly='yes' secure='no' type='pflash'>/usr/share/edk2/ovmf/OVMF_CODE.fd</loader>
    <nvram template='/usr/share/edk2/ovmf/OVMF_VARS.fd'>/var/lib/libvirt/qemu/nvram/uefi_VARS.fd</nvram>
    <boot dev='network'/>
  </os>
  <features>
    <acpi/>
    <apic/>
    <smm state='on'/>
  </features>
  <cpu mode='host-passthrough' check='none' migratable='on'/>
  <pm>
    <suspend-to-mem enabled='no'/>
    <suspend-to-disk enabled='no'/>
  </pm>
  <devices>
    <emulator>/usr/libexec/qemu-kvm</emulator>
    <interface type='network'>
      <source network='dhcp'/>
      <model type='virtio'/>
    </interface>
    <graphics type='vnc' port='-1' autoport='yes' listen='0.0.0.0'>
      <listen type='address' address='0.0.0.0'/>
    </graphics>
  </devices>
</domain>
```
