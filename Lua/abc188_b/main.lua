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

local function solve(n, a_list, b_list)
  local sum = 0
  for i = 1, n do
    sum = sum + a_list[i] * b_list[i]
  end

  if sum == 0 then
    return "Yes"
  end

  return "No"
end


local function main()
  local n = read_number()
  local a_list = read_inputs()
  local b_list = read_inputs()

  print(solve(n, a_list, b_list))
end

main()
