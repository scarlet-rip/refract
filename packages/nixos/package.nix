{ lib, fetchFromGitHub, rustPlatform, pkgs }:

let
	buildInputs = with pkgs; [
		xdotool
		libxkbcommon
		libGL
		wayland
	];

	libraryPaths = lib.makeLibraryPath buildInputs;
in rustPlatform.buildRustPackage rec {
	pname = "refract";
  	version = "0.0.0";

  	src = fetchFromGitHub {
    	owner = "scarlet-rip";
    	repo = "refract";
    	rev = "e558a5af6835a771dcb89c4d2554d16449942cac";
    	hash = "sha256-jvKprA8xiahOO5wzdwpL6ri2OLKLZNwA5DEaZqXtZqE=";
  	};

	cargoLock = {
   		lockFile = src + /Cargo.lock;
		outputHashes = {
         	"scarlet-egui-0.0.0" = "sha256-GmacGcMrbFt1/240yOOMUrWVQip1FrfSDwt9QiGXJTM=";
       	};
  	};

	inherit buildInputs;

	nativeBuildInputs = with pkgs; [
		makeWrapper
    ];

	postConfigure = 
	''
		mkdir -p $out/share
		cp -r assets "$out/share/"
	'';

	postInstall = 
	''
    	wrapProgram $out/bin/refract-frontend \
      	--prefix LD_LIBRARY_PATH : "${ libraryPaths }" \
		--prefix REFRACT_ASSETS_DIRECTORY : $out/share/assets
  	'';
}
