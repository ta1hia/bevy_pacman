with import <nixpkgs> {
  overlays = map (uri: import (fetchTarball uri)) [
    https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
  ];
};

stdenv.mkDerivation {
  name = "rust-bevy-wasm";
  buildInputs = [
    cargo-web
    (latest.rustChannels.stable.rust.override {
      targets = ["wasm32-unknown-unknown"];
    })
    alsaLib
    cmake
    freetype
    libudev
    lutris
    expat
    openssl
    pkgconfig
    vulkan-headers
    vulkan-loader
    vulkan-tools
    vulkan-validation-layers
    xlibs.libX11
  ];
  APPEND_LIBRARY_PATH = stdenv.lib.makeLibraryPath [
    vulkan-loader
    xlibs.libXcursor
    xlibs.libXi
    xlibs.libXrandr
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:$APPEND_LIBRARY_PATH"
    export RUSTFLAGS="-C target-cpu=native"
  '';
}
