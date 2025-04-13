solve :: String
solve =
  "Your solved code"

main :: IO ()
main = do
  line <- getLine
  let [a, b, c] = map read $ words line :: [Int]
  putStrLn $ solve
