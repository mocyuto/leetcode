package main

import "fmt"

func missingNumber(nums []int) int {
	find := make([]bool, len(nums)+1, len(nums)+1)
	for _, i := range nums {
		find[i] = true
	}
	fmt.Println(find)
	for k, v := range find {
		if !v {
			return k
		}
	}
	return len(nums)
}
func main() {
	fmt.Println(missingNumber([]int{3, 0, 1}))
}
