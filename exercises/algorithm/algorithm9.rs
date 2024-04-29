/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            // 留下0号位，不使用
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        self.items.push(value);
        let new_idx = self.count;
        self.heapify_from_bottom_to_top(new_idx);
    }

    fn heapify_from_bottom_to_top(&mut self, mut idx: usize) {
        // 目的：针对某一个元素自下而上堆化
        // 因为加入的新元素可能破坏堆结构
        // idx 为新元素的索引
        // 假设现在需要生成小顶堆，则比较器接口 comparator 应为 a < b -> true

        while idx > 1 {
            if (self.comparator)(&self.items[ idx ], &self.items[ idx/2 ]) {
                // 这里的 i > 1 是因为初始化堆时，已经预留了0号位不使用，所以此时堆中只有一个元素
                // 如果新元素比它现在的父节点大，就交换两者
                self.items.swap(idx, idx/2);
                idx /= 2;
                // 此时新元素的索引已经更新为 i/2 ，再次比较，直到上方元素小于或等于下方元素则终止
                // 注：堆中允许存在重复的元素
            } else {
                break;
                // !!! 这一条break非常重要！！！
                // 用于判定元素是否已经到达符合条件的位置
                // 如果没有break，会导致idx一直没有改变，从而陷入死循环
            }
        }
    }

    fn heapify_from_top_to_bottom(&mut self, mut idx: usize) {
        // 目的：针对某一个元素自上而下堆化
        // 其他同上
        while self.children_present(idx) {
            // 如果当前节点有子节点
            let schild = self.smallest_child_idx(idx);
            // schild 是当前节点下索引应当最小的那个元素的索引
            if (self.comparator)( &self.items[ schild ], &self.items[ idx ] ) {
                // 如果该子节点的值比新元素小，则需要将子节点的值上移
                self.items.swap( schild, idx );
                idx = schild;
                // 此时新元素的索引已经更新为schild，重复该操作直到该元素到达正确的位置，
                // 或者该元素已经没有子节点
            } else {
                break;
                // !!! 这一条break非常重要！！！
                // 用于判定元素是否已经到达符合条件的位置
                // 如果没有break，会导致idx一直没有改变，从而陷入死循环
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        // 该方法用于判定当前节点有无子节点
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        // 目的：返回当前节点下索引应当最小的那个元素的索引（？？？）
        let lchild = self.left_child_idx(idx);
        let rchild = self.right_child_idx(idx);
        if rchild > self.count || (self.comparator)( &self.items[ lchild ], &self.items[ rchild ] ) {
            // 根据堆的结构：最底层的节点靠左填充，其他层节点全部被填满，
            // 如果右边没有元素（表现为用于储存数据的栈 items 长度不够），则直接返回左节点索引
            // 对小顶堆来说，每个节点左侧的节点值必定比右侧的大，此时返回左节点索引
            lchild
        } else {
            rchild
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.count == 0 {
            return None;
        }  
        // 如果堆中存在元素，就将其与堆顶元素交换后移出该元素
        let next_elem = self.items.swap_remove(1);
        self.count -= 1;
        self.heapify_from_top_to_bottom(1);
        Some(next_elem)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}