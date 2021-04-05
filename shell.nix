{}:
let
  rust-overlay = import (fetchTarball "https://github.com/oxalica/rust-overlay/archive/3eed08a074cd2000884a69d448d70da2843f7103.tar.gz");
  pkgs = import <nixpkgs> {
    overlays = [rust-overlay];
  };
  lib = pkgs.lib;
  rpathLibs = with pkgs; [
    expat
    fontconfig
    freetype
    libGL
    wayland
    libxkbcommon
    zeromq
  ] ++ [
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
    xorg.libXxf86vm
    xorg.libxcb
  ];
  patched-tabbed = pkgs.tabbed.overrideAttrs (old: {
    name = "tabbed-20180310-patched";
    src = pkgs.nix-gitignore.gitignoreSource [] ~/GIT/tabbed;
    # src = pkgs.fetchFromGitHub {
    #   owner = "SCOTT-HAMILTON";
    #   repo = "tabbed";
    #   rev = "46f06d663e12287afe25b5fa4a312e5de90fec24";
    #   sha256 = "1asqp015l7gsghqdx5isjjfjmz4ijxq7f4gxh6ivczcp850h0rz6";
    # };
    buildInputs = with pkgs; (old.buildInputs or []) ++ [ libbsd zeromq ];
    makeFlags = [
      "PREFIX=$(out)"
    ];
  });
in
with pkgs; mkShell {
  nativeBuildInputs = [
      rust-bin.stable."1.77.2".default
      clippy
      pkg-config
      fontconfig
      zeromq
    ];

  buildInputs = rpathLibs;
  propagatedBuildInputs = [
    libGL
    patched-tabbed
    rustfmt
  ];
  shellHook = ''
      fix_alacritty() {
        patchelf --set-rpath "${lib.makeLibraryPath rpathLibs}" ./target/debug/alacritty
      }
      run() {
        tabbed -Dcr 2 -w "--working-directory" -x "--xembed-tcp-port" ./target/debug/alacritty --embed "" 2>&1
      }
      run_silent() {
        tabbed -cr 2 -w "--working-directory" -x "--xembed-tcp-port" ./target/debug/alacritty --embed "" 2>&1
      }
  '';
}
