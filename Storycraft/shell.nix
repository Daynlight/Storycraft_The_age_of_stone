{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.pkg-config

    pkgs.wayland
    pkgs.libxkbcommon
    pkgs.mesa

    pkgs.alsa-lib
    pkgs.systemd

    pkgs.vulkan-loader
    pkgs.vulkan-tools
  ];

  shellHook = ''
    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
      pkgs.mesa
      pkgs.vulkan-loader
      pkgs.libxkbcommon
    ]}:$LD_LIBRARY_PATH

    export WGPU_BACKEND=vulkan
  '';
}