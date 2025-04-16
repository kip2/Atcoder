def solve(a, b, c)
  if a * a + b * b < c * c
    puts "Yes"
  else
    puts "No"
  end
end

def main
  line = gets
  a, b, c = line.split.map(&:to_i)
  solve(a, b, c)
end

main
