{
  description = "Flake providing Coping chronicles, a multiplayer voxel RPG written in Rust.";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  inputs.nci.url = "github:yusdacra/nix-cargo-integration";
  inputs.nci.inputs.nixpkgs.follows = "nixpkgs";
  inputs.parts.url = "github:hercules-ci/flake-parts";

  outputs = inp: let
    lib = inp.nci.inputs.nixpkgs.lib;

    git = let
      sourceInfo = inp.self.sourceInfo;
      dateTimeFormat = import ./nix/dateTimeFormat.nix;
      dateTime = dateTimeFormat sourceInfo.lastModified;
      shortRev = sourceInfo.shortRev or "dirty";
    in {
      prettyRev = shortRev + "/" + dateTime;
      tag = "";
    };

    filteredSource = let
      pathsToIgnore = [
        "flake.nix"
        "flake.lock"
        "nix"
        "assets"
        "README.md"
        "CONTRIBUTING.md"
        "CHANGELOG.md"
        "CODE_OF_CONDUCT.md"
        "clippy.toml"
        ".cargo"
        ".github"
        ".gitlab"
      ];
      ignorePaths = path: type: let
        split = lib.splitString "/" path;
        actual = lib.drop 4 split;
        _path = lib.concatStringsSep "/" actual;
      in
        lib.all (n: ! (lib.hasPrefix n _path)) pathsToIgnore;
    in
      builtins.path {
        name = "coping chronicles-source";
        path = toString ./.;
        # filter out unnecessary paths
        filter = ignorePaths;
      };
  in
    inp.parts.lib.mkFlake {inputs = inp;} {
      imports = [inp.nci.flakeModule];
      systems = ["x86_64-linux"];
      perSystem = {
        config,
        pkgs,
        lib,
        ...
      }: let
        checkIfLfsIsSetup = checkFile: ''
          checkFile="${checkFile}"
          result="$(${pkgs.file}/bin/file --mime-type $checkFile)"
          if [ "$result" = "$checkFile: image/jpeg" ]; then
            echo "Git LFS seems to be setup properly."
            true
          else
            echo "
              Git Large File Storage (git-lfs) has not been set up correctly.
              Most common reasons:
                - git-lfs was not installed before cloning this repository.
                - This repository was not cloned from the primary GitLab mirror.
                - The GitHub mirror does not support LFS.
              See the book at https://book.coping chronicles.net/ for details.
              Run 'nix-shell -p git git-lfs --run \"git lfs install --local && git lfs fetch && git lfs checkout\"'
              or 'nix shell nixpkgs#git-lfs nixpkgs#git -c sh -c \"git lfs install --local && git lfs fetch && git lfs checkout\"'.
            "
            false
          fi
        '';
        assets = pkgs.runCommand "coping chronicles-assets" {} ''
          mkdir $out
          ln -sf ${./assets} $out/assets
          ${checkIfLfsIsSetup "$out/assets/voxygen/background/bg_main.jpg"}
        '';
        wrapWithAssets = old:
          pkgs.runCommand
          old.name
          {
            meta = old.meta or {};
            passthru =
              (old.passthru or {})
              // {
                unwrapped = old;
              };
            nativeBuildInputs = [pkgs.makeWrapper];
          }
          ''
            cp -rs --no-preserve=mode,ownership ${old} $out
            wrapProgram $out/bin/* \
              --set COPING CHRONICLES_ASSETS ${assets} \
              --set COPING CHRONICLES_GIT_VERSION "${git.prettyRev}" \
              --set COPING CHRONICLES_GIT_TAG "${git.tag}"
          '';
        coping chronicles-common-env = {
          # We don't add in any information here because otherwise anything
          # that depends on common will be recompiled. We will set these in
          # our wrapper instead.
          NIX_GIT_HASH = "";
          NIX_GIT_TAG = "";
          COPING CHRONICLES_USERDATA_STRATEGY = "system";
        };
        voxygenOut = config.nci.outputs."coping chronicles-voxygen";
        serverCliOut = config.nci.outputs."coping chronicles-server-cli";
      in {
        packages.coping chronicles-voxygen = wrapWithAssets voxygenOut.packages.release;
        packages.coping chronicles-voxygen-dev = wrapWithAssets voxygenOut.packages.dev;
        packages.coping chronicles-server-cli = wrapWithAssets serverCliOut.packages.release;
        packages.coping chronicles-server-cli-dev = wrapWithAssets serverCliOut.packages.dev;
        packages.default = config.packages."coping chronicles-voxygen";

        devShells.default = config.nci.outputs."coping chronicles".devShell.overrideAttrs (old: {
          shellHook = ''
            ${checkIfLfsIsSetup "$PWD/assets/voxygen/background/bg_main.jpg"}
            if [ $? -ne 0 ]; then
              exit 1
            fi
            export COPING CHRONICLES_GIT_VERSION="${git.prettyRev}"
            export COPING CHRONICLES_GIT_TAG="${git.tag}"
          '';
        });

        nci.projects."coping chronicles" = {
          export = false;
          path = ./.;
        };
        nci.crates."coping chronicles-server-cli" = {
          profiles = {
            release.features = ["default-publish"];
            release.runTests = false;
            dev.features = ["default-publish"];
            dev.runTests = false;
          };
          drvConfig = {
            mkDerivation = {
              src = filteredSource;
            };
            env = coping chronicles-common-env;
          };
        };
        nci.crates."coping chronicles-voxygen" = {
          profiles = {
            release.features = ["default-publish"];
            release.runTests = false;
            dev.features = ["default-publish"];
            dev.runTests = false;
          };
          runtimeLibs = with pkgs; [
            xorg.libX11
            xorg.libXi
            xorg.libxcb
            xorg.libXcursor
            xorg.libXrandr
            libxkbcommon
            shaderc.lib
            udev
            alsa-lib
            vulkan-loader
            stdenv.cc.cc.lib
          ];
          depsDrvConfig = {
            env =
              coping chronicles-common-env
              // {
                SHADERC_LIB_DIR = "${pkgs.shaderc.lib}/lib";
                COPING CHRONICLES_ASSETS = "${assets}";
              };
            mkDerivation = {
              buildInputs = with pkgs; [
                alsa-lib
                libxkbcommon
                udev
                xorg.libxcb

                fontconfig
              ];
              nativeBuildInputs = with pkgs; [
                python3
                pkg-config
                cmake
                gnumake
              ];
            };
          };
          drvConfig = let
            depsConf = config.nci.crates."coping chronicles-voxygen".depsDrvConfig;
          in {
            env =
              depsConf.env
              // {
                dontUseCmakeConfigure = true;
              };
            mkDerivation =
              depsConf.mkDerivation
              // {
                src = filteredSource;
                preConfigure = ''
                  substituteInPlace voxygen/src/audio/soundcache.rs \
                    --replace \
                    "../../../assets/voxygen/audio/null.ogg" \
                    "${./assets/voxygen/audio/null.ogg}"
                '';
              };
          };
        };
      };
    };
}
