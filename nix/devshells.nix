{ self, ... }:
{
  perSystem =
    { pkgs, self', ... }:
    {
      devShells = {
        default = pkgs.mkShellNoCC {
          name = "centerpiece";
          inputsFrom = [ self'.packages.default ];
          packages = [
            pkgs.clippy
            self'.formatter.outPath
          ];
          env = import ./env.nix { inherit self; };
          LD_LIBRARY_PATH = pkgs.callPackage ./lib.nix { };
        };
      };
    };
}
