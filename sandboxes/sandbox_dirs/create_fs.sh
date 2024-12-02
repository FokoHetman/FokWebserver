truncate -s 100M $1
shred -zn0 $1
mkfs.ext4 $1
