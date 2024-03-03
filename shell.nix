with import <unstable> {};
mkShell {
  name = "zerotier-controller";
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
    pkg-config
    perl
    openssl
  ];
}
