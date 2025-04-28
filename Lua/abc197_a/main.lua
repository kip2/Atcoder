local function read_a_input()
  local input = io.read("*l")
  return input
end

local function solve(input)
  local head = string.sub(input, 0, 1)
  local tail = string.sub(input, 2, 3)
  return tail .. head
end

local function main()
  local input = read_a_input()
  local result = solve(input)
  print(result)
end

main()
