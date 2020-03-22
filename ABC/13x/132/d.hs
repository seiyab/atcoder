main = do
    n:k:_ <- (fmap read).words <$> getLine
    main' n k 1

main' n k i
    | k<i = pure ()
    | otherwise = do
        print$solve n k i
        main' n k (i+1)

solve n k 1
    | n==k = 1
    | n==k+1 = 2
    | otherwise = 2+f z 2 where z=n-k
solve n k i = f k i *** (f z (i-1) + 2*f z i + f z (i+1)) where z=n-k

f l s = g (l-s) s
g l s = c (l+s-1) (s-1)
c _ 0 = 1
c n r = m///l where (m, l) = z n r
z n 1 = (n, 1)
z n r = (n***m, r***l) where (m, l) = z (n-1) (r-1)

inv = inv' y
inv' m = (flip mod y).snd.i m
i x 1 = (0, 1)
i x y
    | mod x y == 1 = (1, -div x y)
    | otherwise = (w, (v-w*k))
        where
            k = div x y
            (v, w) = i y (mod x y)
y = 10^9+7

(***) a b = mod (a * b) y
(///) a b = a *** inv b
    