use alloc::{string::ToString, vec, vec::Vec};
//use arceos_api::modules::axhal::misc::random;

pub struct MyHashMap<K, V> 
where 
    K: ToString + PartialEq + Clone,
    V: Clone,
{
    size: usize, // 键值对数量
    capacity: usize, // map.len
    map: Vec<Vec<(K, V)>>, // 邻接表
}

impl<K, V> MyHashMap<K, V> 
where 
    K: ToString + PartialEq + Clone,
    V: Clone,
{
    pub fn new() -> MyHashMap<K, V> {
        MyHashMap {
            size: 0,
            capacity: 128,
            map: vec![Vec::new(); 128],
        }
    }
    fn hashcode(&self, string: &str) -> usize{
        let mut sum: usize = 0;
        // a0*31^0 + a1*31^1 + ... + ai*31^i
        for byte in string.as_bytes() {
            sum = sum * 31 + *byte as usize;
        }
        sum % self.capacity
    }
    pub fn insert(&mut self, key: K, value: V) {
        // get hash code
        let hashcode = self.hashcode(key.to_string().as_str());

        if let Some(elem) = self.map[hashcode]
            .iter_mut()
            .find(| elem | { elem.0 == key }) { // 检查该键是否已经存在
            elem.1 = value;
        } else {
            self.map[hashcode].push((key, value)); // 插入新键值对
            self.size += 1; // 更新键值对数量
            if self.size as f64 > self.capacity as f64 * 0.75 { // 检查是否需要扩容
                for _ in 0..self.capacity {
                    self.map.push(Vec::new());
                }
                self.capacity *= 2;
            }
        }

    }
    pub fn iter(&self) -> MyHashMapIterator<K, V> {
        let mut elems: Vec<(&K, &V)> = Vec::new();
        for list in self.map.iter() {
            for elem in list.iter() {
                elems.push((&elem.0, &elem.1));
            }
        } 
        MyHashMapIterator {
            current: 0,
            len: elems.len(),
            data: elems
        }
    }
}
pub struct MyHashMapIterator<'a, K, V> {
    current: usize,
    len: usize,
    data: Vec<(&'a K, &'a V)>,
}
impl<'a, K, V> Iterator for MyHashMapIterator<'a, K, V> {
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.len {
            None
        } else {
            self.current += 1;
            Some(self.data[self.current-1])
        }
    }
}