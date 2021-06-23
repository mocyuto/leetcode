package main

import (
	"fmt"
	"testing"

	"github.com/magiconair/properties/assert"
)

func generate(numRows int) [][]int {
	res := [][]int{{1}}
	for i := 1; i < numRows; i++ {
		res = append(res, next(res[i-1]))
	}
	return res
}

func next(b []int) []int {
	n := []int{}
	for i, v := range b {
		if i == 0 {
			n = append(n, 1)
		} else {
			n = append(n, v+b[i-1])
		}
	}
	n = append(n, 1)
	return n
}

func Test118_generate(t *testing.T) {
	fmt.Println(next([]int{1, 1}))
	assert.Equal(t, [][]int{{1}, {1, 1}, {1, 2, 1}, {1, 3, 3, 1}, {1, 4, 6, 4, 1}}, generate(5))
	assert.Equal(t, [][]int{{1}}, generate(1))
}
