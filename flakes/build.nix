{ inputs, ... }:

{
  perSystem =
    {
      pkgs,
      ...
    }:
    let
      inherit (import ../nix/lib) mkFormula;
      craneLib = inputs.crane.mkLib pkgs;

      # Common arguments for building
      formula = mkFormula {
        inherit pkgs craneLib;
      };

      mail2phone = craneLib.buildPackage (
        formula
        // {
          cargoArtifacts = craneLib.buildDepsOnly formula;
        }
      );
    in
    {
      packages.mail2phone = mail2phone;
      packages.default = mail2phone;
    };
}
