{ inputs, ... }:

{
  perSystem =
    {
      pkgs,
      ...
    }:
    let
      craneLib = inputs.crane.mkLib pkgs;

      # Common arguments for building
      commonArgs = {
        src = craneLib.cleanCargoSource ../.;
        strictDeps = true;
        buildInputs =
          [
            # Add extra build inputs if needed
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
      };

      mail2phone = craneLib.buildPackage (
        commonArgs
        // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        }
      );
    in
    {
      packages.mail2phone = mail2phone;
      packages.default = mail2phone;
    };
}
