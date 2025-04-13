@main def Main(): Unit =
  val lines = scala.io.StdIn.readLine()
  val result = solve(lines)
  println(result)

def solve(input: String): String =
  val Array(a, b, c) = input.split(" ").map(_.toInt)
  if (a * a + b * b < c * c) "Yes" else "No"
