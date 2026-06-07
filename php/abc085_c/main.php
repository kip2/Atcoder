<?php

function main() {
  $line = fgets(STDIN);
  [$n, $y] = array_map("intval", explode(" ", $line));
  $y /= 1000;

  for ($i = 0; $i <= $n; $i++) {
    for ($j = 0; $j <= ($n - $i); $j++) {
      $k = $n - ($i + $j);
      if ($i + $j + $k !== $n)
        continue;
      if (10 * $i + 5 * $j + $k === $y) {
        echo $i . " " . $j . " " . $k . PHP_EOL;
        exit;
      }
    }
  }

  echo "-1 -1 -1" . PHP_EOL;
}

main();
