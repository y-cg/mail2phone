{ inputs, ... }:

{
  perSystem =
    {
      system,
      ...
    }:
    let
      overlays = [ (import inputs.rust-overlay) ];
      pkgs = import inputs.nixpkgs { inherit system overlays; };

      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      isStaticSupported = builtins.elem system supportedSystems;

      targetTriples = {
        "x86_64-linux" = "x86_64-unknown-linux-musl";
        "aarch64-linux" = "aarch64-unknown-linux-musl";
      };
      targetTriple = targetTriples.${system} or (throw "Unsupported system: ${system}");

      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
        p:
        p.rust-bin.stable.latest.default.override {
          targets = [
            targetTriple
          ];
        }
      );

      mail2phone-static = craneLib.buildPackage {
        pname = "mail2phone-static";
        src = craneLib.cleanCargoSource ../.;
        strictDeps = true;
        CARGO_BUILD_TARGET = targetTriple;
        CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
      };
    in
    {
      packages = if isStaticSupported then { mail2phone-static = mail2phone-static; } else { };
      checks = if isStaticSupported then { mail2phone-static = mail2phone-static; } else { };
    };
}
