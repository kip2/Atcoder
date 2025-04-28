local function read_inputs()
  local line = io.read("*l")
  local numbers = {}
  for num in string.gmatch(line, "%S+") do
    table.insert(numbers, tonumber(num))
  end
  return numbers
end

local function read_number()
  return tonumber(io.read("*l"))
end

local function read_a_input()
  local input = io.read("*l")
  return input
end

local function solve()
end

-- debug lib
local function table_to_string(tbl)
  local result = {}
  for _, v in ipairs(tbl) do
    table.insert(result, tostring(v))
  end
  return table.concat(result, " ")
end

local function main()
  local input = read_inputs()
  print(input)
end

main()
