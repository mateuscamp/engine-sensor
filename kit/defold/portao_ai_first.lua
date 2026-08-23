-- Passe os arquivos Lua puros como argumentos; o shell pode expandir globs.
-- Exemplo: luajit .sara/defold/portao_ai_first.lua modules/*.lua
local proibidos = {
  "go%.", "gui%.", "msg%.", "timer%.", "factory%.", "collectionfactory%.",
  "math%.random", "socket%.gettime",
}

if #arg == 0 then
  io.stderr:write("informe ao menos um arquivo Lua puro para o portao\n")
  os.exit(2)
end

local falhas = 0
for i = 1, #arg do
  local caminho = arg[i]
  local arquivo, erro = io.open(caminho, "r")
  if not arquivo then
    io.stderr:write(string.format("nao abriu %s: %s\n", caminho, erro or "erro desconhecido"))
    falhas = falhas + 1
  else
    local fonte = arquivo:read("*a")
    arquivo:close()
    fonte = fonte:gsub("%-%-[^\n]*", "")
    for _, proibido in ipairs(proibidos) do
      if fonte:find(proibido) then
        io.stderr:write(string.format("%s usa %s dentro da fronteira pura\n", caminho, proibido))
        falhas = falhas + 1
      end
    end
  end
end

io.write(string.format('{"portao":"ai-first","arquivos":%d,"falhas":%d}\n', #arg, falhas))
os.exit(falhas == 0 and 0 or 1)
