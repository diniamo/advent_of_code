with import <nixpkgs> {}; mkShellNoCC {
  packages = [
    (ghc.withPackages (hpkgs: with hpkgs; [
      raw-strings-qq
    ]))
    haskell-language-server

    hyperfine
  ];
}
