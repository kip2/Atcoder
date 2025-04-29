solve :: [Int] -> [Int] -> String
solve a b =
  if sum (zipWith (*) a b) == 0 then "Yes" else "No"

readLineToInt :: IO Int
readLineToInt = read <$> getLine

readLineToIntArray :: IO [Int]
readLineToIntArray = map read . words <$> getLine

main :: IO ()
main = do
  _ <- readLineToInt
  a <- readLineToIntArray
  b <- readLineToIntArray

  putStrLn $ solve a b