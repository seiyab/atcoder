package main

import (
	"fmt"
)

func main() {
	var h, w, k int
	const m = 1000*1000*1000 + 7
	fmt.Scan(&h, &w, &k)

	var z [9][2]int
	z[0][0] = 1
	z[0][1] = 1
	z[1][0] = 1
	z[1][1] = 1
	for i := 2; i < 9; i++ {
		z[i][0] = z[i-1][0] + z[i-1][1]
		z[i][1] = z[i-1][0]
	}

	var dp [101][8]int
	dp[0][0] = 1
	for i := 1; i < h+1; i++ {
		for j := 0; j < w; j++ {
			dp[i][j] = (dp[i-1][j] * (z[j][0] * z[w-j-1][0])) % m
		}
		for j := 0; j < w-1; j++ {
			k := z[j][0] * z[w-j-2][0]
			dp[i][j] = (dp[i][j] + k*dp[i-1][j+1]) % m
			dp[i][j+1] = (dp[i][j+1] + k*dp[i-1][j]) % m
		}
	}
	fmt.Println(dp[h][k-1])
}
