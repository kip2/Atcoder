import 'dart:io';

void main() {
  final line = stdin.readLineSync()!;
  final result = solve(line);
  print(result);
}

String solve(String input) {
  // 例: 先頭1文字を最後に回す
  return input.substring(1) + input[0];
}
