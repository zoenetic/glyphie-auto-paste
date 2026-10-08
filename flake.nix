{
  description = "Glyphie development shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          # Loaded at runtime by winit and wgpu, so they must be on the library path.
          runtimeLibs = with pkgs; [
            wayland
            libxkbcommon
            libGL
            vulkan-loader
          ];
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              rustc
              clippy
              rustfmt
              just
              pkg-config
              libdrm
              wayland
              libxkbcommon
              wl-clipboard
              desktop-file-utils
              appstream
            ];
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath runtimeLibs;
          };
        }
      );
    };
}
