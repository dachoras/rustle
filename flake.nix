{
  description = "Minimalist Nix-built container for Rustle";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" ];
        };
        rustPlatform = pkgs.makeRustPlatform {
          rustc = rustVersion;
          cargo = rustVersion;
        };

        # 1. Build the WASM frontend & backend combined in single root
        app = rustPlatform.buildRustPackage {
          pname = "rustle";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [
            rustVersion
            pkgs.wasm-bindgen-cli
            pkgs.trunk
            pkgs.tailwindcss
          ];

          buildPhase = ''
            export HOME=$TMPDIR
            # Build frontend assets
            trunk build --release
            # Build backend server binary
            cargo build --release --bin server
          '';

          installPhase = ''
            mkdir -p $out/bin
            mkdir -p $out/dist
            cp target/release/server $out/bin/server
            cp -r dist/* $out/dist/
          '';
        };

        # 2. Create the layered Docker container image
        dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "rustle-nix";
          tag = "latest";
          
          config = {
            Cmd = [ "${app}/bin/server" ];
            WorkingDir = "/app";
            Env = [
              "PORT=4409"
            ];
            ExposedPorts = {
              "4409/tcp" = {};
            };
            User = "nobody:nobody";
          };

          extraCommands = ''
            mkdir -p app
            cp -r ${app}/dist app/dist
          '';
        };

      in {
        packages = {
          inherit app dockerImage;
          default = dockerImage;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustVersion
            pkgs.trunk
            pkgs.wasm-bindgen-cli
            pkgs.tailwindcss
          ];
        };
      }
    );
}
