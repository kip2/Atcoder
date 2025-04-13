main :: IO ()
main = do
  line <- getLine
  let [a, b, c] = map read $ words line :: [Int]
  if a * a + b * b < c * c
    then putStrLn "Yes"
    else putStrLn "No"
