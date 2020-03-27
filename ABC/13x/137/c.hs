import Data.List

main = interact $ show.s.tail.words

s::[[Char]]->Int
s as = foldl1 (+) $ (map c.f.sort.map sort) as
f::[[Char]]->[Int]
f (_:[]) = 1:[]
f (s:ss)
    | s==head ss = (1+h):hs
    | otherwise = 1:h:hs
    where h:hs = f ss
c::Int->Int
c n = div (n*(n-1)) 2
