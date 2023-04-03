{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, subnix, ... }:
    let
      protocattrs = {
        BuildInputs = [ pkgs.protobuf ];
        PROTOC = "${pkgs.protobuf}/bin/protoc";
        PROTOC_INCLUDE = "${pkgs.protobuf}/include";
        PROTOC_NO_VENDOR = "1";
      };
      centauri-src = pkgs.fetchFromGitHub {
        owner = "ComposableFi";
        repo = "centauri";
        rev = "54a1c42553d18160f5e89542d87aea6fcc95b4b5";
        hash = "sha256-rnKUfGcF9TTSockx/YqJzpsPPu23jplc4BiOyoOSsV8=";
      };
      hyperspace-client-template = {
        chain_a = {     
          channel_whitelist = [ ];
          client_id = "10-grandpa-0";
          commitment_prefix = "0x6962632f";
          finality_protocol = "Grandpa";
          key_type = "sr25519";
          name = "picasso_1";
          para_id = 2087;
          parachain_rpc_url = "ws://devnet-a:9988";
          private_key = "//Alice";
          relay_chain_rpc_url = "ws://devnet-a:9944";
          ss58_version = 49;
          type = "picasso_kusama";
        };
        chain_b = {
          type="composable";
          channel_whitelist = [ ];
          client_id = "10-grandpa-0";
          commitment_prefix = "0x6962632f";
          finality_protocol = "Grandpa";
          key_type = "sr25519";
          name = "picasso_2";
          para_id = 2087;
          parachain_rpc_url = "ws://devnet-b:29988";
          private_key = "//Alice";
          relay_chain_rpc_url = "ws://devnet-b:29944";
          ss58_version = 50;
        };
        core = { prometheus_endpoint = "https://127.0.0.1"; };
      };

      hyperspace-connection-template = hyperspace-client-template // {
        chain_a = hyperspace-client-template.chain_a // {
          connection_id = "connection-0";
        };
        chain_b = hyperspace-client-template.chain_b // {
          connection_id = "connection-0";
        };
      };
    in {
      packages = rec {
        centauri-codegen = crane.stable.buildPackage {
          name = "centauri-codegen";
          cargoArtifacts = crane.stable.buildDepsOnly {
            src = centauri-src;
            doCheck = false;
            cargoExtraArgs = "-p codegen";
            cargoTestCommand = "";
          };
          src = centauri-src;
          doCheck = false;
          cargoExtraArgs = "-p codegen";
          cargoTestCommand = "";
          meta = { mainProgram = "codegen"; };
        };
        centauri-hyperspace = crane.stable.buildPackage (subnix.subenv // {
          name = "centauri-hyperspace";
          cargoArtifacts = crane.stable.buildDepsOnly (subnix.subenv // {
            src = centauri-src;
            doCheck = false;
            cargoExtraArgs = "-p hyperspace";
            cargoTestCommand = "";
          });
          src = centauri-src;
          doCheck = false;
          cargoExtraArgs = "-p hyperspace";
          cargoTestCommand = "";
          meta = { mainProgram = "hyperspace"; };
        });

        # no worries, long names not for public use, just to avoid mistakes
        composable-rococo-picasso-rococo-subxt-hyperspace-patch =
          pkgs.stdenv.mkDerivation rec {
            name = "composable-rococo-picasso-rococo-subxt-hyperspace-patch";
            pname = "${name}";
            buildInputs = [
              self'.packages.composable-rococo-subxt-client
              self'.packages.picasso-rococo-subxt-client
            ];
            src = centauri-src;
            patchPhase = "true";
            installPhase = ''
              mkdir --parents $out
              set +e
              diff --exclude=mod.rs --recursive --unified $src/utils/subxt/generated/src/composable ${self'.packages.composable-rococo-subxt-client}/ > $out/composable_polkadot.patch            
              diff --exclude=mod.rs --recursive --unified $src/utils/subxt/generated/src/picasso_kusama ${self'.packages.picasso-rococo-subxt-client}/ > $out/picasso_kusama.patch            
              if [[ $? -ne 1 ]] ; then
                echo "Failed diff"              
              fi              
              set -e 
            '';
            dontFixup = true;
            dontStrip = true;
          };

        composable-rococo-picasso-rococo-centauri-patched-src =
          pkgs.stdenv.mkDerivation rec {
            name = "composable-rococo-picasso-rococo-centauri-patched-src";
            pname = "${name}";
            src = centauri-src;
            buildInputs = with pkgs; [ sd git ];
            patches = [
              "${composable-rococo-picasso-rococo-subxt-hyperspace-patch}/picasso_kusama.patch"
              "${composable-rococo-picasso-rococo-subxt-hyperspace-patch}/composable_polkadot.patch"
            ];
            patchFlags = "--strip=4";
            installPhase = ''
              mkdir --parents $out
              cp --recursive --no-preserve=mode,ownership $src/. $out/
              cp ${./composable.patch} "$out/hyperspace/core/src/substrate/"

              cd $out/utils/subxt/generated/src/picasso_kusama
              patch ${patchFlags} -- < ${builtins.head patches}

              cd $out/utils/subxt/generated/src/composable
              patch ${patchFlags} -- < ${builtins.head (builtins.tail patches)}
              sd "rococo" "polkadot" "$out/utils/subxt/generated/src/composable/relaychain.rs"

              cd "$out/hyperspace/core/src/substrate/"
              patch -- < ${./composable.patch}

            '';
            dontFixup = true;
            dontStrip = true;
          };

        hyperspace-config = pkgs.writeText "config.toml"
          (self.inputs.nix-std.lib.serde.toTOML hyperspace-connection-template);

        hyperspace-composable-rococo-picasso-rococo-image =
          pkgs.dockerTools.buildImage {
            tag = "latest";
            name = "hyperspace-composable-rococo-picasso-rococo";
            config = {
              Entrypoint = [
                "${hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace"
              ];
            };
          };

        hyperspace-composable-rococo-picasso-rococo = crane.stable.buildPackage
          (protocattrs // rec {
            name = "hyperspace-composable-rococo-picasso-rococo";
            pname = "${name}";
            cargoArtifacts = crane.stable.buildDepsOnly (protocattrs // {
              src = composable-rococo-picasso-rococo-centauri-patched-src;
              doCheck = false;
              cargoExtraArgs = "--package hyperspace";
              cargoTestCommand = "";
            });
            src = composable-rococo-picasso-rococo-centauri-patched-src;
            doCheck = false;
            cargoExtraArgs = "--package hyperspace";
            cargoTestCommand = "";
            meta = { mainProgram = "hyperspace"; };
          });
      };
    };
}
