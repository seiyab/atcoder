import Data.List
main = do
    x <- (fmap read).words<$>getContents
    print$solve x
solve (n:ds) = as!!(m+1)-as!!m
    where as = sort ds
          m = div n 2-1