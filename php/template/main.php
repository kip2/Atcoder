<?php
function get_input(): string
{
  return trim(fgets(STDIN));
}

function println($s): void
{
  echo ($s . "\n");
}

function join_space($arr): string
{
  return implode(" ", $arr);
}

function parse_str_to_int_arr($line): array
{
  return array_map('intval', explode(' ', $line));
}

function get_input_int_arr(): array
{
  $line = trim(fgets(STDIN));
  return array_map('intval', explode(' ', $line));
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
