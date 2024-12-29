{ inputs, ... }:
{
  imports = [ inputs.treefmt-nix.flakeModule ];

  perSystem = _: {
    treefmt = {
      projectRootFile = ".git/config";

      programs = {
        deadnix.enable = true;
        nixfmt.enable = true;
        rustfmt.enable = true;
        statix.enable = true;
        taplo.enable = true;
        yamlfmt.enable = true;
      };
    };
  };
}
