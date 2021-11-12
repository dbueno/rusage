{
  description = "rusage flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, flake-compat }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rusage = pkgs.stdenv.mkDerivation {
          name = "rusage";
          src =
            with builtins; filterSource
            (path: _:
            !elem (baseNameOf path) [ ".git" "result" ]) ./.;
          buildPhase = ''
            set -x
            export CFLAGS=-Wall
            make crusage
          '';
          installPhase = ''
            mkdir -p $out/bin
            mkdir -p $out/etc

            cp crusage $out/etc
            cp rusage $out/bin/rusage

            substituteInPlace $out/bin/rusage \
              --replace 'crusage' "$out/etc/crusage"
          '';
        };
      in {
        packages = { inherit rusage; };
        defaultPackage = rusage;
        devShell = pkgs.mkShell {
          inputsFrom = [ rusage ];
          CFLAGS = "-Wall -Werror";
        };
      });
}
