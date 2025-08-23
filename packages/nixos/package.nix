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
    	rev = "6ecf2281a241caa8efc0e8c8ad6f5abefd6b3183";
    	hash = "sha256-iwYjzPAZRQ4zaa4YVP6Y+qER+x0eeI4asRPhb0s/n8k=";
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
