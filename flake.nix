{
  description = "Your trusty omnibox search.";

  nixConfig = {
    extra-substituters = [ "https://friedow.cachix.org" ];
    extra-trusted-public-keys = [ "friedow.cachix.org-1:JDEaYMqNgGu+bVPOca7Zu4Cp8QDMkvQpArKuwPKa29A=" ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    home-manager.url = "github:nix-community/home-manager";
    treefmt-nix.url = "github:numtide/treefmt-nix/";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = args: import ./nix args;
}
