with import <nixpkgs> {};
import ./encase.nix {
  name = "figura-fok-sandbox";
  net = true;
  command = "${bash}/bin/bash";
  ro.etc = /etc;
  ro.dev = /dev;
  ro.nix = /nix;
  rw.home = /home/foko/Projects/FokPack/FokNetOld/repurposeforfigua/sandboxes/sandbox_dirs/home;
  ro.rules = /home/foko/Projects/FokPack/FokNetOld/repurposeforfigua/sandboxes/sandbox_dirs/rules;
  proc = /proc;
  ro.sys = /sys;
}
