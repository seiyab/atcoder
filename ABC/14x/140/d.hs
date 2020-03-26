main = do
    n:k:_ <- (map read).words <$> getLine
    s <- getLine
    let u = n - (happy $ ['X'] ++ s ++ ['X'])
    let m = max 1 (u - 2*k)
    putStrLn$show (n-m)
happy (_:'R':a:as)
    | a=='R' = 1+ happy ('R':a:as)
    | otherwise = happy ('R':a:as)
happy (a:'L':as)
    | a=='L' = 1+happy ('L':as)
    | otherwise = happy ('L':as)
happy (_:_:[]) = 0