package _go

import (
	"fmt"
	"sort"
	"testing"

	"gotest.tools/assert"
)

func eraseOverlapIntervals(intervals [][]int) int {
	var max int
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][0] < intervals[j][0]
	})
	fmt.Printf("%#v\n", intervals)
	if len(intervals) == 0 {
		return 0
	}
	root := Node{intervals[0][0], 0, nil, nil}
	for _, interval := range intervals {
		depth := append(&root, interval)
		if depth >= max {
			max = depth
		}
	}
	return len(intervals) - max
}

func append(node *Node, interval []int) int {
	top := interval[0]
	bottom := interval[1]
	if node.bottom == top {
		if node.child == nil {
			node.child = &Node{bottom, node.depth + 1, nil, nil}
			return node.depth + 1
		} else if node.bottom == bottom {
			return node.depth
		} else {
			addNext(node.child, bottom)
			return node.depth
		}
	}
	if node.child != nil {
		return append(node.child, interval)
	}
	n := searchNext(node, top)
	if n == nil {
		return 0
	}
	return append(n, interval)
}
func searchNext(node *Node, top int) *Node {
	fmt.Printf("node: %#v, top: %v\n", node, top)
	if node.bottom == top {
		return node
	}
	if node.next == nil {
		return nil
	}
	return searchNext(node.next, top)
}

func addNext(node *Node, bottom int) *Node {
	if node.bottom == bottom {
		return node
	} else if node.next == nil {
		node.next = &Node{bottom, node.depth, nil, nil}
		return node.next
	}
	return addNext(node.next, bottom)
}

type Node struct {
	bottom int
	depth  int
	child  *Node
	next   *Node
}

func Test_EraseOverlap(t *testing.T) {
	assert.Equal(t, eraseOverlapIntervals([][]int{{1, 2}, {2, 3}, {3, 4}, {1, 3}}), 1)
	assert.Equal(t, eraseOverlapIntervals([][]int{{1, 2}, {1, 2}, {1, 2}}), 2)
	assert.Equal(t, eraseOverlapIntervals([][]int{{1, 2}, {2, 3}}), 0)
	assert.Equal(t, eraseOverlapIntervals([][]int{{1, 2}, {2, 3}, {3, 4}, {1, 4}, {4, 5}}), 1)
	assert.Equal(t, eraseOverlapIntervals([][]int{{1, 100}, {11, 22}, {1, 11}, {2, 12}}), 2)
	assert.Equal(t, eraseOverlapIntervals([][]int{{1, 100}, {11, 22}, {1, 11}, {2, 12}, {1, 2}}), 3)
	assert.Equal(t, eraseOverlapIntervals([][]int{{0, 1}}), 0)
	assert.Equal(t, eraseOverlapIntervals([][]int{}), 0)
	assert.Equal(t, eraseOverlapIntervals([][]int{{0, 1}, {3, 4}, {1, 2}}), 0)
}
