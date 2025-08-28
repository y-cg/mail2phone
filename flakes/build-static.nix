{ inputs, ... }:

{
  perSystem =
    {
      system,
      ...
    }:
    let
      inherit (import ../nix/lib) crossBuild;
      crossBuild' =
        {
          target,
        }:
        crossBuild {
          inherit (inputs) nixpkgs rust-overlay crane;
          localSystem = system;
          inherit target;
        };

      crossBuildPackages =
        { name }:
        {
          "${name}-static-x86_64" = crossBuild' {
            target = "x86_64-unknown-linux-musl";
          };
          "${name}-static-aarch64" = crossBuild' {
            target = "aarch64-unknown-linux-musl";
          };
        };
    in
    {
      packages = crossBuildPackages { name = "mail2phone"; };
    };
}
