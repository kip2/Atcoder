<?php

function solve($a, $b, $c)
{
  if ($a * $a + $b * $b < $c * $c) {
    echo "Yes\n";
  } else {
    echo "No\n";
  }
}

function main()
{
  $line = trim(fgets(STDIN));
  [$a, $b, $c] = array_map('intval', explode(' ', $line));
  solve($a, $b, $c);
}

main();
