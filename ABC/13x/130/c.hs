main = do
    a:b:c:d:_ <- fmap (fmap read . words) getLine 
    (s, g) <- solve a b c d
    putStr $ show s
    putStr " "
    putStrLn $ show g

solve :: Int -> Int -> Int -> Int -> IO (Float, Int)
solve a b c d = pure (s, g)
    where 
        s = fromIntegral a * fromIntegral b / 2.0
        g = if mod a 2==0 && mod b 2==0 && div a 2 ==c && div b 2 ==d
            then 1
            else 0
