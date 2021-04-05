{ pkgs ? import <nixpkgs> {}}:
let
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
    xlibs.libX11
    xlibs.libXcursor
    xlibs.libXi
    xlibs.libXrandr
    xlibs.libXxf86vm
    xlibs.libxcb
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
  nativeBuildInputs = [ cargo rustc pkg-config fontconfig zeromq ];
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
        tabbed -cr 2 -w "--working-directory" -x "--xembed-tcp-port" ./target/debug/alacritty --embed "" 2>&1
      }
  '';
}

