
def solve(input)
  input[1..] + input[0]
end

def get_input_as_string
  gets.chomp
end

def main
  input = get_input_as_string
  puts solve(input)
end

main
