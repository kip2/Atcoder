<?php
function main()
{
  $a = trim(fgets(STDIN));
  $line = trim(fgets(STDIN));

  [$b, $c] = array_map('intval', explode(' ', $line));
  $sum = $a + $b + $c;

  $str = trim(fgets(STDIN));

  echo $sum . " " . $str . "\n";
}

main();
