with import <nixpkgs> {}; mkShellNoCC {
  packages = [
    rustc
    cargo
  ];
}
