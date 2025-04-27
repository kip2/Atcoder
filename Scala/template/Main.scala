import scala.io.StdIn.readLine

@main def Main(): Unit =
  // read a int value
  // val n = readInputInt()

  // read a line to Int
  // val a = readInputArray()

  // read line value
  // val lines = scala.io.StdIn.readLine()

  val result = solve()
  println(result)

def solve(input: String): String =
  // Todo: solve code
  input

def readInputInt(): Int =
  readLine().toInt

def readInputArray(): Array[Int] =
  readLine().split(" ").map(_.toInt)
