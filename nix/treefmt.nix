{ inputs, ... }:
{
  imports = [ inputs.treefmt-nix.flakeModule ];
  perSystem =
    { pkgs, ... }:
    {
      treefmt = {
        projectRootFile = "flake.nix";
        programs.nixfmt.enable = true;
        programs.nixfmt.excludes = [ ".direnv" ];
        programs.deadnix.enable = true;
        programs.mdformat.enable = true;
        # programs.mdformat.excludes = [ "docs/**" ];
        programs.yamlfmt.enable = true;
        programs.rustfmt.enable = true;
        programs.rustfmt.package = inputs.self.packages.${pkgs.system}.rust-toolchain.rustfmt;
      };
    };
}
