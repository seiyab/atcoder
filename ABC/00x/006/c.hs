main=interact$(++"\n").unwords.map show.f.map read.words
f(n:m:[])|k-z>2*(n-z)||k<0=[-1,-1,-1]|otherwise=[n-z-a,z,a]where a=div k 2;k=m-2*n-z;z=mod m 2