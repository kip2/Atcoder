<?php

function main() {

  $a = (int) trim(fgets(STDIN));
  $b = (int) trim(fgets(STDIN));
  $c = (int) trim(fgets(STDIN));
  $x = (int) trim(fgets(STDIN));

  $yen500 = 500;
  $yen100 = 100;
  $yen50 = 50;

  $count = 0;
  for ($i = 0; $i <= $a; $i++) {
    for ($j = 0; $j <= $b; $j++) {
      for ($k = 0; $k <= $c; $k++) {
        if (($i * $yen500 + $j * $yen100 + $k * $yen50) === $x)
          $count++;
      }
    }
  }

  echo $count . PHP_EOL;
}

main();
