{
  description = "SRES SNES emulator";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, rust-overlay, ... }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forEachSystem = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forEachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          inherit (pkgs) lib;
          inherit (pkgs.stdenv.hostPlatform) isLinux;

          # Channel, components, and the wasm32 target come from rust-toolchain.toml.
          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          # eframe dlopens X11, GL, and Vulkan. cpal links ALSA. winit needs xkbcommon.
          linuxRuntimeLibs = with pkgs; [
            alsa-lib
            libGL
            libx11
            libxcb
            libxcursor
            libxi
            libxkbcommon
            libxrandr
            libxrender
            vulkan-loader
            wayland
          ];

          mkDevShell =
            {
              extraLibs ? [ ],
              extraHook ? "",
            }:
            pkgs.mkShell {
              packages = [
                rustToolchain
                pkgs.binaryen # wasm-opt for trunk
                pkgs.cargo-nextest
                pkgs.trunk
              ]
              ++ lib.optionals isLinux [
                pkgs.pkg-config
                pkgs.xvfb-run
              ];

              buildInputs = lib.optionals isLinux linuxRuntimeLibs;

              shellHook =
                lib.optionalString isLinux ''
                  export LD_LIBRARY_PATH="${lib.makeLibraryPath (linuxRuntimeLibs ++ extraLibs)}''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
                ''
                + extraHook;
            };
        in
        {
          default = mkDevShell { };

          # For GPU-less Linux CI runners: egui_kittest renders through Mesa llvmpipe (GL) and lavapipe (Vulkan).
          ci = mkDevShell (
            lib.optionalAttrs isLinux {
              extraLibs = [ pkgs.mesa ];
              extraHook = ''
                export LIBGL_ALWAYS_SOFTWARE=1
                export WGPU_BACKEND=gl,vulkan
                export __EGL_VENDOR_LIBRARY_FILENAMES=${pkgs.mesa}/share/glvnd/egl_vendor.d/50_mesa.json
                export VK_DRIVER_FILES=${pkgs.mesa}/share/vulkan/icd.d/lvp_icd.${pkgs.stdenv.hostPlatform.uname.processor}.json
              '';
            }
          );
        }
      );
    };
}
