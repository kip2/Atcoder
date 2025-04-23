import 'dart:io';

void main() {
  final line = stdin.readLineSync()!;
  final parts = line.split(" ").map(int.parse).toList();

  final a = parts[0];
  final b = parts[1];
  final c = parts[2];

  final result = solve(a, b, c);
  print(result);
}

String solve(int a, int b, int c) => (a * a + b * b < c * c) ? "Yes" : "No";
