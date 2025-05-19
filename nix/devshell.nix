{ inputs, ... }:
{
  imports = [ inputs.devshell.flakeModule ];

  perSystem =
    { pkgs, ... }:
    {
      devshells.default = {
        commands = [
          {
            name = "book";
            command = "mdbook build book";
          }
        ];

        devshell.packages =
          let
            zigcc = pkgs.writeShellApplication {
              name = "cc";
              text = ''exec ${pkgs.zig}/bin/zig cc "$@"'';
            };
          in
          [
            pkgs.cargo
            pkgs.rust-analyzer
            pkgs.mdbook
            zigcc
          ];
      };
    };
}
