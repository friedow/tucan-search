_: {
  perSystem =
    { self', ... }:
    {
      packages = rec {
        default = centerpiece;
        inherit (self'.checks)
          centerpiece
          index-git-repositories
          ;
      };
    };
}
