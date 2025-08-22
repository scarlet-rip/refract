{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";

	refract.url = "./packages/nixos";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
		  enigoDeps = with pkgs; [ xdotool ];
		  eframeDeps = with pkgs; [ libxkbcommon libGL wayland ];
		  udevDeps = with pkgs; [ libudev-zero ];

          runtimeDeps = enigoDeps ++ eframeDeps ++ udevDeps;
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
			  test() {
    			cargo build

    			sudo -u "refract" ./target/debug/refract-backend &
    			backend_pid=$!

    			sleep 1

    			./target/debug/refract-frontend &
    			frontend_pid=$!

				cleanup() {
        			sudo kill -9 $backend_pid $frontend_pid 2>/dev/null
    			}

    			trap cleanup EXIT INT TERM

    			wait
    			}
  			  '';
            };
        in {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          devShells.default = mkDevShell pkgs.rust-bin.stable.latest.default;
		  packages.default = inputs.refract.packages.default;
        };

		flake.nixosModules.default = inputs.refract.nixosModules.default;
    };
}
