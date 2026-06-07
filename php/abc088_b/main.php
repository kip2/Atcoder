<?php

function main() {
  $n = (int) trim(fgets(STDIN));

  $line = trim(fgets(STDIN));
  $a = array_map('intval', explode(' ', $line));

  $result = solve($n, $a);

  echo $result . PHP_EOL;
}

function solve(int $n, array $card): int {
  $a = 0;
  $b = 0;

  rsort($card);

  for ($i = 0; $i < $n; $i++) {
    if ($i % 2 == 0) {
      $a += $card[$i];
    } else {
      $b += $card[$i];
    }
  }

  return $a - $b;
}

main();
