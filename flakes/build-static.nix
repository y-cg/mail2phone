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
          name,
        }:
        let
          pkgs = import inputs.nixpkgs {
            overlays = [ (import inputs.rust-overlay) ];
            inherit localSystem;
            crossSystem = {
              config = target;
              # it seems that only when enable this, the aarch64 build will be truly static
              useLLVM = true;
            };
          };

          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
            p:
            p.rust-bin.stable.latest.default.override {
              targets = [
                target
              ];
            }
          );
        in
        craneLib.buildPackage {
          pname = "${name}-static";
          src = craneLib.cleanCargoSource ../.;
          strictDeps = true;
          CARGO_BUILD_TARGET = target;
          CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
        };
      crossBuildPackages =
        { name }:
        {
          "${name}-static-x86_64" = crossBuild {
            inherit name;
            localSystem = system;
            target = "x86_64-unknown-linux-musl";
          };
          "${name}-static-aarch64" = crossBuild {
            inherit name;
            localSystem = system;
            target = "aarch64-unknown-linux-musl";
          };
        };
    in
    {
      packages = crossBuildPackages { name = "mail2phone"; };
    };
}
