import scala.io.StdIn.readLine

@main def Main(): Unit =
  val list = readInputArray()
  val _n = list(0)
  val x = list(1)

  val a = readInputArray()

  val result = solve(a, x)
  val output = joinIntArray(result)
  println(output)

def solve(a: Array[Int], x: Int): Array[Int] =
  a.filter(_ != x)

def joinIntArray(arr: Array[Int]): String =
  arr.mkString(" ")

def readInputInt(): Int =
  readLine().toInt

def readInputArray(): Array[Int] =
  readLine().split(" ").map(_.toInt)
