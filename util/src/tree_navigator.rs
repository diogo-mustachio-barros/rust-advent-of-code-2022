use crate::addressable_tree::AddressableTree;

pub struct TreeNavigator<K, V> {
    previous: Vec<AddressableTree<K, V>>,
    current: AddressableTree<K, V>,
}

impl <K: Eq + core::hash::Hash + PartialEq + Clone, V> TreeNavigator<K, V> {
    pub fn new(tree: AddressableTree<K, V>) -> TreeNavigator<K, V> {
        TreeNavigator {previous: Vec::new(), current: tree}
    }

    pub fn go_into(mut self, key: &mut K) -> Self {
        let mut current = self.current;
        let new_current = current.remove_child(key);

        self.previous.push(current);
        self.current = new_current;

        return self;
    }

    pub fn get_out(mut self) -> Self {
        let current = self.current;
        let mut new_current = self.previous.pop().unwrap();

        new_current.add_child_node(current.get_key().clone(), current);

        self.current = new_current;
        return self;
    }

    pub fn apply_to_current<F>(mut self, f: F) -> Self
    where F:FnOnce(AddressableTree<K, V>) -> AddressableTree<K, V> {
        self.current = f(self.current);
        return self;
    }

    pub fn get(mut self) -> AddressableTree<K, V> {

        // collapse all explored trees back into shape
        while !self.previous.is_empty()
        {
            let mut previous_tree = self.previous.pop().unwrap();

            previous_tree.add_child_node(self.current.get_key().clone(), self.current);

            self.current = previous_tree;
        }
        
        return self.current;
    }
}