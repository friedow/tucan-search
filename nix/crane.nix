{
  self,
  lib,
  pkgs,
  makeWrapper,
  pkgconf,
  dbus,
}:
let
  cargoTOML = builtins.fromTOML (builtins.readFile (self + "/Cargo.toml"));
  inherit (cargoTOML.workspace.package) version;
  pname = "centerpiece";
  meta = import ./meta.nix { inherit lib; };
  craneLib = self.inputs.crane.mkLib pkgs;
  commonArgs = {
    nativeBuildInputs = [
      makeWrapper
      # wifi plugin
      # cargo networkmanager dependency
      pkgconf
      dbus
    ];
    buildInputs = [ dbus ];
    inherit
      meta
      version
      pname
      ;
    src =
      let
        fontFilter = path: _type: builtins.match ".*ttf$" path != null;
        configFilter = path: _type: builtins.match ".*config.yml$" path != null;
        assetOrCargo =
          path: type:
          (configFilter path type) || (fontFilter path type) || (craneLib.filterCargoSources path type);
      in
      lib.cleanSourceWith {
        src = craneLib.path ../.;
        filter = assetOrCargo;
      };
  };
  cargoArtifacts = craneLib.buildDepsOnly commonArgs;
  cargoClippy = craneLib.cargoClippy (
    commonArgs
    // {
      inherit cargoArtifacts;
      cargoClippyExtraArgs = "--all-targets --all-features";
    }
  );
  cargoTest = craneLib.cargoNextest (commonArgs // { inherit cargoArtifacts; });
in
{
  centerpiece = craneLib.buildPackage (
    commonArgs
    // {
      env = import ./env.nix { inherit self; };
      postFixup = pkgs.lib.optional pkgs.stdenv.isLinux ''
        rpath=$(patchelf --print-rpath $out/bin/${pname})
        patchelf --set-rpath "$rpath:${pkgs.callPackage ./lib.nix { }}" $out/bin/${pname}
      '';
      doCheck = false;
      inherit
        cargoArtifacts
        ;
    }
  );
  index-git-repositories = craneLib.buildPackage (
    commonArgs
    // {
      cargoExtraArgs = "-p index-git-repositories";
      pname = "index-git-repositories";
      doCheck = false;
      inherit
        cargoArtifacts
        ;
    }
  );
  inherit
    cargoClippy
    cargoArtifacts
    cargoTest
    ;
}
