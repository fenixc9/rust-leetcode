// use std::cell::RefCell;
// use std::cmp::max;
// use std::fmt::Formatter;
// use std::fmt::Debug;
// use std::ops::Add;
// use std::rc::Rc;
//
// // node
// struct Node<T> {
//     height: i32,
//     data: T,
//     left: Option<Rc<RefCell<Node<T>>>>,
//     right: Option<Rc<RefCell<Node<T>>>>,
// }
//
// impl<T> Node<T> {
//     fn new(s: T) -> Self <> {
//         Node {
//             height: 1,
//             data: s,
//             left: None,
//             right: None,
//         }
//     }
// }
//
// struct AvlTree<T: Ord> {
//     root: Option<Rc<RefCell<Node<T>>>>,
// }
//
// impl<T: Ord> AvlTree<T> {
//     fn new() -> Self {
//         AvlTree { root: None }
//     }
// }
//
// impl<T: Debug + Ord> Debug for Node<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         self.data.fmt(f)
//     }
// }
//
// impl<T: Ord + Debug> Node<T> {
//     fn display(&self) {
//         let mut m = String::new();
//         if let Some(s) = &self.left {
//             m += &format!("{:?}", s.borrow().data);
//             m += "=>";
//         } else {
//             m += "END=>";
//         }
//         m += &format!("{:?}", self.data);
//         if let Some(s) = &self.left {
//             m += "<=";
//             m += &format!("{:?}", s.borrow().data);
//         } else {
//             m += "<=END";
//         }
//         println!("{}", m);
//         if let Some(s) = &self.left { s.borrow().display() }
//         if let Some(s) = &self.right { s.borrow().display() }
//     }
// }
//
// impl<T: Debug + Ord> AvlTree<T> {
//     fn display(&self) {
//         match &self.root {
//             None => {
//                 println!("EMPTY TREE")
//             }
//             Some(s) => {
//                 s.borrow().display()
//             }
//         }
//     }
// }
//
// impl<T: Ord> AvlTree<T> {
//     fn insert(&mut self, t: T) {
//         let r = match &self.root {
//             None => None,
//             Some(d) => { Some(d.clone()) }
//         };
//         self.root = Self::insert0(r, Rc::new(RefCell::new(Node::new(t))));
//     }
//
//     fn insert0(cur: Option<Rc<RefCell<Node<T>>>>, t: Rc<RefCell<Node<T>>>) -> Option<Rc<RefCell<Node<T>>>> {
//         return match cur {
//             None => { Some(t) }
//             Some(n) => {
//                 if n.borrow().data > t.borrow().data {
//                     n.borrow_mut().left = Self::insert0(Some(n.clone()), t.clone());
//                 } else if n.borrow().data < t.borrow().data {
//                     n.borrow_mut().right = Self::insert0(Some(n.clone()), t.clone());
//                 }
//                 n.borrow_mut().height = max(Self::height(&n.borrow().left), Self::height(&n.borrow().right)) + 1;
//                 let bf = Self::bf(Some(n.clone()));
//                 if bf > 1 && t.borrow().data < n.borrow().left.as_ref().unwrap().borrow().data { return Some(Self::right_rotate(n.clone())); }
//                 if bf < -1 && t.borrow().data > n.borrow().right.as_ref().unwrap().borrow().data { return Some(Self::left_rotate(n.clone())); }
//                 if bf < -1 && t.borrow().data < n.borrow().right.as_ref().unwrap().borrow().data {
//                     // let mut r = &n.borrow_mut().right;
//                     // r.replace(Self::right_rotate());
//                     n.borrow_mut().right = Some(Self::right_rotate(n.borrow_mut().right.as_ref().unwrap().clone()));
//                     return Some(Self::left_rotate(n));
//                 }
//                 if bf > 1 && t.borrow().data > n.borrow().left.as_ref().unwrap().borrow().data {
//                     n.borrow_mut().left = Some(Self::left_rotate(n.borrow_mut().left.as_ref().unwrap().clone()))
//                 }
//                 Some(n)
//             }
//         };
//     }
//
//
//     fn bf(cur: Option<Rc<RefCell<Node<T>>>>) -> i32 {
//         match cur {
//             None => 0,
//             Some(s) => { Self::height(&s.borrow().left) - Self::height(&s.borrow().right) }
//         }
//     }
//     fn height(cur: &Option<Rc<RefCell<Node<T>>>>) -> i32 {
//         match cur {
//             None => 0,
//             Some(s) => s.borrow().height
//         }
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use crate::common::tree::avl::AvlTree;
//
//     #[test]
//     fn f1() {
//         let mut tree = AvlTree::new();
//         tree.insert(2);
//         tree.insert(3);
//         tree.insert(1);
//     }
// }
