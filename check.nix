{ nixosModule, pkgs }:
pkgs.nixosTest {
  name = "centerpiece";

  nodes.machine = { ... }: {
    imports = [
      nixosModule
      {
        programs.centerpiece = {
          enable = true;
          services.index-git-repositories = {
            enable = true;
            interval = "hourly";
          };
        };
      }
    ];
  };

  testScript = ''
    start_all()
    # with subtest("wait for indexer unit"):
    #   machine.wait_for_unit("index-git-repositories-service.service")
  '';
}
