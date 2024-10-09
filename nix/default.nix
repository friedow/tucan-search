{ flake-parts, ... }@inputs:
flake-parts.lib.mkFlake { inherit inputs; } {
  systems = [
    "x86_64-linux"
    "aarch64-linux"
  ];

  imports = [
    ./devshells.nix
    ./formatter.nix
    ./packages.nix
    ./checks.nix
  ];

}
