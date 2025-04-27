@main def Main(): Unit =
  // read a int value
  // val sc = new java.util.Scanner(System.in)
  // val n = sc.nextInt()

  // read line value
  // val lines = scala.io.StdIn.readLine()

  val result = solve(lines)
  println(result)

def solve(input: String): String =
  // Todo: solve code
  input

def readInputArray(sc: java.util.Scanner, n: Int): Array[Int] =
  // example
  // val sc = new java.util.Scanner(System.in)
  // val a = readInputArray(sc, n)
  // val b = readInputArray(sc, n)
  Array.fill(n)(sc.nextInt())
