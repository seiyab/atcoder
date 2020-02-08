import Data.IntSet
main = do
    n:m:_ <- (Prelude.map read . words) <$> getLine
    (xs, ys) <- getxy m
    putStrLn $ red n m
    putStrLn $ show xs
    putStrLn $ show $ simulate (fromList [1]) xs ys
    putStrLn $ show $ size $ simulate (fromList [1]) xs ys

getxy :: Int -> IO ([Int], [Int])
getxy 0 = do
    return ([], [])
getxy i = do
    x:y:_ <- (Prelude.map read . words) <$> getLine
    (xs, ys) <- getxy (i-1)
    return (x:xs, y:ys)

simulate :: IntSet -> [Int] -> [Int] -> IntSet
simulate s [] [] = s
simulate s (x:xs) (y:ys)
    | x `member` s = simulate (insert y s) xs ys
    | otherwise  = simulate s xs ys

red :: Int -> Int -> String
red a b = show a ++ show b
