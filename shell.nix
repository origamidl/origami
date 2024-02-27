{pkgs ? import <nixpkgs> {}}:
with pkgs;
  mkShell {
    buildInputs = [
      cargo
      rustc
      gnum4 # Required for GMP/rug. Can be removed in case we decide against GMP
    ];

    shellHook = ''
      # ...
    '';
  }
