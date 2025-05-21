{ inputs, ... }:
{
  perSystem =
    { pkgs, ... }:
    let
      fenix = inputs.fenix.packages.${pkgs.system};
      channel = fenix.stable;
      toolchain = channel.withComponents [
        "cargo"
        "rust-src"
        "rustc"
        "rustfmt"
        "rust-analyzer"
      ];
    in
    {
      packages.rust-toolchain = toolchain // {
        inherit (channel) rustfmt;
      };
    };
}
