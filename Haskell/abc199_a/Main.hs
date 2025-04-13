solve :: Int -> Int -> Int -> String
solve a b c =
  if a * a + b * b < c * c
    then "Yes"
    else "No"

main :: IO ()
main = do
  line <- getLine
  let [a, b, c] = map read $ words line :: [Int]
  putStrLn $ solve a b c
