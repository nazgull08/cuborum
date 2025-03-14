{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy

    # X11
    xorg.libX11
    xorg.libXi
    xorg.libXcursor
    xorg.libXrandr
    libxkbcommon  # Для клавиатурного ввода (XKB)

    # Vulkan + NVIDIA
    mesa           # драйверы Mesa, в т.ч. для llvmpipe
    vulkan-loader  # основной Vulkan loader
    vulkan-tools   # vkcube, vulkaninfo
    nvidia-vaapi-driver  # иногда помогает при наличии nvidia
  ];

  nativeBuildInputs = [ pkgs.pkg-config ];

  shellHook = ''
    echo "Cuborum dev env loaded."

    # Пробуем заставить wgpu взять Vulkan
    export WGPU_BACKEND=VULKAN
    # Отключаем принудительный софтварный fallback
    export WGPU_FORCE_FALLBACK_ADAPTER=0
    # Пишем трассы в wgpu-trace
    export WGPU_TRACE=1

    # Пробуем логи отладочного уровня (можно уменьшить при шуме):
    export RUST_LOG=wgpu=trace,wgpu_hal=trace,info

    # Если нужно, добавляем libxkbcommon в LD_LIBRARY_PATH
    export LD_LIBRARY_PATH="${pkgs.libxkbcommon}/lib:$LD_LIBRARY_PATH"

    export LD_LIBRARY_PATH="${pkgs.vulkan-loader}/lib:$LD_LIBRARY_PATH"

    echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
    echo "WGPU_BACKEND=$WGPU_BACKEND"
    echo "RUST_LOG=$RUST_LOG"
    echo "WGPU_FORCE_FALLBACK_ADAPTER=$WGPU_FORCE_FALLBACK_ADAPTER"
    echo "WGPU_TRACE=$WGPU_TRACE"
  '';
}
