{pkgs ? import (if pin == false then <nixpkgs> else pin) {},
 pin ? ./nixpkgs.nix, ... }:
let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import pkgs.path { overlays = [ moz_overlay ]; };
in with nixpkgs;
let
  # ------------ Commands ----------------

  # ------------------Rust-------------------
  ruststable = latest.rustChannels.stable.rust.override {
    extensions = [ "rust-src" "rls-preview" "rust-analysis" "rustfmt-preview" ];
  };


  rust-pkgs = [
    openssl pkgconfig nasm ruststable cmake zlib
    lldb
    # rustup
#    racer
    rustracer
    gcc
    cmake
    gnumake

    zlib

    # gcc-arm-embedded
    binutils.bintools
  ];

  # embedded-tools = [
  #   gdb-multitarget
  #   openocd
  #   usbutils
  # ];

  shell = mkShell rec {
    name = "rust-env";
    buildInputs = with pkgs; []
    ++ rust-pkgs;

    shellHook = ''
      echo "##### ${name} #####"
    '';
  };

in {
  inherit pkgs rust-pkgs shell;

}
