{ pkgs, toolchain }:

pkgs.mkShell {
  packages = [
    toolchain
    pkgs.rust-analyzer
  ];
}