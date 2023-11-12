use std::cmp::max;
use std::fmt::Debug;
use std::ptr::null_mut;

struct Node<T> {
    data: T,
    height: i32,
    left: *mut Node<T>,
    right: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(t: T) -> Self {
        Node {
            data: t,
            height: 1,
            left: null_mut(),
            right: null_mut(),
        }
    }
}

impl<T: Ord + Debug> Node<T> {
    fn display(&self) {
        let mut m = String::new();
        unsafe {
            if !self.left.is_null() {
                let node: Node<T> = self.left.read();
                m += &format!("{:?}", node.data);
                m += "=>";
            } else {
                m += "END=>";
            }
        }
        m += &format!("{:?}", self.data);
        unsafe {
            if !self.right.is_null() {
                let node = self.right.read();
                m += "<=";
                m += &format!("{:?}", node.data);
            } else {
                m += "<=END";
            }
        }
        println!("{}", m);
        if !self.left.is_null() { unsafe { (*self.left).display() } }
        if !self.right.is_null() { unsafe { (*self.right).display() } }
    }
}

struct AvlTree<T> {
    root: *mut Node<T>,
}

impl<T: Ord + Copy + Debug> AvlTree<T> {
    fn display(&self) {
        unsafe {
            if self.root.is_null() {
                println!("EMPTY")
            } else {
                let node = self.root.read();
                node.display();
            }
        }
    }

    fn new(t: T) -> Self {
        AvlTree { root: Box::into_raw(Box::new(Node::new(t))) }
    }

    fn insert(&mut self, t: T) {
        self.root = Self::insert0(self.root, t);
    }

    fn insert0(node: *mut Node<T>, t: T) -> *mut Node<T> {
        if node.is_null() {
            return Box::into_raw(Box::new(Node::new(t)));
        }
        unsafe {
            let n = node.read();
            if n.data > t { (*node).left = Self::insert0(n.left, t) }
            else if n.data < t { (*node).right = Self::insert0(n.right, t) }
            (*node).height = max(Self::height(n.left), Self::height(n.right));
            let bf = Self::bf(node);
            if bf > 1 && t < (*n.left).data { return Self::right_rotate(node); }
            if bf < -1 && t > (*n.right).data { return Self::left_rotate(node); }
            if bf < -1 && t < (*n.right).data {
                (*node).right = Self::right_rotate(n.right);
                return Self::left_rotate(node);
            }
            if bf > 1 && t > (*n.left).data {
                (*node).left = Self::right_rotate(n.left);
                return Self::right_rotate(node);
            }
        }
        node
    }

    unsafe fn height(n: *mut Node<T>) -> i32 {
        if n.is_null() { return 0; };
        return (*n).height;
    }

    unsafe fn bf(n: *mut Node<T>) -> i32 {
        if n.is_null() { return 0; }
        return Self::height((*n).left) - Self::height((*n).right);
    }

    unsafe fn left_rotate(n: *mut Node<T>) -> *mut Node<T> {
        let mut b = (*n).right;
        if b.is_null() { panic!("咋回事啊"); }
        let mut T3 = (*b).left;
        std::ptr::swap(b, T3);

        let bnode = b.read();
        let node = n.read();

        (*n).height = max(Self::height(node.left), Self::height(node.right));
        (*b).height = max(Self::height(bnode.left), Self::height(bnode.right));

        return b;
    }
    unsafe fn right_rotate(n: *mut Node<T>) -> *mut Node<T> {
        let mut b = (*n).left;
        if b.is_null() { panic!("咋回事啊"); }
        let mut T3 = (*b).right;
        std::ptr::swap(b, T3);

        let bnode = b.read();
        let node = n.read();

        (*n).height = max(Self::height(node.left), Self::height(node.right));
        (*b).height = max(Self::height(bnode.left), Self::height(bnode.right));

        return b;
    }
}

#[cfg(test)]
mod test {
    use crate::common::tree::raw_avl::AvlTree;

    #[test]
    fn f1() {
        let mut tree = AvlTree::new(5);
        tree.insert(2);
        tree.insert(3);
        tree.insert(1);

        tree.display();
    }
}

