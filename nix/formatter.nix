{ inputs
, ...
}:

{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];

  perSystem = { config, pkgs, ... }: {
    treefmt.config = {
      inherit (config.flake-root) projectRootFile;
      package = pkgs.treefmt;

      programs = {
        # nix
        nixpkgs-fmt.enable = true;
        deadnix.enable = true;
        statix.enable = true;

        # rust
        rustfmt.enable = true;
      };

      settings.formatter = {
        nixpkgs-fmt.excludes = [ "Cargo.nix" ];
        deadnix.excludes = [ "Cargo.nix" ];
        statix.excludes = [ "Cargo.nix" ];
      };
    };
  };
}
