{
  perSystem =
    {
      pkgs,
      system,
      self',
      ...
    }:
    let
      # Only support Docker images for Linux architectures
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      isDockerSupported = builtins.elem system supportedSystems;

      dockerImage = pkgs.dockerTools.buildImage {
        name = "mail2phone-image";
        tag = "latest";
        copyToRoot = [
          self'.packages.mail2phone-static
        ];
        config = {
          Entrypoint = [ "${self'.packages.mail2phone-static}/bin/mail2phone" ];
          Env = [
            "RUST_LOG=info"
          ];
        };
      };
    in
    {
      packages = if isDockerSupported then { mail2phone-image = dockerImage; } else { };
      checks = if isDockerSupported then { mail2phone-image = dockerImage; } else { };
    };
}
