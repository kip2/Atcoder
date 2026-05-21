<?php
function println($s): void
{
  echo ($s . "\n");
}

function solve($x, $a, $b)
{
  $result = eat($x, $a, $b);

  println($result);
}

function eat($x, $a, $b)
{
  $daysExpiration = -$a + $b;

  if ($daysExpiration <= 0)
    return "delicious";
  else if ($daysExpiration <= $x)
    return "safe";
  else
    return "dangerous";
}

function main()
{
  $line = trim(fgets(STDIN));
  [$x, $b, $c] = array_map('intval', explode(' ', $line));
  solve($x, $b, $c);
}

main();
