use rust_leetcode::common::cache::lru::LruCache;





fn main() {
    let mut cache = LruCache::new(3);
    cache.put("2", 1);
    cache.put("3", 2);
    cache.put("4", 4);
    cache.put("5", 6);
    assert_eq!(cache.get(&"2"), None);
}
