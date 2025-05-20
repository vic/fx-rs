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
            help = "Build book at book/book";
            command = "mdbook build book";
          }

          {
            name = "coverage";
            help = "Test coverate at tarpaulin-report.html";
            command = "cargo tarpaulin --out Html";
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
            pkgs.cargo-tarpaulin
            pkgs.mdbook
            zigcc
          ];
      };
    };
}
