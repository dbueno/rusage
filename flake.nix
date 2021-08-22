{
  description = "rusage flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
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
            CFLAGS=-Wall
            make rusage.o
            $CC $CFLAGS -o crusage rusage.o
          '';
          installPhase = ''
            mkdir -p $out/bin
            mkdir -p $out/etc

            cp crusage $out/etc
            cp rusage.sh $out/bin/rusage

            substituteInPlace $out/bin/rusage \
              --replace 'crusage' "$out/etc/crusage"
          '';
        };
      in {
        packages = { inherit rusage; };
        defaultPackage = rusage;
      });
}
