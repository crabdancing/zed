{
  lib,
  nixfmt-tree,
  runCommand,
}:

let
  inherit (lib.fileset) toSource fileFilter;
  filesWithExtension =
    ext:
    toSource {
      root = ../../.;
      fileset = fileFilter (file: file.hasExt ext) ../../.;
    };
in

{
  format-nix = runCommand "format-nix" { } ''
    ${lib.getExe nixfmt-tree} --ci ${filesWithExtension "nix"}
    touch $out
  '';
}
