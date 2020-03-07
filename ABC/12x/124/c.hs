main = do
    s <- getLine
    putStrLn $ show $ min (solve '0' s) (solve '1' s)

solve _ [] = 0
solve b (a:as)
    | b == a = solve (fl b) as
    | otherwise = 1 + solve (fl b) as

fl '0' = '1'
fl _ = '0'