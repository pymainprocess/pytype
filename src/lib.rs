use std::collections::HashMap;

#[cfg(target_pointer_width = "32")]
pub type PyInt = i32;

#[cfg(target_pointer_width = "64")]
pub type PyInt = i64;

#[cfg(target_pointer_width = "32")]
pub type PyFloat = f32;

#[cfg(target_pointer_width = "64")]
pub type PyFloat = f64;

pub struct PyTuple<T1, T2> {
    pub first: T1,
    pub second: T2,
}

impl<T1, T2> PyTuple<T1, T2> {
    pub fn new(first: T1, second: T2) -> Self {
        PyTuple { first, second }
    }

    pub fn to_tuple(self) -> (T1, T2) {
        (self.first, self.second)
    }
}

pub struct PyList<T> {
    pub items: Vec<T>,
}

impl<T> PyList<T> {
    pub fn new() -> Self {
        PyList { items: Vec::new() }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn to_list(self) -> Vec<T> {
        self.items
    }
}

pub struct PyDict<K, V> {
    pub items: HashMap<K, V>,
}

impl<K, V> PyDict<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        PyDict {
            items: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.items.insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.items.get(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.items.remove(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py_dict() {
        let mut py_dict = PyDict::new();
        py_dict.insert("key1", "value1");
        py_dict.insert("key2", "value2");

        assert_eq!(py_dict.get(&"key1"), Some(&"value1"));
        assert_eq!(py_dict.get(&"key2"), Some(&"value2"));

        py_dict.remove(&"key1");
        assert_eq!(py_dict.get(&"key1"), None);
    }

    #[test]
    fn test_py_list() {
        let mut py_list = PyList::new();
        py_list.add("item1");
        py_list.add("item2");

        assert_eq!(py_list.to_list(), vec!["item1", "item2"]);
    }
}