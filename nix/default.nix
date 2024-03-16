{ inputs
, ...
}:

{
  imports = [
    inputs.flake-root.flakeModule

    ./devshell.nix
    ./toolchain.nix
    ./formatter.nix
    ./package.nix
  ];
}
