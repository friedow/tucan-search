{ index-git-repositories, centerpiece }:
{
  lib,
  pkgs,
  config,
  ...
}:
let
  cfg = config.programs.centerpiece;
  git-index-name = "index-git-repositories";
in
{
  options.programs.centerpiece = {
    enable = lib.mkEnableOption (lib.mdDoc "Centerpiece");

    config = {
      color = {
        text = lib.mkOption {
          default = "#ffffff";
          type = lib.types.str;
          description = lib.mdDoc "Text color within centerpiece.";
        };

        background = lib.mkOption {
          default = "#000000";
          type = lib.types.str;
          description = lib.mdDoc "Background color within centerpiece.";
        };
      };

      plugin = {
        applications = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        brave_bookmarks = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        brave_history = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        brave_progressive_web_apps = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        clock = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        firefox_bookmarks = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        firefox_history = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        git_repositories = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
          zoxide = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable zoxide integration.";
          };
          commands = lib.mkOption {
            default = [
              [
                "alacritty"
                "--command"
                "nvim"
                "$GIT_DIRECTORY"
              ]
              [
                "alacritty"
                "--working-directory"
                "$GIT_DIRECTORY"
              ]
            ];
            type = lib.types.listOf (lib.types.listOf lib.types.str);
            description = lib.mdDoc ''
              The commands to launch when an entry is selected.
              Use the $GIT_DIRECTORY variable to pass in the selected directory.
              Use the $GIT_DIRECTORY_NAME variable to pass in the selected directory name.
            '';
            example = [
              [
                "code"
                "--new-window"
                "$GIT_DIRECTORY"
              ]
              [
                "alacritty"
                "--command"
                "lazygit"
                "--path"
                "$GIT_DIRECTORY"
              ]
              [
                "alacritty"
                "--working-directory"
                "$GIT_DIRECTORY"
              ]
            ];
          };
        };

        gitmoji = {
          enable = lib.mkOption {
            default = false;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        resource_monitor_battery = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        resource_monitor_cpu = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        resource_monitor_disks = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        resource_monitor_memory = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        sway_windows = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        system = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };

        wifi = {
          enable = lib.mkOption {
            default = true;
            type = lib.types.bool;
            description = lib.mdDoc "Enable / disable the plugin.";
          };
        };
      };
    };

    services.index-git-repositories = {
      enable = lib.mkOption {
        default = true;
        type = lib.types.bool;
        description = lib.mdDoc "Enable / disable the git repositories indexer service.";
      };
      interval = lib.mkOption {
        default = "5min";
        type = lib.types.str;
        example = "hourly";
        description = lib.mdDoc ''
          Frequency of index creation.

          The format is described in
          {manpage}`systemd.time(7)`.
        '';
      };
    };
  };

  config = lib.mkMerge [
    (lib.mkIf cfg.enable { home.packages = [ centerpiece ]; })

    (lib.mkIf cfg.enable {
      home.file.".config/centerpiece/config.yml".text = lib.generators.toYAML { } cfg.config;
    })

    (lib.mkIf cfg.services.index-git-repositories.enable {
      systemd.user = {
        services = {
          index-git-repositories-service = {
            Unit = {
              Description = "Centerpiece - your trusty omnibox search";
              Documentation = "https://github.com/friedow/centerpiece";
            };

            Service = {
              ExecStart = "${pkgs.writeShellScript "${git-index-name}-service-ExecStart" ''
                exec ${lib.getExe index-git-repositories}
              ''}";
              Type = "oneshot";
              Nice = 19;
              IOSchedulingClass = "best-effort";
              IOSchedulingPriority = 7;
            };
          };
        };
        timers = {
          index-git-repositories-timer = {
            Unit = {
              Description = "Activate the git repository indexer";
            };
            Install = {
              WantedBy = [ "timers.target" ];
            };
            Timer = {
              OnUnitActiveSec = cfg.services.index-git-repositories.interval;
              OnBootSec = "0min";
              Persistent = true;
              Unit = "${git-index-name}-service.service";
            };
          };
        };
      };
    })
  ];
}
