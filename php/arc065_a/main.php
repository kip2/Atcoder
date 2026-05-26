<?php

function solve($s): string {
  $words = ["dream", "dreamer", "erase", "eraser"];

  while ($s != "") {
    $matched = false;
    foreach ($words as $word) {
      if (str_ends_with($s, $word)) {
        $s = substr($s, 0, strlen($s) - strlen($word));
        $matched = true;
        break;
      }
    }
    if (!$matched) {
      return "NO";
    }
  }
  return "YES";
}

function main() {
  $s = trim(fgets(STDIN));

  $result = solve($s);
  echo $result . PHP_EOL;
}

main();
