{
  description = "Minimalist Nix-built container for Rustle";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    shared-assets = {
      url = "github:UberMetroid/shared-assets?ref=v3.0.18";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, shared-assets, ... }:
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
          version = (fromTOML (builtins.readFile ./backend/Cargo.toml)).package.version;
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "shared-core-3.0.18" = "sha256-sjkQrpXtrCQKWk1hQeTw1GHvcpl+tthWKkEWuyZOXSA=";
              "shared-frontend-3.0.18" = "sha256-sjkQrpXtrCQKWk1hQeTw1GHvcpl+tthWKkEWuyZOXSA=";
            };
          };

          nativeBuildInputs = [
            rustVersion
            pkgs.wasm-bindgen-cli
            pkgs.trunk
            pkgs.tailwindcss
          ];

          doCheck = false;

          buildPhase = ''
            export HOME=$TMPDIR
            mkdir -p frontend/shared-assets
            cp -r ${shared-assets}/* frontend/shared-assets/
            # Build frontend assets
            cd frontend
            trunk build --release
            cd ..
            # Build backend server binary
            cargo build --release --bin server --bin sh
          '';

          installPhase = ''
            mkdir -p $out/bin
            mkdir -p $out/dist
            cp target/release/server $out/bin/server
            cp target/release/sh $out/bin/sh
            cp -r frontend/dist/* $out/dist/
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
              "PORT=4502"
              "RUNNING_IN_DOCKER=true"
            ];
            ExposedPorts = {
              "4502/tcp" = {};
            };
            User = "nobody:nobody";
          };

          extraCommands = ''
            mkdir -p app
            cp -r ${app}/dist app/dist
            mkdir -p bin
            cp ${app}/bin/sh bin/sh
            cp ${app}/bin/sh bin/bash
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
