{
  pkgs,
  ...
}:
pkgs.lib.makeLibraryPath [
  pkgs.wayland
  pkgs.libxkbcommon
  pkgs.vulkan-loader
  pkgs.libGL
]
