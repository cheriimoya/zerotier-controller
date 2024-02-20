with import <unstable> {};
mkShell {
  name = "zerotier-controller";
  nativeBuildInputs = with pkgs; [
    pkg-config
    perl
    openssl
  ];
}
