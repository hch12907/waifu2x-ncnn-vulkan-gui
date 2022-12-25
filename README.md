# waifu2x-ncnn-vulkan-gui

A Win32 GUI for [waifu2x-ncnn-vulkan](https://github.com/nihui/waifu2x-ncnn-vulkan).
I did this mostly because why not? In the future, I plan to add a GTK or Qt-based
implementation for Linux and MacOS. (I initially *was* trying to do that but Rust
and Qt doesn't mesh well together, and GTK4 on Windows is just yuck.)

## Usage

1. `cargo build` the project.
2. Copy the resulting `waifu2x-ncnn-vulkan-gui.exe` into `waifu2x-ncnn-vulkan`'s
   main directory.
3. Rename `waifu2x-ncnn-vulkan.exe` into `waifu2x-ncnn-vulkan-cli.exe`.
4. Voila! Just run `waifu2x-ncnn-vulkan-gui.exe` and you should see a GUI popping up.
