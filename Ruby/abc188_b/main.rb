def get_numbers
  gets.split.map(&:to_i)
end

def get_input_as_string
  gets.chomp
end

def get_input_as_int
  gets.chomp.to_i
end

def solve(n, a, b)
  sum = 0
  for i in 0...n 
    sum += a[i] * b[i]
  end

  if sum == 0 
    "Yes"
  else
    "No"
  end
end

def main
  n = get_input_as_int
  a = get_numbers
  b = get_numbers

  puts solve(n, a, b)
end

main
