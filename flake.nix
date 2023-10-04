{
  description = "rusage flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, flake-compat }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        craneLib = crane.lib.${system};
        pkgs = import nixpkgs { inherit system; };
        inherit (pkgs) lib;
        rusage = craneLib.buildPackage {
          name = "rusage";
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          buildInputs = lib.optionals pkgs.stdenv.isDarwin [ pkgs.libiconv ];
        };
      in {
        packages = { inherit rusage; };
        defaultPackage = rusage;
        devShell = pkgs.mkShell {
          inputsFrom = [ rusage ];
          packages = with pkgs; [
            cargo-edit rustfmt
          ];
        };
      });
}
