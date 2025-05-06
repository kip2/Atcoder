solve :: Int -> [Int] -> [Int]
solve x a = filter (/= x) a

readLineToIntArray :: IO [Int]
readLineToIntArray = map read . words <$> getLine

main :: IO ()
main = do
  line1 <- readLineToIntArray
  let n = head line1
  let x = line1 !! 1
  a <- readLineToIntArray
  let result = solve x a
  putStrLn $ unwords (map show result)
