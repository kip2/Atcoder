<?php

function main() {
  $line = trim(fgets(STDIN));
  $data = array_map('intval', explode(' ', $line));

  $counts = array_count_values($data);
  $result = array_filter($data, fn($x) => $counts[$x] === 1);

  if (count($result) === 1)
    echo "Yes" . PHP_EOL;
  else
    echo "No" . PHP_EOL;
}

main();
