# theme-toggle-nvim

> under active development, not usable yet

The inspiration to make this thing came from another plugin named
[dark-notify](https://github.com/cormacrelf/dark-notify). What does that plugin
do? It toggles the theme for neovim when display mode changes but it works only
on MacOS.

How that plugin works is, it calls a GUI app, this GUI app doesn't actually show
the GUI but is used only to know the display mode initially and to handle the
event when the display mode changes.

The GUI is written in rust and uses MacOS specific libraries (cocoa and objc) to
check the display mode (and also has ability to do other thing on top of it)

This plugin tries to do the same but in a cross platform way.

## Installation using packer

```lua
use({
    "SyedFasiuddin/theme-toggle-nvim",
    config = function ()
        require("theme-toggle-nvim").setup({
            colorscheme = {
                light = "onedark",
                dark = "gruvbox",
            }
        })
    end
})
```
