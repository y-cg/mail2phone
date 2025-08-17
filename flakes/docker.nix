{
  perSystem =
    {
      pkgs,
      self',
      ...
    }:
    let
      mkDockerImage =
        { arch }:
        pkgs.dockerTools.buildImage {
          name = "mail2phone-image";
          tag = "latest";
          copyToRoot = [
            self'.packages."mail2phone-static-${arch}"
          ];
          config = {
            Entrypoint = [
              "/bin/mail2phone"
              "-c"
              "/config/config.toml"
            ];
            Env = [
              "RUST_LOG=info"
            ];
          };
        };
      mkDockerPackages =
        { name }:
        {
          "${name}-image-x86_64" = mkDockerImage { arch = "x86_64"; };
          "${name}-image-aarch64" = mkDockerImage { arch = "aarch64"; };
        };
    in
    {
      packages = mkDockerPackages { name = "mail2phone"; };
    };
}
