{ inputs, ... }:
{
  imports = [ inputs.devshell.flakeModule ];

  perSystem =
    { pkgs, ... }:
    let
      fenix = inputs.fenix.packages.${pkgs.system};
      nightly = fenix.latest.withComponents [
        "cargo"
        "rust-src"
        "rustc"
        "rustfmt"
        "rust-analyzer"
      ];
    in
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

          { package = pkgs.mdbook; }
        ];

        devshell.packages = [
          nightly
          pkgs.cargo-tarpaulin
          pkgs.mdbook
          pkgs.gcc
        ];
      };
    };
}
