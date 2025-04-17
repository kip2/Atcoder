# main.jq
def solve($a; $b; $c):
  if ($a * $a + $b * $b < $c * $c) then
    "Yes"
  else
    "No"
  end;

(split(" ") | map(tonumber)) as [$a, $b, $c]
| solve($a; $b; $c)
