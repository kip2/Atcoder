def get_numbers
  gets.split.map(&:to_i)
end

def get_input_as_string
  gets.chomp
end

def get_input_as_int
  gets.chomp.to_i
end

def solve()

end

def main
  input = get_input_as_string
  puts input
end

main
