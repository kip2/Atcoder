@main def Main(): Unit =
  val lines = scala.io.StdIn.readLine()
  val result = solve(lines)
  println(result)

def solve(input: String): String =
  val x = input.substring(0, 1)
  val xs = input.substring(1, input.length)
  xs + x
