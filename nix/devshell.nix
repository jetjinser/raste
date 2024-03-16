{ inputs
, lib
, config
, ...
}:

{
  imports = [
    inputs.devshell.flakeModule
  ];

  perSystem = { pkgs, ... }: {
    devshells.default =
      {
        motd = ''
          {italic}{99}👾 Dev in Rust with Nix 🦀{reset}
          $(type -p menu &>/dev/null && menu)
        '';
        name = "RasteDev";

        env =
          let
            inherit (pkgs.stdenv) isDarwin;
          in
          lib.mkMerge [
            (lib.mkIf isDarwin [{ name = "MACOSX_DEPLOYMENT_TARGET"; value = 11; }])
          ];

        commands =
          let
            category = "dev";
            nom = lib.getExe pkgs.nix-output-monitor;
          in
          [
            {
              inherit category;
              name = "fmt";
              help = "format the current flake project";
              command = "nix fmt";
            }
            {
              inherit category;
              name = "r";
              help = "run the project offline";
              command = ''
                nix run .#"''${1:-default}" --offline
              '';
            }
            {
              inherit category;
              name = "or";
              help = "run the project online";
              command = ''
                nix run .#"''${1:-default}"
              '';
            }
            {
              inherit category;
              name = "b";
              help = "build the project";
              command = ''
                ${nom} build .#"''${1:-default}"
              '';
            }
            {
              inherit category;
              name = "upc";
              help = "update Cargo.nix using cargo2nix";
              command = "nix run github:cargo2nix/cargo2nix";
            }
          ];

        packages = with pkgs; [
          (config.mkRustToolchain pkgs)

          zstd
        ];
      };
  };
}
