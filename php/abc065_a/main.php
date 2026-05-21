<?php

[$x, $a, $b] = array_map('intval', explode(' ', trim(fgets(STDIN))));
$diff = $b - $a;
echo match (true) {
  $diff <= 0 => 'delicious',
  $diff <= $x => 'safe',
  default => 'dangerous',
} . PHP_EOL;
