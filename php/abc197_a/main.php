<?php

function solve($s)
{
  $head = substr($s, 0, 1);
  $tail = substr($s, 1);
  return $tail . $head;
}

function get_input(): string
{
  return trim(fgets(STDIN));
}

function println($s): void
{
  echo ($s . "\n");
}

function main()
{
  $s = get_input();
  println(solve($s));
}

main();
