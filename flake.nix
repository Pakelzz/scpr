{
  description = "My Project";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
  let 
    system = "x86_64-linux";
    overlays = [
      rust-overlay.overlays.default
    ];
    pkgs = import nixpkgs { inherit system overlays; };
  in {
    devShells.${system} = {
      default = pkgs.mkShell {
        packages = with pkgs; [
          openssl
          pkg-config
          gcc
          (rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
            targets = [ "x86_64-unknown-linux-musl" ];
          })
        ];

        shellHook = ''
          if [ -f .env ]; then
            set -a
            source .env
            set +a
          fi
        '';
      };
    };
  };
}