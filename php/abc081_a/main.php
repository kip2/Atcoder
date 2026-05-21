<?php

$s = trim(fgets(STDIN));
$count = substr_count($s, "1");

echo $count . PHP_EOL;