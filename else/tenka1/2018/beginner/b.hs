main = interact solve
solve = f 0 <$>(map read).words
f::Int->[Int]->String
f i (a:b:k:[])
    | i==k  = show a ++ " " ++ show b ++ "\n"
    | mod i 2 == 0 = f (i+1)$(div a 2):(b+div a 2):k:[]
    | otherwise = f (i+1)$(a+div b 2):(div b 2):k:[]