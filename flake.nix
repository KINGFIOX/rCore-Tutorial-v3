# update in 2023-05-24 21:26
{
  description = "A devShell for rCore";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # nixpkgs-qemu7.url = "https://github.com/NixOS/nixpkgs/archive/7cf5ccf1cdb2ba5f08f0ac29fc3d04b0b59a07e4.tar.gz";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)
          (self: super: {
            # ref: https://github.com/the-nix-way/dev-templates
            rust-toolchain = let rust = super.rust-bin;
            in if builtins.pathExists ./rust-toolchain.toml then
              rust.fromRustupToolchainFile ./rust-toolchain.toml
            else if builtins.pathExists ./rust-toolchain then
              rust.fromRustupToolchainFile ./rust-toolchain
            else
            # The rust-toolchain when i make this file, which maybe change
              (rust.nightly."2022-08-05".minimal.override {
                extensions =
                  [ "rust-src" "llvm-tools-preview" "rustfmt" "clippy" ];
                targets = [ "riscv64gc-unknown-none-elf" ];
              });
          })
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        # pkg-qemu = import nixpkgs-qemu7 { inherit system; };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = (with pkgs; [
            # Basic
            openssl
            pkg-config
            eza
            fd
            libclang
            # Cross Compile
            (with pkgsCross.riscv64;
              [
                musl.stdenv.cc
              ]) # If use normally, no necessary need to change.
            # Rust Configuraiton  
            rustup
            cargo-binutils
            rust-toolchain
          ]) ++ [
            pkgs.qemu
            # pkg-qemu.qemu
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
          '';
        };
      });
}
