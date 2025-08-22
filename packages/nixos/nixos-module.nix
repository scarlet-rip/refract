# TODO: Home manager module

{ flakePackages }: 
{ config, lib, pkgs, ... }: let
	inherit (lib) mkIf mkEnableOption;
	inherit (pkgs) system;
in {
	options.programs.refract = {
  		enable = lib.mkEnableOption "refract";

		user = lib.mkOption {
    		type = lib.types.str;
    		description = "The user that should be added to the refract group.";
  		};
	};

	config = mkIf config.programs.refract.enable {
		users = {
			users.refract = {
    			isSystemUser = true;
				group = "refract";
				extraGroups = [ "input" ];
			};

			groups.refract = {};

			users.${config.programs.refract.user} = mkIf config.programs.refract.enable {
  				extraGroups = [ "refract" ];
			};
		};

		systemd = {
			user = {
				services.refract-frontend = {
					description = "Refract Frontend";
					requires = [ "graphical-session.target" ];
					after = [ "graphical-session.target" ];

      				serviceConfig = {
        				Type = "simple";
        				ExecStart = "${flakePackages.${system}.default}/bin/refract-frontend";
        				Restart = "on-failure";
      				};
				};
			};

			services.refract-backend = {
				description = "Refract Backend";

      			serviceConfig = {
        			Type = "simple";
        			User = "refract";
        			Group = "refract";
					SupplementaryGroups = [ "input" ];
        			ExecStart = "${flakePackages.${system}.default}/bin/refract-backend";
        			Restart = "on-failure";
      			};
			};
		};

		security.polkit.extraConfig = 
		''
  			polkit.addRule(function(action, subject) {
    			if (action.id == "org.freedesktop.systemd1.manage-units" &&
        			action.lookup("unit") == "refract-backend.service" &&
        			subject.isInGroup("refract")) {
      					return polkit.Result.YES;
    				}
  			});
		'';
	};
}
