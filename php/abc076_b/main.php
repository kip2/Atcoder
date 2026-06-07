<?php

function main() {
  $n = (int) trim(fgets(STDIN));
  $k = (int) trim(fgets(STDIN));

  $x = 1;
  for ($i = 0; $i < $n; $i++) {
    $x = min($x * 2, $x + $k);
  }

  echo $x . PHP_EOL;
}

main();
