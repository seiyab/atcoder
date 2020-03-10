main = do
    _ <- getLine
    bs <- map read <$> words <$> getLine
    print $ solve bs
solve bs = head bs + last bs + foldl (+) 0 (zipWith min bs $tail bs)