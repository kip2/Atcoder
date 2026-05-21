<?php

function getInput(): string {
  return trim(fgets(STDIN));
}

function readIntArray(): array {
  return array_map('intval', explode(' ', getInput()));
}

function countDivisibleBy2(int $x): int {
  $count = 0;
  while($x % 2 === 0) {
    $x = intdiv($x, 2);
    $count++;
  }
  return $count;
}

function main() {

  $_n = (int)getInput();

  $data = readIntArray();

  $counts = array_map('countDivisibleBy2', $data);

  echo min($counts) . PHP_EOL;
}

main();