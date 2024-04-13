/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM NOT DONE

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
        self.items.push(value);
        self.count += 1;
        self.percolate_up(self.count);
    }

    // 插入新元素后，将该元素向上移动
    // 参数 idx 表示要上浮的元素的索引
    fn percolate_up(&mut self, mut idx: usize) {
        let mut parent_idx; // 父节点的索引
        while idx > 1 {
            parent_idx = self.parent_idx(idx);
            // 然后比较当前节点与其父节点的值
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                // 交换它们的位置
                self.items.swap(idx, parent_idx);
                // 当前节点的索引更新为父节点的索引，继续向上比较,到达堆的顶部为止
                idx = parent_idx;
            } else {
                break;
            }
        }
    }

    // 删除根节点后，将最后一个元素向下移动
    fn percolate_down(&mut self, mut idx: usize) {
        // 检查当前节点是否有子节点
        while self.children_present(idx) {
            // 找到其中较小（或较大）的子节点
            let smallest_child = self.smallest_child_idx(idx);
            // 与当前节点进行比较
            if (self.comparator)(&self.items[smallest_child], &self.items[idx]) {
                // 交换它们的位置
                self.items.swap(smallest_child, idx);
                // 继续向下比较
                idx = smallest_child;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
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
        let left_child_idx = self.left_child_idx(idx);
        let right_child_idx = self.right_child_idx(idx);

        if right_child_idx > self.count || (self.comparator)(&self.items[left_child_idx], &self.items[right_child_idx]) {
            left_child_idx
        } else {
            right_child_idx
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
		if self.is_empty() {
            return None;
        }

        let root = self.items.swap_remove(1);
        self.count -= 1;
        if self.count > 0 {
            self.items.swap(1, self.count);
            self.percolate_down(1);
        }
        Some(root)
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