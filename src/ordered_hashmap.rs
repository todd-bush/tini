use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::IntoIterator;

#[derive(Debug)]
pub struct OrderedHashMap<K, V> {
    base: HashMap<K, V>,
    order: Vec<K>,
}

pub struct Iter<'a, K, V> {
    base: &'a HashMap<K, V>,
    order_iterator: std::slice::Iter<'a, K>,
}

pub struct IterMut<'a, K, V> {
    base: &'a mut HashMap<K, V>,
    order_iterator: std::slice::Iter<'a, K>,
}

impl<'a, K, V> IntoIterator for &'a OrderedHashMap<K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            base: &self.base,
            order_iterator: self.order.iter(),
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        match self.order_iterator.next() {
            Some(k) => self.base.get_key_value(&k),
            None => None,
        }
    }
}

impl<'a, K, V> Iterator for IterMut<'a, K, V>
where
    K: Eq + Hash,
{
    type Item = (&'a K, &'a mut V);
    fn next(&mut self) -> Option<Self::Item> {
        match self.order_iterator.next() {
            Some(ref k) => Some((&k, self.base.get_mut(&k).unwrap())),
            None => None,
        }
    }
}

impl<K, V> OrderedHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    pub fn new() -> OrderedHashMap<K, V> {
        OrderedHashMap {
            base: HashMap::<K, V>::new(),
            order: Vec::<K>::new(),
        }
    }
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.base.get(k)
    }
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.order.push(k.clone());
        self.base.insert(k, v)
    }
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            base: &self.base,
            order_iterator: self.order.iter(),
        }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut {
            base: &mut self.base,
            order_iterator: self.order.iter(),
        }
    }
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        self.order.push(key.clone());
        self.base.entry(key)
    }
}
