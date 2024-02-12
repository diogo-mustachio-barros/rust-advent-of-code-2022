use std::collections::HashMap;

// non-empty tree for simplicity
pub struct AddressableTree<K, V> {
    key: K,
    value: V,
    children: HashMap<K, AddressableTree<K, V>>,
}

impl <K: Eq + core::hash::Hash + PartialEq + Clone, V> AddressableTree<K, V> {
    pub fn singleton(key: K, value: V) -> AddressableTree<K, V> {
        AddressableTree { key: key
                        , value: value
                        , children: HashMap::new() }
    }

    pub fn get_key(&self) -> &K {
        return &self.key
    }

    pub fn get_value(&self) -> &V {
        return &self.value
    }

    pub fn get_children(&self) -> Vec<&AddressableTree<K, V>> {
        let mut ret = Vec::new();

        for (_, value) in &self.children {
            ret.push(value);
        }

        return ret;
    }

    pub fn add_child(&mut self, key: K, value: V) {
        let key_clone = key.clone();
        self.children.insert(key, AddressableTree::singleton(key_clone, value));
    }

    pub fn add_child_node(&mut self, key: K, node:AddressableTree<K, V>) {
        self.children.insert(key, node);
    }

    pub fn remove_child(&mut self, key: &mut K) -> AddressableTree<K, V> {
        self.children.remove(key).unwrap()
    }

    pub fn map_values(&mut self, f: fn(&AddressableTree<K, V>) -> V)
    {
        // map children
        for child in self.children.values_mut() {
            child.map_values(f);
        }

        // map value
        self.value = f(&self);
    }

    pub fn fold<T>(&self, f: &impl Fn(&AddressableTree<K, V>, T) -> T, initial: T) -> T
    {
        let mut acc = initial;

        for child in self.children.values() {
            acc = child.fold(f, acc);
        }

        acc = f(self, acc);

        return acc;
    }
}