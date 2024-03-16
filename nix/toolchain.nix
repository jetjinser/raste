{ inputs
, lib
, ...
}:

{
  options = {
    mkRustToolchain = lib.mkOption {
      type = with lib.types; functionTo unspecified;
      default = pkgs:
        let
          inherit (pkgs.rust-bin) fromRustupToolchainFile;
        in
        fromRustupToolchainFile ../rust-toolchain.toml;
    };
  };

  config = {
    perSystem = { pkgs, system, ... }: {
      _module.args.pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [
          (import inputs.rust-overlay)
          inputs.cargo2nix.overlays.default
        ];
      };
    };
  };
}
