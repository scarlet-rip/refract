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
		  eframeDeps = with pkgs; [ libxkbcommon libGL wayland ];

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
# Process input devices
printf "USB Devices:"

while IFS= read -r line; do
    if [[ $line == *"Name="* ]]; then
		device_name=$(echo "$line" | awk -F'\"' '{print $2}')
    fi

    if [[ $line == *"Handlers="* ]]; then
        handlers=$(echo "$line" | grep -o "event[0-9]*")
        if [[ -n $handlers ]]; then
            printf "%s -> %s\n" "$device_name" "$handlers"
        fi
    fi
done < /proc/bus/input/devices

# Prompt user to specify event numbers
echo "Select event numbers from the above list."

# First event number
while :; do
    echo -n "Enter the first event number (e.g., 0 for /dev/input/event0): "
    read -r event1
    if [[ -e "$(printf "/dev/input/event%d" "$event1")" ]]; then
        break
    else
        echo "Invalid event number. Please try again."
    fi
done

# Second event number
while :; do
    echo -n "Enter the second event number (e.g., 26 for /dev/input/event26): "
    read -r event2
    if [[ -e "$(printf "/dev/input/event%d" "$event2")" ]]; then
        break
    else
        echo "Invalid event number. Please try again."
    fi
done

# Construct the event paths
event_path1=$(printf "/dev/input/event%d" "$event1")
event_path2=$(printf "/dev/input/event%d" "$event2")

# Set permissions
sudo setfacl -m u:"$(whoami)":rw "$event_path1"
sudo setfacl -m u:"$(whoami)":rw "$event_path2"

# Set up trap to remove permissions on exit or interrupt
trap 'sudo setfacl -x u:"$(whoami)" "$event_path1" && sudo setfacl -x u:"$(whoami)" "$event_path2"' SIGINT EXIT

# Notify user
echo "Permissions set for $event_path1 and $event_path2. Permissions will be removed on exit"
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
