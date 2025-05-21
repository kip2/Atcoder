<?php
function get_input(): string
{
  return trim(fgets(STDIN));
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

function println($s): void
{
  echo ($s . "\n");
}

function solve($a, $x): array
{
  return array_filter($a, fn($n) => $n != $x);
}

function join_space($arr): string
{
  return implode(" ", $arr);
}

function main()
{
  $line = get_input_int_arr();
  $n = $line[0];
  $x = $line[1];
  $a = get_input_int_arr();

  $ret = solve($a, $x);

  println(join_space($ret));
}

main();
