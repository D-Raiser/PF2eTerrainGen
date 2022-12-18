# PF2eTerrainGen
Randomly generates terrain hex maps for use in Pathfinder 2e campaigns.
Used as a project to learn Rust (& SDL2).

# Requirements:
 - SDL2
 - SDL2 GFX

 On Windows you need to put the `SDL2.dll` and `SDL2_gfx.dll` into this directory as well in order to run the application. The former can simply be downloaded the latter can most easily be installed via vcpkg: `.\vcpkg.exe install sdl2-gfx --triplet x64-windows`.