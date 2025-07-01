let 
    pkgs = import <nixpkgs> { };
    deps = with pkgs; [
        rustc
        cargo

        libGL
        xorg.libX11
        xorg.libXi
        libxkbcommon

        alsa-lib.dev
    ];
in pkgs.mkShell {
  buildInputs = deps;
  nativeBuildInputs = with pkgs; [ pkg-config ];
  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath (deps);
}
