package main

import (
	"fmt"
	"github.com/magiconair/properties/assert"
	"testing"
)

var stair = map[int]int{0: 0}

func climbStairs(n int) int {
	for i := 0; i < n; i++ {
		c := stairs(i+1, stair)
		stair[i+1] = c
	}
	fmt.Println(stair)
	return stair[n]
}
func stairs(n int, stair map[int]int) int {
	if n == 1 {
		return 1
	} else if n == 2 {
		return 2
	}
	return stair[n-1] + stair[n-2]
}

func Test_climb(t *testing.T) {
	fmt.Println(climbStairs(3))
	assert.Equal(t, climbStairs(2), 2)
	assert.Equal(t, climbStairs(3), 3)
	assert.Equal(t, climbStairs(4), 5) // 1,1,1,1  1,2,1, 2,2
}

func maxSubArray(nums []int) int {
	m := nums[0]
	tmpSum := 0
	for _, n := range nums {
		if tmpSum < 0 {
			tmpSum = 0
		}
		tmpSum += n
		if m < tmpSum {
			m = tmpSum
		}
		println(n, tmpSum)
	}
	return m
}
func Test_maxSubArray(t *testing.T) {
	assert.Equal(t, maxSubArray([]int{4, -1, 2, 1, -5, 3, -3, 4, -1, 2, 1, -5, 4}), 7)
}
