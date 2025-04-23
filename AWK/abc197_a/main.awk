function solve(s, head, tail) {
  head = substr(s, 1, 1)
  tail = substr(s, 2)
  return tail "" head
}

BEGIN {
  if ((getline line) > 0) {
    print solve(line)
  }
}
