{ config
, ...
}:

{
  perSystem = { pkgs, lib, ... }:
    let
      rustToolchain = config.mkRustToolchain pkgs;

      rustPkgs = pkgs.rustBuilder.makePackageSet {
        inherit rustToolchain;
        packageFun = import ../Cargo.nix;
        packageOverrides = pkgs: pkgs.rustBuilder.overrides.all ++ [
          (pkgs.rustBuilder.rustLib.makeOverride {
            # FIXME: x86_64-darwin sdk version default is 10.12
            # https://github.com/NixOS/nixpkgs/issues/101229
            # damn x86_64-darwin
            name = "librocksdb-sys";
            overrideAttrs = drv: {
              propagatedBuildInputs = drv.propagatedBuildInputs or [ ] ++ [
                pkgs.zstd
              ] ++ lib.optionals pkgs.stdenv.isDarwin
                (with pkgs.darwin.apple_sdk.frameworks; [
                  CoreFoundation
                  Security
                ]);
              shellHook = drv.shellHook or "" + ''
                export MACOSX_DEPLOYMENT_TARGET=11
              '';
            };
          })
        ];
      };

      raste = rustPkgs.workspace.raste { };
    in
    {
      packages = {
        inherit raste;
        default = raste;
      };
    };
}
