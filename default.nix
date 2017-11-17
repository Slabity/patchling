with import <nixpkgs> { };

let
  xLibPath = lib.makeLibraryPath (with xorg; [libX11 libXcursor libXxf86vm libXi libXrandr xinput zlib]);
in
stdenv.mkDerivation rec {
  name = "patchling";

  shellHook = ''
    export LD_LIBRARY_PATH=/run/opengl-driver/lib/:${xLibPath}
  '';
}
