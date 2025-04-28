local function read_numbers()
  local line = io.read("*l")
  local numbers = {}
  for num in string.gmatch(line, "%S+") do
    table.insert(numbers, tonumber(num))
  end
  return numbers
end

local function solve(a, b, c)
  if a * a + b * b < c * c then
    return "Yes"
  else
    return "No"
  end
end

local function main()
  local nums = read_numbers()
  local a, b, c = nums[1], nums[2], nums[3]

  local result = solve(a, b, c)
  print(result)
end

main()
