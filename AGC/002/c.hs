import Data.List
main = do
    n:l:_ <- (map read . words) <$> getLine
    as <- (map read . words) <$> getLine
    let (b, i) = solve l 1 
    putStrLn $ 
    putStrLn $ show $ 0 + n + l
    putStrLn $ show $ 0 + sum as

solve :: Int -> Int -> [Int] -> (Bool, Int)
solve _ _ (_:[]) = (False, 0)
solve l i (a:b:cs)
    | a+b >= l  = (True, i)
    | otherwise = solve l (i+1) (b:cs)
