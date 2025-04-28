local function read_inputs()
  local line = io.read("*l")
  local numbers = {}
  for num in string.gmatch(line, "%S+") do
    table.insert(numbers, tonumber(num))
  end
  return numbers
end

local function solve()
end

local function main()
  local input = read_inputs()
  print(input)
end

main()
