{ self, ... }:
{
  perSystem =
    { pkgs, ... }:
    {
      checks = {
        inherit ((pkgs.callPackage ./crane.nix { inherit self; }))
          centerpiece
          index-git-repositories
          cargoArtifacts
          cargoClippy
          cargoTest
          ;
      };
    };
}
