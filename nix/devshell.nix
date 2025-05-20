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

          {
            name = "fmt";
            help = "Format all files";
            command = "nix fmt";
          }

          {
            name = "tests";
            help = "Run project tests";
            command = "cargo test";
          }

          { package = pkgs.cargo; }
          { package = pkgs.mdbook; }
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
            pkgs.rustfmt
            pkgs.rust-analyzer
            pkgs.cargo-tarpaulin
            pkgs.mdbook
            zigcc
          ];
      };
    };
}
