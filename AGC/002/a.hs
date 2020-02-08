main = do
    a:b:_ <- (map (\x->read x::Int) . words) <$> getLine
    putStrLn $ solve a b

solve :: Int -> Int -> String
solve a b
    | a * b <= 0 = "Zero"
    | a > 0      = "Positive"
    | (b-a) `mod` 2 == 0 = "Negative"
    | otherwise          = "Positive"
