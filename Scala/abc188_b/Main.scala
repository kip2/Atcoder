import scala.util.chaining.*
import scala.io.StdIn.readLine

@main def Main(): Unit =
  val n = readInputInt()
  val a = readInputArray()
  val b = readInputArray()

  val result = solve(a, b)
  println(result)

def readInputInt(): Int =
  readLine().toInt

def readInputArray(): Array[Int] =
  readLine().split(" ").map(_.toInt)

def solve(a: Array[Int], b: Array[Int]): String =
  (a zip b)
    .map { case (x, y) => x * y }
    .sum
    .pipe:
      case 0 => "Yes"
      case _ => "No"
