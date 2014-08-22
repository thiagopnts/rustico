#!/bin/bash

qemu-img create disk.img 2M

# 66 heads, 1 sector, 62 tracks
fdisk disk.img << EOF
x     # enter expert mode
c     # cylinders
4     # set cylinders to 4
r     # return to main menu
n     # new partition
p     # set new partition as primary
1     # choose the first partition
1     # default value for sectors
4095  # empty for default option
a     # toogle bootable flag
1     # choose the first partition
w     # write all changes
EOF

losetup -o 32256 /dev/loop1 disk.img

mke2fs /dev/loop1

mkdir -p /mnt/disk
mount /dev/loop1 /mnt/disk
echo mounted

umount /mnt/disk

losetup -d /dev/loop1


