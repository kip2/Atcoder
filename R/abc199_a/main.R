solve <- function(a, b, c) {
  if (a^2 + b^2 < c^2) {
    return("Yes")
  } else {
    return("No")
  }
}

main <- function() {
  line <- readLines("stdin", n = 1)
  nums <- as.integer(strsplit(line, " ")[[1]])
  result <- solve(nums[1], nums[2], nums[3])
  cat(result, "\n", sep="")
}

main()
