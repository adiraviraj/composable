{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, devnetTools, ... }: {
    packages = let packages = self'.packages;
    in rec {
      centauri-configure-and-run = pkgs.writeShellApplication rec {
        name = "centauri-configure-and-run";
        text = ''
          cp --force ${self'.packages.hyperspace-config} /tmp/config.toml  
          ${pkgs.lib.meta.getExe devnet-centauri}       
        '';
      };

      devnet-centauri = pkgs.composable.mkDevnetProgram "devnet-centauri"
        (import ./specs/centauri.nix {
          inherit pkgs devnetTools packages;
          devnet-a = packages.zombienet-picasso-centauri-a;
          devnet-b = packages.zombienet-composable-centauri-b;
        });

      devnet-centauri-no-relay =
        pkgs.composable.mkDevnetProgram "devnet-centauri"
        (import ./specs/centauri.nix {
          inherit pkgs devnetTools packages;
          devnet-a = packages.zombienet-picasso-centauri-a;
          devnet-b = packages.zombienet-composable-centauri-b;
          hyperspace-relay = false;
        });

      centauri-no-relay = pkgs.writeShellApplication rec {
        name = "centauri-configure-and-run";
        text = ''
          cp --force ${self'.packages.hyperspace-config} /tmp/config.toml  
          ${pkgs.lib.meta.getExe devnet-centauri-no-relay}       
        '';
      };

      devnet-picasso-image = devnetTools.buildDevnetImage {
        name = "devnet-picasso";
        container-tools = devnetTools.withDevNetContainerTools;
        devNet = packages.zombienet-rococo-local-picasso-dev;
      };

      devnet-picasso-complete = packages.zombienet-picasso-complete;
      devnet-initialize-script-local = devnetTools.mkDevnetInitializeScript {
        polkadotUrl = "ws://localhost:9944";
        composableUrl = "ws://localhost:9988";
        parachainIds = [ 1000 2000 2087 ];
      };

      devnet-initialize-script-picasso-persistent =
        devnetTools.mkDevnetInitializeScript {
          polkadotUrl =
            "wss://persistent.picasso.devnets.composablefinance.ninja/chain/rococo";
          composableUrl =
            "wss://persistent.picasso.devnets.composablefinance.ninja/chain/picasso";
          parachainIds = [ 1000 2000 2087 ];
        };

      devnet = pkgs.composable.mkDevnetProgram "devnet-default"
        (import ./specs/default.nix {
          inherit pkgs devnetTools;
          price-feed = packages.price-feed;
          devnet = packages.devnet-picasso-complete;
          frontend = packages.frontend-static;
        });

      devnet-xcvm = pkgs.composable.mkDevnetProgram "devnet-xcvm"
        (import ./specs/xcvm.nix {
          inherit pkgs devnetTools;
          devnet-picasso = packages.zombienet-rococo-local-picasso-dev;
        });

      devnet-picasso-persistent =
        pkgs.composable.mkDevnetProgram "devnet-picasso-persistent"
        (import ./specs/default.nix {
          inherit pkgs devnetTools;
          price-feed = packages.price-feed;
          devnet = packages.devnet-picasso-complete;
          frontend = packages.frontend-static-picasso-persistent;
        });
    };
  };
}
