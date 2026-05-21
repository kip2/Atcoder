<?php

function solve($s)
{
  $head = substr($s, 0, 1);
  $tail = substr($s, 1);
  return $tail . $head;
}

function getInput(): string
{
  return trim(fgets(STDIN));
}

function println($s): void
{
  echo ($s . "\n");
}

function main()
{
  $s = getInput();
  println(solve($s));
}

main();
