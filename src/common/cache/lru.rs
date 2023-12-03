use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ptr::{drop_in_place, null_mut};

pub struct Entry<K, V> {
    pre: *mut Entry<K, V>,
    next: *mut Entry<K, V>,
    k: K,
    v: V,
}

pub struct LruCache<K, V>
where
    K: Hash + Eq + Copy,
{
    data: HashMap<K, *mut Entry<K, V>>,
    head: *mut Entry<K, V>,
    tail: *mut Entry<K, V>,
    cap: usize,
}

impl<K, V> Drop for LruCache<K, V>
where
    K: Hash + Eq + Copy,
{
    fn drop(&mut self) {
        let mut cur = self.head;
        unsafe {
            while !cur.is_null() {
                self.data.remove(&(*cur).k);
                let d = cur;
                cur = (*cur).next;
                // drop(Box::from_raw(d));
                drop_in_place(d);
            }
        }
    }
}

#[allow(dead_code)]
impl<K, V> LruCache<K, V>
where
    K: Hash + Eq + Copy,
{
    pub fn new(cap: usize) -> Self {
        LruCache {
            data: HashMap::new(),
            head: null_mut(),
            tail: null_mut(),
            cap,
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn get(&mut self, k: &K) -> Option<&V> {
        if !self.data.contains_key(k) {
            return None;
        };
        let r = *self.data.get(k).unwrap();
        unsafe {
            self.move_to_last(r);
            return Some(&(*r).v);
        }
    }

    pub fn print(&self)
    where
        K: Debug,
        V: Debug,
    {
        let mut cur = self.head;
        while !cur.is_null() {
            unsafe {
                println!("k = {:?} v = {:?}", (*cur).k, (*cur).v);
                cur = (*cur).next;
            }
        }
    }

    // mut版

    unsafe fn move_to_last(&mut self, entry: *mut Entry<K, V>)
    where
        K: Eq + Hash,
    {
        if self.tail.is_null() {
            return;
        }
        let e = entry.read();
        let pre = e.pre;
        let next = e.next;
        if !pre.is_null() {
            (*pre).next = next;
        }
        if !next.is_null() {
            (*next).pre = pre;
        }
        if self.head == entry {
            self.head = next
        }
        (*self.tail).next = entry;
        (*entry).pre = self.tail;
        (*entry).next = std::ptr::null_mut();
        self.tail = entry;
    }

    // swap版
    // unsafe fn move_to_last(& mut self, entry: *mut Entry<K, V>)
    // where
    //     K: Eq + Hash,
    // {
    //     if self.tail.is_null() {
    //         return;
    //     }
    //     let e = entry.read();
    //     let pre = e.pre;
    //     let next = e.next;
    //     if !pre.is_null() {
    //         (*pre).next = next;
    //     }
    //     if !next.is_null() {
    //         (*next).pre = pre;
    //     }
    //     if self.head == entry {
    //         self.head = ne
    //     }
    //     (*self.tail).next = entry;
    //     (*entry).pre = self.tail;
    //     (*entry).next = std::ptr::null_mut();
    //     self.tail.swap(entry);
    // }

    pub fn put(&mut self, k: K, v: V) {
        if self.data.contains_key(&k) {
            let entry = *self.data.get(&k).unwrap();
            unsafe {
                (*entry).v = v;
                self.move_to_last(entry);
                return;
            }
        };
        unsafe {
            if self.data.len() == self.cap {
                let new_entry = self.evict();
                self.data.remove(&(*new_entry).k);
                drop(Box::from_raw(new_entry));
            }
            let new_entry = Box::into_raw(Box::new(Entry {
                pre: null_mut(),
                next: null_mut(),
                k,
                v,
            }));
            self.add_new_entry(new_entry);
            self.data.insert(k, new_entry);
        }
    }

    unsafe fn add_new_entry(&mut self, new_entry: *mut Entry<K, V>) {
        if self.data.len() == 0 {
            self.head = new_entry;
            self.tail = new_entry;
            return;
        };
        (*self.tail).next = new_entry;
        (*new_entry).pre = self.tail;
        (*new_entry).next = null_mut();
        self.tail = new_entry;
    }

    unsafe fn evict(&mut self) -> *mut Entry<K, V> {
        if self.head.is_null() {
            panic!("empty");
        };
        let h = self.head;
        self.head = (*h).next;
        (*self.head).pre = null_mut();
        (*h).next = null_mut();
        return h;
    }
}

#[cfg(test)]
mod test {
    use crate::common::cache::lru::LruCache;

    #[test]
    fn lru_f1() {
        let mut cache = LruCache::new(3);
        cache.put("2", 1);
        cache.put("3", 2);
        cache.put("4", 2);
        cache.put("5", 2);
        cache.put("7", 2);
        assert_eq!(cache.size(), 3);
    }

    #[test]
    fn lru_f2() {
        let mut cache = LruCache::new(3);
        cache.put("2", 1);
        cache.put("3", 2);
        cache.put("4", 4);
        assert_eq!(cache.get(&"2").unwrap(), &1);
    }

    #[test]
    fn lru_f3() {
        let mut cache = LruCache::new(3);
        cache.put("2", 1);
        cache.put("3", 2);
        cache.put("4", 4);
        cache.put("5", 6);
        assert_eq!(cache.get(&"2"), None);
    }

    #[test]
    fn lru_f4() {
        let mut cache = LruCache::new(3);
        cache.put("2", 1);
        cache.print();
        cache.put("3", 2);
        cache.print();
        assert_eq!(cache.get(&"2"), Some(&1));
        cache.print();
        cache.put("4", 4);
        cache.print();
        cache.put("5", 6);
        cache.print();
        assert_eq!(cache.get(&"3"), None);
    }
}
