use std::fmt::Debug;

struct Heap<T>
    where
        T: Ord,
{
    inner: Vec<T>,
    size: usize,
}

impl<T> Heap<T>
    where
        T: Ord,
{
    pub fn new() -> Self {
        Heap {
            inner: Vec::new(),
            size: 0,
        }
    }

    pub fn push(&mut self, n: T) {
        self.size += 1;
        self.inner.push(n);
        // parent compare
        let mut index = self.size - 1;
        while index > 0 && self.inner[parent(index)] < self.inner[index] {
            self.swap(index, parent(index));
            index = parent(index);
        }
    }

    fn swap(&mut self, i1: usize, i2: usize) {
        self.inner.swap(i1, i2);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.swap(0, self.size - 1);
            self.size -= 1;
            self.heapfy(0);
            Some(self.inner.remove(self.size))
        } else {
            None
        }
    }

    fn heapfy(&mut self, index: usize) {
        let mut lagest = index;

        if left_child(index) < self.size && self.inner[left_child(index)] > self.inner[lagest] {
            lagest = left_child(index);
        }
        if right_child(index) < self.size && self.inner[right_child(index)] > self.inner[lagest] {
            lagest = right_child(index);
        }
        if lagest != index {
            self.swap(lagest, index);
            self.heapfy(lagest);
        }
    }
}

#[allow(dead_code)]
#[inline]
fn left_child(i: usize) -> usize {
    (i << 1) + 1
}

#[allow(dead_code)]
#[inline]
fn right_child(i: usize) -> usize {
    (i << 1) + 2
}

#[allow(dead_code)]
#[inline]
fn parent(i: usize) -> usize {
    (i - 1) >> 1
}

fn print_all<T: Debug + Ord>(h: Heap<T>) {
    let mut heap = h;
    while let Some(f) = &heap.pop() {
        println!("{:?}", f);
    }
}

#[test]
fn f1() {
    let mut heap = Heap::new();
    heap.push(3);
    heap.push(1);
    heap.push(2);

    print_all(heap);
}