{pkgs, ...}: let
  baseImage = pkgs.ociTools.pullImage {
    imageName = "alpine";
    tag = "latest";
  };
in
  pkgs.dockerTools.buildImage {
    name = "try-out-api";

    fromImage = baseImage;

    copyToRoot = pkgs.buildEnv {
      name = "app-env";
      paths = [
        (pkgs.callPackage ./default.nix {})
      ];
    };

    config = {
      Cmd = ["/bin/try-out-api"];
      WorkingDir = "/bin";
    };
  }
