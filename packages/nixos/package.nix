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
    	rev = "dfec26e505182f6872e086b8fe176a87ba0d7c12";
    	hash = "sha256-W+Z5DU7EqGCNWPidb4jtizUTE30vGTR3i2NoXHUZD78=";
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
