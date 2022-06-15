#![feature(map_first_last)]
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Default)]
pub struct TagMap<K, V, T> {
    pub main: HashMap<K, (V, T)>,
    pub tag: BTreeMap<T, K>,
}

impl<K, V, T> TagMap<K, V, T>
where
    K: Eq + Hash + Copy,
    T: Ord + Copy,
{
    pub fn insert(&mut self, key: K, value: V, tag: T) -> Option<(V, T)> {
        let res = self.main.insert(key, (value, tag));
        if let Some((_, t)) = res {
            self.tag.remove(&t);
        };
        self.tag.insert(tag, key);
        res
    }

    pub fn delete(&mut self, key: K) -> Option<(V, T)> {
        let res = self.main.remove(&key);
        if let Some((_, t)) = res {
            self.tag.remove(&t);
        };
        res
    }

    pub fn delete_min_tag(&mut self) -> Option<(K, V, T)> {
        let res = self.tag.pop_first();
        if let Some((_, k)) = res {
            let res2 = self.main.remove(&k);
            match res2 {
                Some((v, t)) => return Some((k, v, t)),
                None => return None,
            }
        }
        None
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        match self.main.get(key) {
            Some((v, _)) => return Some(v),
            None => return None,
        }
    }

    pub fn update(&mut self, key: K, value: V, tag: T) -> Option<(V, T)> {
        self.delete(key);
        self.insert(key, value, tag)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use crate::TagMap;
    use candid::Principal;

    fn new() -> TagMap<Principal, u64, (u64, Option<Principal>)> {
        TagMap {
            main: HashMap::new(),
            tag: BTreeMap::new(),
        }
    }

    #[test]
    fn insert() {
        let mut map = new();
        let a1 = Principal::from_slice(&[1, 2]);
        let a2 = Principal::from_slice(&[2, 2]);
        let a3 = Principal::from_slice(&[3, 2]);
        let a4 = Principal::from_slice(&[4, 2]);
        let a5 = Principal::from_slice(&[5, 2]);
        let min = Principal::from_slice(&[]);
        let max = Principal::from_slice(&[
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ]);

        assert_eq!(map.insert(a1, 100, (100, None)), None);
        assert_eq!(map.insert(a2, 200, (200, None)), None);
        assert_eq!(map.insert(a3, 300, (300, None)), None);
        assert_eq!(map.insert(a4, 400, (400, None)), None);
        assert_eq!(map.insert(a5, 500, (500, Some(a5))), None);
        assert_eq!(map.insert(min, 500, (500, Some(min))), None);
        assert_eq!(map.insert(max, 500, (500, Some(max))), None);

        for i in map.tag.iter() {
            println!("{:?}", i);
        }

        for i in map.main.iter() {
            println!("{:?}", i);
        }
    }

    #[test]
    fn delete_min_tag() {
        let mut map = new();
        let a1 = Principal::from_slice(&[1, 2]);
        let a2 = Principal::from_slice(&[2, 2]);
        let a3 = Principal::from_slice(&[3, 2]);
        let a4 = Principal::from_slice(&[4, 2]);
        let a5 = Principal::from_slice(&[5, 2]);
        let min = Principal::from_slice(&[]);
        let max = Principal::from_slice(&[
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        ]);

        assert_eq!(map.insert(a1, 100, (100, None)), None);
        assert_eq!(map.insert(a2, 200, (200, None)), None);
        assert_eq!(map.insert(a3, 300, (300, None)), None);
        assert_eq!(map.insert(a4, 400, (400, None)), None);
        assert_eq!(map.insert(a5, 500, (500, Some(a5))), None);
        assert_eq!(map.insert(min, 500, (500, Some(min))), None);
        assert_eq!(map.insert(max, 500, (500, Some(max))), None);

        for _ in 0..10 {
            println!("{:?}", map.delete_min_tag());
        }
    }
}
