solve :: String -> String
solve (x : xs) =
  xs ++ [x]

main :: IO ()
main = do
  str <- getLine
  putStrLn $ solve str
