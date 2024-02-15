with import <unstable> {};
mkShell {
  name = "ztui";
  nativeBuildInputs = with pkgs; [
    pkg-config
    perl
    openssl
  ];

  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
