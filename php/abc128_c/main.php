<?php

function isOnSwitch(int $bit, int $switchNum): bool {
  return (($bit & (1 << $switchNum - 1)) !== 0);
}

function isOnRamp(array $switches, int $bit, int $on): bool {
  $onCount = 0;

  foreach ($switches as $sw) {
    if (isOnSwitch($bit, $sw)) {
      $onCount++;
    }
  }

  return $onCount % 2 === $on;
}

function main() {
  [$n, $m] = array_map("intval", explode(" ", trim(fgets(STDIN))));

  $switches = [];
  for ($bit = 0; $bit < $m; $bit++) {
    $line = array_map("intval", explode(" ", trim(fgets(STDIN))));
    $k = $line[0];

    $arr = [];
    for ($j = 1; $j <= $k; $j++) {
      $arr[] = $line[$j];
    }

    $switches[] = $arr;
  }

  $p = array_map("intval", explode(" ", trim(fgets(STDIN))));

  $answer = 0;

  for ($bit = 0; $bit < (1 << $n); $bit++) {
    $ok = true;

    for ($lamp = 0; $lamp < $m; $lamp++) {
      if (!isOnRamp($switches[$lamp], $bit, $p[$lamp])) {
        $ok = false;
        break;
      }
    }
    if ($ok) {
      $answer++;
    }
  }

  echo $answer . PHP_EOL;
}

main();
