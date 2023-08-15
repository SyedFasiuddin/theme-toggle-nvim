---Check if the user provided config is as expected
---@param args table
---@return boolean
local function handle_user_config(args)
    if args == nil then
        vim.notify("theme-toggle-nvim: Expected an argument in setup function",
        vim.log.levels.ERROR)
        return false
    end

    if args.colorscheme == nil then
        vim.notify("theme-toggle-nvim: Expected `colorscheme` table",
        vim.log.levels.ERROR)
        return false
    end

    if args.colorscheme.light == nil then
        vim.notify("theme-toggle-nvim: Expected `colorscheme.light` a colorscheme for light mode",
        vim.log.levels.ERROR)
        return false
    end

    if args.colorscheme.dark == nil then
        vim.notify("theme-toggle-nvim: Expected `colorscheme.dark` a colorscheme for dark mode",
        vim.log.levels.ERROR)
        return false
    end

    return true
end

---Initialize the plugin with light and dark colorschemes
---
---@param args table
---@usage [[
--- require("theme-toggle-nvim").setup({
---     colorscheme = {
---         light = "onedark",
---         dark = "gruvbox",
---     }
--- })
---@usage ]]
local function setup(args)
    if not handle_user_config(args) then return end
end

return setup
