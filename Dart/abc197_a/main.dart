import 'dart:io';

void main() {
  final line = stdin.readLineSync()!;
  final result = solve(line);
  print(result);
}

String solve(String input) {
  return input.substring(1) + input[0];
}
