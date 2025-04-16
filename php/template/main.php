<?php

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
