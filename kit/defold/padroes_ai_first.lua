-- Modelos pequenos: adapte os nomes ao domínio e mantenha a engine neste adapter.
local bit = require("bit")
local M = {}

function M.novo_rng(semente)
  local estado = bit.band(semente, 0x7fffffff)
  if estado == 0 then estado = 1 end
  return function(limite_exclusivo)
    assert(limite_exclusivo > 0)
    estado = bit.bxor(estado, bit.lshift(estado, 13))
    estado = bit.bxor(estado, bit.rshift(estado, 17))
    estado = bit.bxor(estado, bit.lshift(estado, 5))
    estado = bit.band(estado, 0x7fffffff)
    return estado % limite_exclusivo
  end
end

function M.normalizar_entrada(action_id, action)
  if action_id == hash("acao_primaria") and action.pressed then
    return { acao = "primaria", pressionada = true }
  end
  return nil
end

function M.log_estruturado(evento, dados)
  local registro = dados or {}
  registro.evento = evento
  print(json.encode(registro))
end

return M
