{ self, ... }: {
  perSystem =
    { config
    , self'
    , inputs'
    , pkgs
    , lib
    , system
    , crane
    , systemCommonRust
    , subnix
    , ...
    }:
    let
      buildPolkadotNode =
        { name, version, repo, owner, rev, hash, cargoSha256 }:
        pkgs.rustPlatform.buildRustPackage (rec {
          inherit name version cargoSha256;
          src = pkgs.fetchgit {
            url = "https://github.com/${owner}/${repo}.git";
            inherit rev;
            sha256 = hash;
            fetchSubmodules = false;
          };

          meta = { mainProgram = "polkadot"; };

          __noChroot = true;

        } // subnix.subenv);
      cargo-lock = builtins.fromTOML (builtins.readFile ../../code/Cargo.lock);
      rococo-runtime-dep = builtins.head (builtins.filter (x: x.name == "rococo-runtime") (cargo-lock.package));
      rococo-runtime-commit = builtins.elemAt (builtins.split "#" rococo-runtime-dep.source) 2;
    in
    {
      packages =
        rec {
          rococo-wasm-runtime-9360 = pkgs.stdenv.mkDerivation {
            name = "rococo-wasm-runtime";
            dontUnpack = true;
            src = pkgs.fetchurl {
              url =
                "https://github.com/paritytech/polkadot/releases/download/v0.9.36/rococo_runtime-v9360.compact.compressed.wasm";
              hash = "sha256-inq526PxU2f4+m4RSTiv5oOpfSZfnQpXkhpYmqZ9gOs=";
            };
            installPhase = ''
              mkdir -p $out/lib
              cp $src $out/lib/rococo_runtime.compact.compressed.wasm
            '';
          };
          rococo-wasm-runtime-current = rococo-wasm-runtime-9360;

          polkadot-node-dep =
            let version = "current";
            in buildPolkadotNode rec {
              name = rococo-runtime-commit;
              inherit version;
              repo = "polkadot";
              owner = "paritytech";
              rev = rococo-runtime-commit;
              hash = "sha256-x2IEIHxH8Hg+jFIpnPrTsqISEAZHFuXhJD+H1S+G3nk=";
              cargoSha256 = "sha256-639VMQBvDIRPlCTQVGsS8Xy3Y6nJVqO5VDkuCDQtjqg=";
            };

          polkadot-node-9360 =
            let version = "v0.9.36";
            in buildPolkadotNode rec {
              name = "polkadot-node-${version}";
              inherit version;
              repo = "polkadot";
              owner = "paritytech";
              rev = "refs/tags/${version}";
              hash = "sha256-x2IEIHxH8Hg+jFIpnPrTsqISEAZHFuXhJD+H1S+G3nk=";
              cargoSha256 = "sha256-639VMQBvDIRPlCTQVGsS8Xy3Y6nJVqO5VDkuCDQtjqg=";
            };
          # for xcmv3 release and centauri client asap they upgrade
          polkadot-node-9390 =
            let version = "v0.9.39";
            in buildPolkadotNode rec {
              name = "polkadot-node-next";
              inherit version;
              repo = "polkadot";
              owner = "paritytech";
              rev = "refs/tags/${version}";
              hash = "sha256-++aSGovKRE4+1hRoDqo6lSO4aenNrdvkVqaIXz4s0bk=";
              cargoSha256 = "sha256-RG/FvtrMCJB1BbMosSPlGJCKmIbRJT7ZUDkj1dVKWKg=";
            };

          #ASD
          polkadot-node-on-parity-kusama = polkadot-node-9360;
          polkadot-node-on-parity-polkadot = polkadot-node-9360;
          #polkadot-node-on-parity-rococo = polkadot-node-9390;
        };
    };
}
