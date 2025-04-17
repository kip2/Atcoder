solve <- function(a, b, c) {
  ""
}

main <- function() {
  line <- readLines("stdin", n = 1)
  nums <- as.integer(strsplit(line, " ")[[1]])
  result <- solve(nums[1], nums[2], nums[3])
  cat(result, "\n", sep="")
}

main()
