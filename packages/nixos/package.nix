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
    	rev = "11ccd763484ad70dd9ac460f6d75050c95029821";
    	hash = "sha256-zskAwb/cJEncen5IxEj/Y+Awf/IpvhDR89UJ2/hcvzc=";
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
