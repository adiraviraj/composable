{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, systemCommonRust, ... }: {
    devShells = rec {
      minimal = pkgs.mkShell {
        buildInputs = with pkgs;
          [ clang nodejs python3 yarn ]
          ++ (with self'.packages; [ rust-nightly ]);
        LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath
          (with pkgs; [ stdenv.cc.cc.lib llvmPackages.libclang.lib pkgs.zlib ]);
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        PROTOC = "${pkgs.protobuf}/bin/protoc";
        ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
        NIX_PATH = "nixpkgs=${pkgs.path}";
      };

      dev = minimal.overrideAttrs (base: {
        buildInputs = base.buildInputs ++ (with pkgs;
          with self'.packages; [
            bacon
            google-cloud-sdk
            jq
            lldb
            llvmPackages_latest.bintools
            llvmPackages_latest.lld
            llvmPackages_latest.llvm
            nix-tree
            nixfmt
            openssl
            openssl.dev
            pkg-config
            qemu
            rnix-lsp
            taplo
            xorriso
            zlib.out
            nix-tree
            nixfmt
            rnix-lsp
            nodePackages.typescript
            nodePackages.typescript-language-server
            git
            git-lfs
            subwasm
            binaryen
          ]);
      });

      default = self.inputs.devenv.lib.mkShell {
        inherit pkgs;
        inputs = self.inputs;
        modules = [{
          packages = with self'.packages;
            minimal.buildInputs ++ [ centauri-configure-and-run ];

          enterShell = ''
            echo "Setting up dev environment..."
          '';
          devcontainer.enable = true;
        }];
      };

      with-helix = dev.overrideAttrs (base: {
        buildInputs = base.buildInputs ++ [ inputs'.helix.packages.default ];
      });

      xcvm = dev.overrideAttrs (base: {
        buildInputs = base.buildInputs ++ (with self'.packages; [ junod gex ]);
        shellHook = ''
          echo "junod alice key:"
          echo "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose" | junod keys add alice --recover --keyring-backend test || true
        '';
      });
    };
  };
}
