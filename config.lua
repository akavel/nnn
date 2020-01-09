local mana = require 'mana'
-- TODO: make handlers pluggable external processes, with a configurable
-- command to run each one (managed through popen and json, one line = one
-- command)
mana.handle('c', require'mana.winfs'.fordisk'c')
-- TODO: add refreshenv support copied from chocolatey
mana.handle('path', require 'mana.winpath')
-- TODO: mana.handle('0install', require 'mana.zeroinstall')
-- TODO: mana.handle('choco', require 'mana.chocolatey')
mana.handle('home', require 'mana.winhome')

require 'vimrc'

-- One-line scripts (~aliases)
local oneliners = {
  gd = "git diff";
  gds = "git diff --staged";
  gf = "git fetch --all";
  gl = "glg --all";
  glg = "git log --graph \"--pretty=format:%%Cred%%h%%Creset -%%C(yellow)%%d%%Creset %%s %%Cgreen(%%cr %%cd) %%C(cyan)%%an%%Creset\" \"--date=format:'%%d.%%m\" --abbrev-commit";
  gs = "git status";
  -- Show git branches in ~ "most recently used" order
  ["git-bs"] = "git branch --sort=-committerdate";
}
-- Render oneliners to full files
for name, text in pairs(oneliners) do
  -- TODO: make this work correctly on Linux/Mac
  mana.wanted["c/bin/" .. name .. ".bat"] = "@" .. text .. " %*"
end

mana.wanted["path/c/bin"] = ""
mana.wanted["path/C/Users/Mateusz/.nimble/bin"] = ""

-- Execute --
local winfs = require 'mana.winfs'
mana.osmkdirp = winfs.mkdirp
mana.exec_with(function(path)
  return winfs.ospath(table.concat({'c/prog/shadow', path}, '/'))
end)

