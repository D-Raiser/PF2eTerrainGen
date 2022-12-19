# PF2eTerrainGen
Randomly generates terrain hex maps for use in Pathfinder 2e campaigns.
Used as a project to learn Rust (& SDL2).

# Requirements:
 - SDL2
 - SDL2 GFX
 - SDL2 Image

 On Windows you need to put the `SDL2.dll` and `SDL2_gfx.dll` and `SDL2_image.dll` into this directory as well in order to run the application:
  - sdl2: https://github.com/libsdl-org/SDL/releases (VC Release)
  - sdl2-image: https://github.com/libsdl-org/SDL_image/releases (VC Release)
  - sdl2-gfx `./vcpkg.exe install sdl2-gfx --triplet x64-windows` (requires `vcpkg`)