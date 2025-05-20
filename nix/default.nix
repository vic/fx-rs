{ inputs, ... }:
{
  systems = import inputs.systems;
  imports = [
    ./rust.nix
    ./devshell.nix
    ./treefmt.nix
  ];
}
