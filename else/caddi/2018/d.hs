main = do
    n <- read<$>getLine
    as <- g n
    putStrLn$solve as

g 0 = pure []
g n = do
    x <- read<$>getLine
    r <- g$n-1
    pure$x:r

solve as = if foldl1 (&&) (map even as)
    then "second"
    else "first"