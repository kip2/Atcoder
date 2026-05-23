<?php

function digitSum($x): int {
  $sum = 0;

  while (0 < $x) {
    $sum += $x % 10;
    $x = intDiv($x, 10);
  }
  return $sum;
}

function solve($n, $a, $b): int {
  $total = 0;
  for ($i = 1; $i <= $n; $i++) {
    $t = digitSum($i);
    if ($a <= $t && $t <= $b) {
      $total += $i;
    }
  }
  return $total;
}

function main() {
  $ln = trim(fgets(STDIN));
  [$n, $a, $b] = array_map('intval', explode(' ', $ln));

  $result = solve($n, $a, $b);
  echo $result . PHP_EOL;
}

main();
