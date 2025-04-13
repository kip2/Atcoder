main :: IO ()
main = do
  line <- getLine
  let [a, b, c] = map read $ words line :: [Int]
  if a ^ 2 + b ^ 2 + c ^ 2
    then putStrLn "Yes"
    else putStrLn "No"
