<?php

enum State {
  case START;
  case DREAM;
  case ERASE;
  case ER;
  case R;
}

function solve($s): string {

  $length = strlen($s);
  $i = 0;
  $state = State::START;
  $tmp = "";
  $no = "NO";
  $yes = "YES";

  while ($i < $length) {
    switch ($state) {
      case State::START:
        $tmp = substr($s, $i, 5);
        if ($tmp !== "dream" && $tmp !== "erase") {
          return $no;
        } else {
          if ($tmp === "dream") {
            $state = State::DREAM;
            $i += 5;
          } else if ($tmp === "erase") {
            $state = State::ERASE;
            $i += 5;
          }
        }
        break;
      case State::DREAM:
        $tmp = substr($s, $i, 5);
        if ($tmp === "erase") {
          $state = State::ERASE;
          $i += 5;
        } else if ($tmp === "dream") {
          $state = State::DREAM;
          $i += 5;
        } else {
          $tmp = substr($s, $i, 2);
          if ($tmp === "er") {
            $state = State::ER;
            $i += 2;
          } else {
            return $no;
          }
        }
        break;
      case State::ERASE:
        $tmp = substr($s, $i, 1);
        if ($tmp === "r") {
          $state = State::R;
          $i += 1;
        } else {
          $state = State::START;
        }
        break;
      case State::ER:
      case State::R:
        $state = State::START;
        break;
      default:
        return $no;
        break;
    }
  }

  return $yes;
}

function main() {
  $s = trim(fgets(STDIN));

  $result = solve($s);
  echo $result . PHP_EOL;
}

main();
