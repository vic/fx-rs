{ inputs, ... }:
{
  systems = import inputs.systems;
  imports = [
    ./devshell.nix
    ./treefmt.nix
  ];
}
