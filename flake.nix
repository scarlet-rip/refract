{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
		  enigoDeps = with pkgs; [ xdotool ];
		  eframeDeps = with pkgs; [ libxkbcommon libGL wayland xorg.libX11 libudev-zero ];
          runtimeDeps = enigoDeps ++ eframeDeps;

          buildDeps = with pkgs; [ pkg-config ];

          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          msrv = cargoToml.package.rust-version;

          rustPackage = features:
            (pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable.latest.minimal;
              rustc = pkgs.rust-bin.stable.latest.minimal;
            }).buildRustPackage {
              inherit (cargoToml.package) name version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
              buildFeatures = features;
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps;
              doCheck = false;
            };

          mkDevShell = rustc:
            pkgs.mkShell {
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps ++ [ rustc ];
			  LD_LIBRARY_PATH = "${lib.makeLibraryPath runtimeDeps}";
			  shellHook = 
			  ''
			  export PKG_CONFIG_PATH="$PKG_CONFIG_PATH:${pkgs.libxkbcommon}/lib/pkgconfig"

			  echo "newgrp might ask for the password of the input group"
			  newgrp input
  			  '';
            };
        in {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          devShells.default = mkDevShell pkgs.rust-bin.stable.latest.default;
        };
    };
}
