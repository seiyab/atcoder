main = do
    _ <- getLine
    as <- fmap (fmap read . words) getLine
    bs <- fmap (fmap read . words) getLine
    putStrLn $ show $ solve as bs

solve :: [Int] -> [Int]-> Int
solve _ [] = 0
solve (a1:a2:as) (b:bs) = z + solve (a:as) bs
    where a = a2 - max 0 (z-a1)
          z = min (a1+a2) b