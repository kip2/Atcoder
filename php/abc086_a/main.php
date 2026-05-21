<?php

function solve($a, $b)
{
  $mul = $a * $b;
  if ($mul % 2 == 0) {
    return "Even";
  } else {
    return "Odd";
  }
}

function main()
{
  $line = trim(fgets(STDIN));
  [$a, $b] = array_map('intval', explode(' ', $line));
  $result = solve($a, $b);
  echo $result . "\n";
}

main();
