{ inputs, ... }:
{
  systems = import inputs.systems;
  imports = [
    inputs.devshell.flakeModule
    inputs.treefmt-nix.flakeModule
    ./treefmt.nix
  ];
}
