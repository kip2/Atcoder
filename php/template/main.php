<?php
function get_input(): string
{
  return trim(fgets(STDIN));
}

function println($s): void
{
  echo ($s . "\n");
}

function solve($a, $b, $c)
{
  "";
}

function main()
{
  $line = trim(fgets(STDIN));
  [$a, $b, $c] = array_map('intval', explode(' ', $line));
  solve($a, $b, $c);
}

main();
