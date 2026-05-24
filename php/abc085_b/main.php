<?php

function solve($a, $b, $c) {
  $s = $a . $b . $c;
  println($s);
}

function main() {
  $n = (int) trim(fgets(STDIN));

  $riceCake = [];
  for ($i = 0; $i < $n; $i++) {
    $riceCake[] = (int) trim(fgets(STDIN));
  }

  rsort($riceCake);
  $riceCake = array_values(array_unique($riceCake));
  echo count($riceCake) . PHP_EOL;

}

main();
