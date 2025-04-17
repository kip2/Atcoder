function solve(a, b, c) {
  if (a * a + b * b < c * c) {
    return "Yes"
  } else {
    return "No"
  }
}

BEGIN {
  getline
  split($0, nums, " ")
  a = nums[1]
  b = nums[2]
  c = nums[3]
  
  print solve(a, b, c)
}

