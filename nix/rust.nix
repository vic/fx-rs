{ inputs, ... }:
{
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
      packages.rust-toolchain = nightly // {
        inherit (fenix.latest) rustfmt;
      };
    };
}
