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
    	rev = "3ef5056c8dfb8ac0db9c3a059b7c00c8a38d7c28";
    	hash = "sha256-EVs0W9oDtQYdjVZ/RoS71aKBJn5Zya/no4ENxUXG3+c=";
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
