{ inputs, ... }:

{
  perSystem =
    {
      system,
      ...
    }:
    let
      crossBuild =
        {
          localSystem,
          target,
        }:
        let
          inherit (import ../nix/lib) mkCrossCraneLib mkCrossPkgs mkFormula;
          pkgs = mkCrossPkgs {
            inherit localSystem target;
            inherit (inputs) nixpkgs rust-overlay;
          };
          craneLib = mkCrossCraneLib {
            inherit (inputs) crane;
            inherit pkgs target;
          };
          formula = mkFormula {
            inherit pkgs craneLib;
          };
        in
        craneLib.buildPackage (
          formula
          // {
            CARGO_BUILD_TARGET = target;
            CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
          }
        );
      crossBuildPackages =
        { name }:
        {
          "${name}-static-x86_64" = crossBuild {
            localSystem = system;
            target = "x86_64-unknown-linux-musl";
          };
          "${name}-static-aarch64" = crossBuild {
            localSystem = system;
            target = "aarch64-unknown-linux-musl";
          };
        };
    in
    {
      packages = crossBuildPackages { name = "mail2phone"; };
    };
}
