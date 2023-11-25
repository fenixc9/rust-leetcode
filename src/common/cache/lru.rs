use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::mem::take;
use std::ptr::null_mut;


struct Entry<K, V> {
    pre: *mut Entry<K, V>,
    next: *mut Entry<K, V>,
    k: K,
    v: V,
}

impl<K, V> Entry<K, V> {
    fn new() {}
}

struct LruCache<K, V> {
    data: HashMap<K, *mut Entry<K, V>>,
    head: *mut Entry<K, V>,
    tail: *mut Entry<K, V>,
    cap: usize,
}


impl<K, V> LruCache<K, V>
    where K: Hash + Eq + Copy {
    fn new(cap: usize) -> Self {
        LruCache {
            data: HashMap::new(),
            head: null_mut(),
            tail: null_mut(),
            cap,
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }
    fn get(&mut self, k: &K) -> Option<&V> {
        if !self.data.contains_key(k) { return None; };
        // let x = self.data.get(k).unwrap();
        let x = *self.data.get(k).unwrap();
        unsafe {
            self.moveToLast(x);
            let r = Some(&(*x).v);
            return r;
        }
    }

    fn print(&self)
        where K: Debug, V: Debug
    {
        let mut cur = self.head;
        while !cur.is_null() {
            unsafe {
                println!("k = {:?} v = {:?}", (*cur).k, (*cur).v);
                cur = (*cur).next;
            }
        }
    }

    unsafe fn moveToLast(&mut self, entry: *mut Entry<K, V>) where K: Eq + Hash {
        if self.tail.is_null() { return; }
        let e = entry.read();
        let pre = e.pre;
        let next = e.next;
        if !pre.is_null() { (*pre).next = next; }
        if !next.is_null() { (*next).pre = pre; }
        if self.head == entry {
            self.head = next
        }
        (*self.tail).next = entry;
        (*entry).pre = self.tail;
        (*entry).next = std::ptr::null_mut();
        self.tail = entry;
    }

    fn put(&mut self, k: K, v: V) {
        if self.data.contains_key(&k) {
            let entry = *self.data.get(&k).unwrap();
            unsafe {
                (*entry).v = v;
                self.moveToLast(entry);
                return;
            }
        };
        unsafe {
            if self.data.len() == self.cap {
                let newEntry = self.evict();
                self.data.remove(&(*newEntry).k);
                drop(Box::from_raw(newEntry));
            }
            let newEntry = Box::into_raw(Box::new(Entry {
                pre: null_mut(),
                next: null_mut(),
                k,
                v,
            }));
            self.addNewEntry(newEntry);
            self.data.insert(k, newEntry);
        }
    }

    unsafe fn addNewEntry(&mut self, newEntry: *mut Entry<K, V>) {
        if self.data.len() == 0 {
            self.head = newEntry;
            self.tail = newEntry;
            return;
        };
        (*self.tail).next = newEntry;
        (*newEntry).pre = self.tail;
        (*newEntry).next = null_mut();
        self.tail = newEntry;
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
    fn f1() {
        let mut cache = LruCache::new(10);
        cache.put("2", 1);
        cache.put("3", 2);
        assert_eq!(cache.size(), 2);
    }

    #[test]
    fn f2() {
        let mut cache = LruCache::new(3);
        cache.put("2", 1);
        cache.put("3", 2);
        cache.put("4", 4);
        assert_eq!(cache.get(&"2").unwrap(), &1);
    }

    #[test]
    fn f3() {
        let mut cache = LruCache::new(3);
        cache.put("2", 1);
        cache.put("3", 2);
        cache.put("4", 4);
        cache.put("5", 6);
        assert_eq!(cache.get(&"2"), None);
    }

    #[test]
    fn f4() {
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