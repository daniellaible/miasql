use crate::database::memstruct::{IndexValue, MemoryStructure, RowId};
use crate::database::table::Row;
use log::{info, warn};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

/// Right now we are using max 3 keys per [Node]. Later on we will optimize this and make it
/// changeable at runtime.
const MAX_KEYS: usize = 3;

/// [Link is just so we don't have to write Arc...Mutext...Node all the time
pub type Link<K> = Arc<Mutex<Node<K>>>;

/// Basic enum type of [Node] in a [BPlusTree]
/// A [Node] can either be of type [Node::Internal] or it can be of type [Node::Leaf]
#[derive(Clone, Debug)]
pub enum Node<K> {
    Internal(InternalNode<K>),
    Leaf(LeafNode<K>),
}

/// An [InternalNode] is a special [Node] in the [BPlusTree], it contains a list of the keys and
/// a list of [Link]s to the children.
#[derive(Clone, Debug)]
pub struct InternalNode<K> {
    pub keys: Vec<K>,
    pub children: Vec<Link<K>>,
}

/// A [LeafNode] is a special [Node] in the [BPlusTree]; it contains a list of all the keys, and
/// a list of all values for the keys. Furthermore, it contains a [Link] the next [LeafNode] to the right.
#[derive(Clone, Debug)]
pub struct LeafNode<K> {
    pub keys: Vec<K>,
    pub values: Vec<Vec<u64>>,
    pub next: Option<Link<K>>,
}

/// This is the basic struct of a [BPlusTree]
/// It is just a [Link} to the root. Link is just a bubble wrapped [Node] which can be of type
/// [InternalNode] or [LeafNode]. So this is not the whole tree, it is just a reference to its root.
#[derive(Clone, Debug)]
pub struct BPlusTree<K> {
    pub root: Link<K>,
    pub len: usize,
}

/// This is the [MemoryStructure] for whole numbers - not for fractions.
/// The underlying structure is a [BPlusTree]. This implementation is used for timestamps.
/// The underlying datatype is u64.
impl MemoryStructure for BPlusTree<u64> {
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::Date(date) => match self.get(&date) {
                None => {
                    let mut keys = Vec::new();
                    keys.push(id);
                    self.insert_into_tree(date, keys);
                }
                Some(mut ids) => {
                    ids.push(id);
                    self.insert_into_tree(date, ids);
                }
            },
            _ => {
                warn!("I expected a date (u64), you gave me something different")
            }
        }
    }

    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
        match key {
            IndexValue::Date(date) => {
                let ids_option = self.get(&date);
                match ids_option {
                    None => Vec::new(),
                    Some(ids) => ids,
                }
            }
            _ => Vec::new(),
        }
    }

    /// I think we went over this already <br>
    /// Don't use this function - use the [HashmapStructure] instead
    fn retrieve_by_index(&self, id: RowId) -> Option<Row> {
        panic!(
            "You really should start to read the comments and not use this function is this context"
        )
    }

    /// Deletes the given id from the [BPlusTree]
    fn delete(&mut self, given_id: RowId, value: Option<IndexValue>) {
        if self.is_empty() {
            return;
        }

        match value {
            Some(value) => match value {
                IndexValue::Date(date) => {
                    let result_option = self.get(&date);
                    match result_option {
                        None => {
                            warn!("Somethings odd here - there should be a result")
                        }
                        Some(result) => {
                            let mut new_id_vec = Vec::new();
                            for i in 0..result.len() {
                                if result[i] != given_id {
                                    new_id_vec.push(result[i]);
                                }
                            }
                            if new_id_vec.len() == 0 {
                                self.remove(&date);
                            } else {
                                self.insert_into_tree(date, new_id_vec);
                            }
                        }
                    }
                }
                _ => {
                    panic!("The value is needed to retrieve the data")
                }
            },
            None => {
                panic!("In the tree we actually need the value")
            }
        }
    }

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }

    fn kind(&self) -> &'static str {
        "tree"
    }
}

impl MemoryStructure for BPlusTree<i8> {
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::TinyInt(tiny) => match self.get(&tiny) {
                None => {
                    let mut keys = Vec::new();
                    keys.push(id);
                    self.insert_into_tree(tiny, keys);
                }
                Some(mut ids) => {
                    ids.push(id);
                    self.insert_into_tree(tiny, ids);
                }
            },
            _ => {
                warn!("I expected a tiny int (i8), you gave me something different")
            }
        }
    }

    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
        match key {
            IndexValue::TinyInt(tiny) => {
                let ids_option = self.get(&tiny);
                match ids_option {
                    None => Vec::new(),
                    Some(ids) => ids,
                }
            }
            _ => Vec::new(),
        }
    }

    fn retrieve_by_index(&self, id: RowId) -> Option<Row> {
        panic!(
            "You really should start to read the comments and not use this function is this context"
        )
    }

    fn delete(&mut self, given_id: RowId, value: Option<IndexValue>) {
        if self.is_empty() {
            return;
        }

        match value {
            Some(value) => match value {
                IndexValue::TinyInt(tiny) => {
                    let result_option = self.get(&tiny);
                    match result_option {
                        None => {
                            warn!("Somethings odd here - there should be a result")
                        }
                        Some(result) => {
                            let mut new_id_vec = Vec::new();
                            for i in 0..result.len() {
                                if result[i] != given_id {
                                    new_id_vec.push(result[i]);
                                }
                            }
                            if new_id_vec.len() == 0 {
                                self.remove(&tiny);
                            } else {
                                self.insert_into_tree(tiny, new_id_vec);
                            }
                        }
                    }
                }
                _ => {
                    panic!("The value is needed to retrieve the data")
                }
            },
            None => {
                panic!("In the tree we actually need the value")
            }
        }
    }

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }

    fn kind(&self) -> &'static str {
        "tree"
    }
}

impl MemoryStructure for BPlusTree<i16> {
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::SmallInt(small) => match self.get(&small) {
                None => {
                    let mut keys = Vec::new();
                    keys.push(id);
                    self.insert_into_tree(small, keys);
                }
                Some(mut ids) => {
                    ids.push(id);
                    self.insert_into_tree(small, ids);
                }
            },
            _ => {
                warn!("I expected a smallint (i16), you gave me something different")
            }
        }
    }

    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
        match key {
            IndexValue::SmallInt(small) => {
                let ids_option = self.get(&small);
                match ids_option {
                    None => Vec::new(),
                    Some(ids) => ids,
                }
            }
            _ => Vec::new(),
        }
    }

    fn retrieve_by_index(&self, id: RowId) -> Option<Row> {
        panic!(
            "You really should start to read the comments and not use this function is this context"
        )
    }

    fn delete(&mut self, given_id: RowId, value: Option<IndexValue>) {
        if self.is_empty() {
            return;
        }

        match value {
            Some(value) => match value {
                IndexValue::SmallInt(date) => {
                    let result_option = self.get(&date);
                    match result_option {
                        None => {
                            warn!("Somethings odd here - there should be a result")
                        }
                        Some(result) => {
                            let mut new_id_vec = Vec::new();
                            for i in 0..result.len() {
                                if result[i] != given_id {
                                    new_id_vec.push(result[i]);
                                }
                            }
                            if new_id_vec.len() == 0 {
                                self.remove(&date);
                            } else {
                                self.insert_into_tree(date, new_id_vec);
                            }
                        }
                    }
                }
                _ => {
                    panic!("The value is needed to retrieve the data")
                }
            },
            None => {
                panic!("In the tree we actually need the value")
            }
        }
    }

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }

    fn kind(&self) -> &'static str {
        "tree"
    }
}

impl MemoryStructure for BPlusTree<i32> {
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::Int(inty) => match self.get(&inty) {
                None => {
                    let mut keys = Vec::new();
                    keys.push(id);
                    self.insert_into_tree(inty, keys);
                }
                Some(mut ids) => {
                    ids.push(id);
                    self.insert_into_tree(inty, ids);
                }
            },
            _ => {
                warn!("I expected an int (i64), you gave me something different")
            }
        }
    }

    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
        match key {
            IndexValue::Int(inty) => {
                let ids_option = self.get(&inty);
                match ids_option {
                    None => Vec::new(),
                    Some(ids) => ids,
                }
            }
            _ => Vec::new(),
        }
    }

    fn retrieve_by_index(&self, id: RowId) -> Option<Row> {
        panic!(
            "You really should start to read the comments and not use this function is this context"
        )
    }

    fn delete(&mut self, given_id: RowId, value: Option<IndexValue>) {
        if self.is_empty() {
            return;
        }

        match value {
            Some(value) => match value {
                IndexValue::Int(inty) => {
                    let result_option = self.get(&inty);
                    match result_option {
                        None => {
                            warn!("Somethings odd here - there should be a result")
                        }
                        Some(result) => {
                            let mut new_id_vec = Vec::new();
                            for i in 0..result.len() {
                                if result[i] != given_id {
                                    new_id_vec.push(result[i]);
                                }
                            }
                            if new_id_vec.len() == 0 {
                                self.remove(&inty);
                            } else {
                                self.insert_into_tree(inty, new_id_vec);
                            }
                        }
                    }
                }
                _ => {
                    panic!("The value is needed to retrieve the data")
                }
            },
            None => {
                panic!("In the tree we actually need the value")
            }
        }
    }

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }

    fn kind(&self) -> &'static str {
        "tree"
    }
}

impl MemoryStructure for BPlusTree<i64> {
    fn insert(&mut self, value: IndexValue, id: RowId) {
        match value {
            IndexValue::BigInt(biggy) => match self.get(&biggy) {
                None => {
                    let mut keys = Vec::new();
                    keys.push(id);
                    self.insert_into_tree(biggy, keys);
                }
                Some(mut ids) => {
                    ids.push(id);
                    self.insert_into_tree(biggy, ids);
                }
            },
            _ => {
                warn!("I expected a bigint (i64), you gave me something different")
            }
        }
    }

    fn retrieve_range(&self, key: &IndexValue) -> Vec<RowId> {
        match key {
            IndexValue::BigInt(biggy) => {
                let ids_option = self.get(&biggy);
                match ids_option {
                    None => Vec::new(),
                    Some(ids) => ids,
                }
            }
            _ => Vec::new(),
        }
    }

    fn retrieve_by_index(&self, id: RowId) -> Option<Row> {
        panic!(
            "You really should start to read the comments and not use this function is this context"
        )
    }

    fn delete(&mut self, given_id: RowId, value: Option<IndexValue>) {
        if self.is_empty() {
            return;
        }

        match value {
            Some(value) => match value {
                IndexValue::BigInt(biggy) => {
                    let result_option = self.get(&biggy);
                    match result_option {
                        None => {
                            warn!("Somethings odd here - there should be a result")
                        }
                        Some(result) => {
                            let mut new_id_vec = Vec::new();
                            for i in 0..result.len() {
                                if result[i] != given_id {
                                    new_id_vec.push(result[i]);
                                }
                            }
                            if new_id_vec.len() == 0 {
                                self.remove(&biggy);
                            } else {
                                self.insert_into_tree(biggy, new_id_vec);
                            }
                        }
                    }
                }
                _ => {
                    panic!("The value is needed to retrieve the data")
                }
            },
            None => {
                panic!("In the tree we actually need the value")
            }
        }
    }

    fn clone_box(&self) -> Box<dyn MemoryStructure> {
        Box::new(self.clone())
    }

    fn kind(&self) -> &'static str {
        "tree"
    }
}

impl<K> Default for BPlusTree<K>
where
    K: Ord + Clone + Send,
{
    fn default() -> Self {
        let root = Arc::new(Mutex::new(Node::Leaf(LeafNode {
            keys: Vec::new(),
            values: Vec::new(),
            next: None,
        })));
        Self { root, len: 0 }
    }
}

impl<K> BPlusTree<K>
where
    K: Ord + Clone,
{
    fn min_keys() -> usize {
        (MAX_KEYS + 1) / 2
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, key: &K) -> Option<Vec<u64>> {
        let leaf = self.find_leaf(self.root.clone(), key);
        let leaf_guard = leaf.lock().unwrap();
        let Node::Leaf(ln) = &*leaf_guard else {
            unreachable!("find_leaf must return leaf");
        };
        match ln.keys.binary_search(key) {
            Ok(i) => Some(ln.values[i].clone()),
            Err(_) => None,
        }
    }

    pub fn insert_into_tree(&mut self, key: K, value: Vec<u64>) -> Option<Vec<u64>> {
        let mut path = Vec::<(Link<K>, usize)>::new();
        let leaf = self.find_leaf_with_path(self.root.clone(), &key, &mut path);

        let mut leaf_mut = leaf.lock().unwrap();
        let Node::Leaf(ln) = &mut *leaf_mut else {
            unreachable!()
        };

        match ln.keys.binary_search(&key) {
            Ok(i) => {
                let old = std::mem::replace(&mut ln.values[i], value);
                return Some(old);
            }
            Err(pos) => {
                ln.keys.insert(pos, key);
                ln.values.insert(pos, value);
                self.len += 1;
            }
        }

        if ln.keys.len() <= MAX_KEYS {
            return None;
        }

        // Split leaf and propagate.
        drop(leaf_mut);
        let (sep_key, new_right) = self.split_leaf(leaf.clone());

        self.insert_in_parent(path, leaf, sep_key, new_right);
        None
    }

    /// Remove a key. Returns removed value if present.
    pub fn remove(&mut self, key: &K) -> Option<Vec<u64>> {
        let mut path = Vec::<(Link<K>, usize)>::new();
        let leaf = self.find_leaf_with_path(self.root.clone(), key, &mut path);

        // Remove from leaf.
        let mut leaf_mut = leaf.lock().unwrap();
        let Node::Leaf(ln) = &mut *leaf_mut else {
            unreachable!()
        };

        let idx = match ln.keys.binary_search(key) {
            Ok(i) => i,
            Err(_) => return None,
        };

        ln.keys.remove(idx);
        let removed = ln.values.remove(idx);
        self.len -= 1;

        // If leaf is root, done (root may be empty).
        if Arc::ptr_eq(&leaf, &self.root) {
            return Some(removed);
        }

        let leaf_first_key = ln.keys.first().cloned();
        drop(leaf_mut);

        if let Some((parent, child_index_in_parent)) = path.last().cloned() {
            if idx == 0 {
                if let Some(new_first) = leaf_first_key {
                    self.update_parent_separator_key(
                        parent.clone(),
                        child_index_in_parent,
                        new_first,
                    );
                }
            }
        }

        // Rebalance if underflow.
        self.rebalance_after_delete(path, leaf);

        Some(removed)
    }

    pub fn leaf_walker_rows(&self, leaf: LeafNode<K>) -> anyhow::Result<Vec<Vec<u64>>> {
        let mut result_vec: Vec<Vec<u64>> = Vec::new();
        let mut cur: Option<LeafNode<K>> = Some(leaf);

        while let Some(ln) = cur {
            result_vec.extend(ln.values.clone());

            cur = match ln.next.clone() {
                Some(next_link) => {
                    let next_guard = next_link.lock().unwrap();
                    let Node::Leaf(next_leaf) = &*next_guard else {
                        unreachable!("leaf.next must point to a leaf");
                    };
                    Some(next_leaf.clone())
                }
                None => None,
            };
        }
        Ok(result_vec)
    }

    pub fn range(&self, start: Option<&K>, end: Option<&K>) -> Vec<(K, Vec<u64>)> {
        let mut out = Vec::new();

        let leaf = match start {
            Some(k) => self.find_leaf(self.root.clone(), k),
            None => self.leftmost_leaf(self.root.clone()),
        };

        let mut cur = Some(leaf);
        while let Some(node) = cur {
            let b = node.lock().unwrap();
            let Node::Leaf(ln) = &*b else { unreachable!() };

            let start_i = match start {
                Some(s) => match ln.keys.binary_search(s) {
                    Ok(i) => i,
                    Err(i) => i,
                },
                None => 0,
            };

            for i in start_i..ln.keys.len() {
                if let Some(e) = end {
                    if ln.keys[i].cmp(e) != Ordering::Less {
                        return out;
                    }
                }
                out.push((ln.keys[i].clone(), ln.values[i].clone()));
            }
            cur = ln.next.clone();
        }
        out
    }

    pub fn leftmost_leaf(&self, mut node: Link<K>) -> Link<K> {
        loop {
            let b = node.lock().unwrap();
            match &*b {
                Node::Leaf(_) => {
                    drop(b); // end the borrow explicitly
                    return node; // now it's legal
                }
                Node::Internal(internal) => {
                    let next = internal.children[0].clone();
                    drop(b); // end borrow before reassign
                    node = next;
                }
            }
        }
    }

    // -------------------------
    // Search helpers
    // -------------------------

    fn find_leaf(&self, mut node: Link<K>, key: &K) -> Link<K> {
        loop {
            let b = node.lock().unwrap();
            match &*b {
                Node::Leaf(_) => {
                    drop(b);
                    return node;
                }
                Node::Internal(internal) => {
                    let child_idx = child_index_for_key(&internal.keys, key);
                    let next = internal.children[child_idx].clone();
                    drop(b);
                    node = next;
                }
            }
        }
    }

    fn find_leaf_with_path(
        &self,
        mut node: Link<K>,
        key: &K,
        path: &mut Vec<(Link<K>, usize)>,
    ) -> Link<K> {
        loop {
            let next = {
                let b = node.lock().unwrap();
                match &*b {
                    Node::Leaf(_) => None,
                    Node::Internal(internal) => {
                        let child_idx = child_index_for_key(&internal.keys, key);
                        let next = internal.children[child_idx].clone();
                        // record parent + which child we took
                        Some((next, child_idx))
                    }
                }
            };

            match next {
                None => return node, // no active borrow here
                Some((next_node, child_idx)) => {
                    path.push((node.clone(), child_idx));
                    node = next_node;
                }
            }
        }
    }

    // -------------------------
    // Insert split / propagate
    // -------------------------

    fn split_leaf(&self, leaf: Link<K>) -> (K, Link<K>) {
        let mut leaf_mut = leaf.lock().unwrap();
        let Node::Leaf(ln) = &mut *leaf_mut else {
            unreachable!()
        };

        // Split roughly in half; right gets the larger half.
        let mid = ln.keys.len() / 2;

        let right_keys = ln.keys.split_off(mid);
        let right_vals = ln.values.split_off(mid);

        let sep_key = right_keys[0].clone();

        let right = Arc::new(Mutex::new(Node::Leaf(LeafNode {
            keys: right_keys,
            values: right_vals,
            next: ln.next.take(),
        })));

        ln.next = Some(right.clone());

        (sep_key, right)
    }

    fn split_internal(&self, internal: Link<K>) -> (K, Link<K>) {
        let mut nmut = internal.lock().unwrap();
        let Node::Internal(inode) = &mut *nmut else {
            unreachable!()
        };

        let mid_key_index = inode.keys.len() / 2;
        let promoted = inode.keys[mid_key_index].clone();

        let right_keys = inode.keys.split_off(mid_key_index + 1);
        inode.keys.pop(); // remove promoted from left

        let right_children = inode.children.split_off(mid_key_index + 1);

        let right = Arc::new(Mutex::new(Node::Internal(InternalNode {
            keys: right_keys,
            children: right_children,
        })));

        (promoted, right)
    }

    fn insert_in_parent(
        &mut self,
        mut path: Vec<(Link<K>, usize)>,
        left: Link<K>,
        sep_key: K,
        right: Link<K>,
    ) {
        // If no parent, create new root.
        let Some((parent, left_index)) = path.pop() else {
            let new_root = Arc::new(Mutex::new(Node::Internal(InternalNode {
                keys: vec![sep_key],
                children: vec![left, right],
            })));
            self.root = new_root;
            return;
        };

        {
            let mut pb = parent.lock().unwrap();
            let Node::Internal(pn) = &mut *pb else {
                unreachable!()
            };

            pn.keys.insert(left_index, sep_key);
            pn.children.insert(left_index + 1, right);
        }

        // If parent overflows, split and propagate up.
        let overflow = {
            let pb = parent.lock().unwrap();
            let Node::Internal(pn) = &*pb else {
                unreachable!()
            };
            pn.keys.len() > MAX_KEYS
        };

        if !overflow {
            return;
        }

        let (promoted, new_right) = self.split_internal(parent.clone());
        self.insert_in_parent(path, parent, promoted, new_right);
    }

    // -------------------------
    // Delete rebalance
    // -------------------------

    fn rebalance_after_delete(&mut self, mut path: Vec<(Link<K>, usize)>, mut node: Link<K>) {
        loop {
            // If node is root, shrink root if possible.
            if Arc::ptr_eq(&node, &self.root) {
                self.maybe_shrink_root();
                return;
            }

            let min_keys = Self::min_keys();

            let node_key_count = {
                let nb = node.lock().unwrap();
                match &*nb {
                    Node::Leaf(ln) => ln.keys.len(),
                    Node::Internal(inode) => inode.keys.len(),
                }
            };

            if node_key_count >= min_keys {
                return;
            }

            let Some((parent, idx_in_parent)) = path.pop() else {
                return;
            };

            // Try redistribute from siblings first.
            if self.try_redistribute(&parent, idx_in_parent) {
                return;
            }

            let merged_into_left = self.merge_with_sibling(&parent, idx_in_parent);

            node = if merged_into_left {
                parent.clone()
            } else {
                parent.clone()
            };
        }
    }

    fn try_redistribute(&self, parent: &Link<K>, idx: usize) -> bool {
        let (left_sib, right_sib) = {
            let pb = parent.lock().unwrap();
            let Node::Internal(pn) = &*pb else {
                unreachable!()
            };

            let left = if idx > 0 {
                Some(pn.children[idx - 1].clone())
            } else {
                None
            };
            let right = if idx + 1 < pn.children.len() {
                Some(pn.children[idx + 1].clone())
            } else {
                None
            };
            (left, right)
        };

        let min_keys = Self::min_keys();

        // Try left -> current
        if let Some(left) = left_sib {
            let left_count = node_key_len(&left);
            if left_count > min_keys {
                self.redistribute_from_left(parent.clone(), idx, left);
                return true;
            }
        }

        // Try right -> current
        if let Some(right) = right_sib {
            let right_count = node_key_len(&right);
            if right_count > min_keys {
                self.redistribute_from_right(parent.clone(), idx, right);
                return true;
            }
        }

        false
    }

    fn redistribute_from_left(&self, parent: Link<K>, idx: usize, left: Link<K>) {
        let mut pb = parent.lock().unwrap();
        let Node::Internal(pn) = &mut *pb else {
            unreachable!()
        };

        let cur = pn.children[idx].clone();

        match (&mut *left.lock().unwrap(), &mut *cur.lock().unwrap()) {
            (Node::Leaf(ln_left), Node::Leaf(ln_cur)) => {
                // Move last key/value from left to front of current.
                let k = ln_left.keys.pop().expect("left has keys");
                let v = ln_left.values.pop().expect("left has values");

                ln_cur.keys.insert(0, k.clone());
                ln_cur.values.insert(0, v);

                // Update parent separator for current: parent key at idx-1 becomes current first key.
                pn.keys[idx - 1] = ln_cur.keys[0].clone();
            }
            (Node::Internal(in_left), Node::Internal(in_cur)) => {
                let sep_down = pn.keys[idx - 1].clone();

                let borrowed_key = in_left.keys.pop().expect("left has keys");
                let borrowed_child = in_left.children.pop().expect("left has child");

                pn.keys[idx - 1] = borrowed_key;

                in_cur.keys.insert(0, sep_down);
                in_cur.children.insert(0, borrowed_child);
            }
            _ => unreachable!("tree nodes at same level must have same variant"),
        }
    }

    fn redistribute_from_right(&self, parent: Link<K>, idx: usize, right: Link<K>) {
        let mut pb = parent.lock().unwrap();
        let Node::Internal(pn) = &mut *pb else {
            unreachable!()
        };

        let cur = pn.children[idx].clone();

        match (&mut *cur.lock().unwrap(), &mut *right.lock().unwrap()) {
            (Node::Leaf(ln_cur), Node::Leaf(ln_right)) => {
                // Move first key/value from right to end of current.
                let k = ln_right.keys.remove(0);
                let v = ln_right.values.remove(0);

                ln_cur.keys.push(k.clone());
                ln_cur.values.push(v);

                // Update parent separator for right: parent key at idx becomes right first key.
                pn.keys[idx] = ln_right.keys[0].clone();
            }
            (Node::Internal(in_cur), Node::Internal(in_right)) => {
                let sep_down = pn.keys[idx].clone();

                let borrowed_key = in_right.keys.remove(0);
                let borrowed_child = in_right.children.remove(0);

                pn.keys[idx] = borrowed_key;

                in_cur.keys.push(sep_down);
                in_cur.children.push(borrowed_child);
            }
            _ => unreachable!("tree nodes at same level must have same variant"),
        }
    }

    fn merge_with_sibling(&self, parent: &Link<K>, idx: usize) -> bool {
        let mut pb = parent.lock().unwrap();
        let Node::Internal(pn) = &mut *pb else {
            unreachable!()
        };

        if idx > 0 {
            // Merge current into left sibling
            let left = pn.children[idx - 1].clone();
            let cur = pn.children[idx].clone();
            let sep_key = pn.keys.remove(idx - 1);
            pn.children.remove(idx);

            drop(pb); // avoid double borrow during merge

            self.merge_nodes(left, cur, Some(sep_key));
            true
        } else {
            let cur = pn.children[idx].clone();
            let right = pn.children[idx + 1].clone();
            let sep_key = pn.keys.remove(idx);
            pn.children.remove(idx + 1);

            drop(pb);

            self.merge_nodes(cur, right, Some(sep_key));
            false
        }
    }

    fn merge_nodes(&self, left: Link<K>, right: Link<K>, sep_key_for_internal: Option<K>) {
        match (&mut *left.lock().unwrap(), &mut *right.lock().unwrap()) {
            (Node::Leaf(ln_left), Node::Leaf(ln_right)) => {
                ln_left.keys.extend(ln_right.keys.drain(..));
                ln_left.values.extend(ln_right.values.drain(..));
                ln_left.next = ln_right.next.take();
            }
            (Node::Internal(in_left), Node::Internal(in_right)) => {
                let sep = sep_key_for_internal.expect("internal merge needs separator key");
                in_left.keys.push(sep);
                in_left.keys.extend(in_right.keys.drain(..));
                in_left.children.extend(in_right.children.drain(..));
            }
            _ => unreachable!("merge nodes must be same type"),
        }
    }

    fn maybe_shrink_root(&mut self) {
        let shrink_to = {
            let rb = self.root.lock().unwrap();
            match &*rb {
                Node::Leaf(_) => None,
                Node::Internal(inode) => {
                    if inode.keys.is_empty() && inode.children.len() == 1 {
                        Some(inode.children[0].clone())
                    } else {
                        None
                    }
                }
            }
        };

        if let Some(new_root) = shrink_to {
            self.root = new_root;
        }
    }

    fn update_parent_separator_key(&self, parent: Link<K>, child_index: usize, new_first_key: K) {
        // For child at index i>0, parent's key at i-1 should equal child's first key.
        if child_index == 0 {
            return;
        }
        let mut pb = parent.lock().unwrap();
        let Node::Internal(pn) = &mut *pb else {
            unreachable!()
        };
        pn.keys[child_index - 1] = new_first_key;
    }
}

// -------------------------
// Utility functions
// -------------------------

fn child_index_for_key<K: Ord>(keys: &[K], key: &K) -> usize {
    // For internal keys [k0,k1,...], children are:
    // child0: < k0
    // child1: >=k0 and <k1
    // ...
    // childN: >= k_{N-1}
    match keys.binary_search(key) {
        Ok(i) => i + 1,
        Err(i) => i,
    }
}

fn node_key_len<K>(n: &Link<K>) -> usize {
    let b = n.lock().unwrap();
    match &*b {
        Node::Leaf(ln) => ln.keys.len(),
        Node::Internal(inode) => inode.keys.len(),
    }
}

// -------------------------
// Optional: basic validation (debug / tests)
// -------------------------

impl<K> BPlusTree<K>
where
    K: Ord + Clone + Debug,
{
    pub fn validate(&self) {
        let min_keys = Self::min_keys();
        self.validate_node(self.root.clone(), true, min_keys);
        self.validate_leaf_chain();
    }

    fn validate_node(&self, node: Link<K>, is_root: bool, min_keys: usize) -> Option<(K, K)> {
        let b = node.lock().unwrap();
        match &*b {
            Node::Leaf(ln) => {
                assert_eq!(ln.keys.len(), ln.values.len(), "leaf keys/values mismatch");
                assert!(ln.keys.len() <= MAX_KEYS, "leaf overflow");
                if !is_root {
                    assert!(ln.keys.len() >= min_keys, "leaf underflow");
                }

                for w in ln.keys.windows(2) {
                    assert!(
                        w[0] < w[1],
                        "leaf keys not strictly increasing: {:?} >= {:?}",
                        w[0],
                        w[1]
                    );
                }

                if ln.keys.is_empty() {
                    // Valid only for an empty tree (root leaf).
                    assert!(is_root, "non-root empty leaf");
                    return None;
                }

                Some((
                    ln.keys.first().unwrap().clone(),
                    ln.keys.last().unwrap().clone(),
                ))
            }

            Node::Internal(inode) => {
                assert!(inode.keys.len() >= MAX_KEYS / 2, "internal underflow");
                assert_eq!(
                    inode.children.len(),
                    inode.keys.len() + 1,
                    "internal arity mismatch"
                );
                if !is_root {
                    assert!(inode.keys.len() >= MAX_KEYS / 2, "internal underflow");
                }

                for w in inode.keys.windows(2) {
                    assert!(
                        w[0] < w[1],
                        "internal keys not strictly increasing: {:?} >= {:?}",
                        w[0],
                        w[1]
                    );
                }

                // Validate children
                let mut child_ranges: Vec<(K, K)> = Vec::with_capacity(inode.children.len());
                for ch in &inode.children {
                    let bounds = self
                        .validate_node(ch.clone(), false, min_keys)
                        .expect("internal node cannot have empty child subtree");
                    child_ranges.push(bounds);
                }

                // Boundary checks (recommended invariant)
                for i in 0..inode.keys.len() {
                    let sep = &inode.keys[i];
                    let (_lmin, lmax) = &child_ranges[i];
                    let (rmin, _rmax) = &child_ranges[i + 1];

                    assert!(
                        lmax < sep,
                        "separator boundary violated at key index {i}: left_max={lmax:?} !< sep={sep:?}"
                    );
                    assert!(
                        rmin >= sep,
                        "separator boundary violated at key index {i}: right_min={rmin:?} !>= sep={sep:?}"
                    );
                }

                let min = child_ranges.first().unwrap().0.clone();
                let max = child_ranges.last().unwrap().1.clone();
                Some((min, max))
            }
        }
    }

    fn validate_leaf_chain(&self) {
        // Walk leaf chain from leftmost and ensure sorted global order.
        let start = self.leftmost_leaf(self.root.clone());
        let mut cur = Some(start);
        let mut last_key: Option<K> = None;

        while let Some(n) = cur {
            let b = n.lock().unwrap();
            let Node::Leaf(ln) = &*b else { unreachable!() };

            for k in &ln.keys {
                if let Some(prev) = &last_key {
                    assert!(prev < k, "leaf chain not increasing: {:?} >= {:?}", prev, k);
                }
                last_key = Some(k.clone());
            }

            cur = ln.next.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BPlusTree, Node};
    use crate::database::memstruct::{IndexValue, MemoryStructure};

    // ----------------------------------------
    // MemoryStructure Tests
    // ----------------------------------------
    #[test]
    fn insert_into_tree_including_duplicates_test() {
        let mut tree: BPlusTree<u64> = BPlusTree::default();
        tree.insert(IndexValue::Date(100), 1);
        tree.insert(IndexValue::Date(100), 7);
        tree.insert(IndexValue::Date(101), 2);
        tree.insert(IndexValue::Date(102), 3);
        tree.insert(IndexValue::Date(103), 4);
        tree.insert(IndexValue::Date(104), 5);
        tree.insert(IndexValue::Date(105), 6);
        let guard = tree.leftmost_leaf(tree.root.clone());
        let guarded = guard.lock().unwrap();
        let Node::Leaf(leaf) = &*guarded else {
            unreachable!()
        };
        let keys = &leaf.keys;
        let values = &leaf.values;
        assert_eq!(keys[0], 100);
        assert_eq!(keys[1], 101);
        assert_eq!(values[0][0], 1);
        assert_eq!(values[0][1], 7);
        assert_eq!(values[1][0], 2);
        println!("{:?}", leaf);
    }

    #[test]
    fn delete_from_memorystruct() {
        let mut tree: BPlusTree<u64> = BPlusTree::default();
        tree.insert(IndexValue::Date(100), 1);
        tree.insert(IndexValue::Date(100), 7);
        tree.insert(IndexValue::Date(101), 2);
        tree.insert(IndexValue::Date(102), 3);
        tree.insert(IndexValue::Date(103), 4);
        tree.insert(IndexValue::Date(104), 5);
        tree.insert(IndexValue::Date(105), 6);

        tree.delete(2, Some(IndexValue::Date(101)));
        tree.delete(7, Some(IndexValue::Date(100)));

        let guard = tree.leftmost_leaf(tree.root.clone());
        let guarded = guard.lock().unwrap();
        let Node::Leaf(leaf) = &*guarded else {
            unreachable!()
        };
        let keys = &leaf.keys;
        let values = &leaf.values;
        assert_eq!(keys[0], 100);
        assert_eq!(keys[1], 102);
        assert_eq!(keys[2], 103);
        assert_eq!(values[0][0], 1);
        assert_eq!(values[1][0], 3);
        assert_eq!(values[2][0], 4);
    }

    // ----------------------------------------
    // BPlusTree Tests
    // ----------------------------------------

    fn build(keys: &[i64]) -> BPlusTree<i64> {
        let mut t = BPlusTree::default();
        for &k in keys {
            t.insert_into_tree(k, vec![k as u64]);
        }
        t
    }

    #[test]
    fn default_tree_is_empty() {
        let t: BPlusTree<i64> = BPlusTree::default();
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn default_tree_get_returns_none() {
        let t: BPlusTree<i64> = BPlusTree::default();
        assert_eq!(t.get(&0), None);
        assert_eq!(t.get(&99), None);
    }

    #[test]
    fn default_tree_range_returns_empty() {
        let t: BPlusTree<i64> = BPlusTree::default();
        assert!(t.range(None, None).is_empty());
        assert!(t.range(Some(&0), Some(&100)).is_empty());
    }

    #[test]
    fn single_insert_get() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        let old = t.insert_into_tree(42, vec![1, 2]);
        assert_eq!(old, None);
        assert_eq!(t.len(), 1);
        assert!(!t.is_empty());
        assert_eq!(t.get(&42), Some(vec![1, 2]));
        assert_eq!(t.get(&0), None);
    }

    #[test]
    fn insert_replaces_existing_value_returns_old() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        assert_eq!(t.insert_into_tree(10, vec![1]), None);
        assert_eq!(t.len(), 1);

        let old = t.insert_into_tree(10, vec![2, 3]);
        assert_eq!(old, Some(vec![1]));
        // Length does not change on replacement.
        assert_eq!(t.len(), 1);
        assert_eq!(t.get(&10), Some(vec![2, 3]));
    }

    #[test]
    fn insert_empty_vec_as_value() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        assert_eq!(t.insert_into_tree(5, vec![]), None);
        assert_eq!(t.get(&5), Some(vec![]));
    }

    #[test]
    fn insert_multiple_row_ids_per_key() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        t.insert_into_tree(7, vec![10, 20, 30]);
        assert_eq!(t.get(&7), Some(vec![10, 20, 30]));
    }

    #[test]
    fn insert_increasing_keys_len_correct() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        for k in 0..50_i64 {
            assert_eq!(t.insert_into_tree(k, vec![k as u64]), None);
        }
        assert_eq!(t.len(), 50);
        for k in 0..50_i64 {
            assert_eq!(t.get(&k), Some(vec![k as u64]));
        }
    }

    #[test]
    fn insert_decreasing_keys_len_correct() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        for k in (0..50_i64).rev() {
            assert_eq!(t.insert_into_tree(k, vec![k as u64]), None);
        }
        assert_eq!(t.len(), 50);
        for k in 0..50_i64 {
            assert_eq!(t.get(&k), Some(vec![k as u64]));
        }
    }

    #[test]
    fn insert_triggers_splits_validate_passes() {
        // MAX_KEYS = 3, so after 4 inserts a split must have happened.
        let mut t: BPlusTree<i64> = BPlusTree::default();
        for k in 0..20_i64 {
            t.insert_into_tree(k, vec![k as u64]);
        }
        t.validate(); // panics on structural violation
    }

    #[test]
    fn insert_many_interleaved_keys_validate_passes() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        // Alternate low / high insertions to exercise different split paths.
        for i in 0..100_i64 {
            t.insert_into_tree(i, vec![i as u64]);
            t.insert_into_tree(10_000 - i, vec![(10_000 - i) as u64]);
        }
        assert_eq!(t.len(), 200);
        t.validate();
        // Spot-check
        assert_eq!(t.get(&0), Some(vec![0]));
        assert_eq!(t.get(&50), Some(vec![50]));
        assert_eq!(t.get(&9950), Some(vec![9950]));
        assert_eq!(t.get(&10_000), Some(vec![10_000]));
    }

    #[test]
    fn insert_negative_keys() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        for k in [-5_i64, -3, -1, 0, 1, 3, 5] {
            t.insert_into_tree(k, vec![k.unsigned_abs()]);
        }
        assert_eq!(t.len(), 7);
        t.validate();
        assert_eq!(t.get(&-3), Some(vec![3]));
        assert_eq!(t.get(&0), Some(vec![0]));
        assert_eq!(t.get(&5), Some(vec![5]));
    }

    #[test]
    fn remove_missing_key_returns_none() {
        let mut t = build(&[1, 2, 3]);
        assert_eq!(t.remove(&99), None);
        assert_eq!(t.remove(&0), None);
        assert_eq!(t.len(), 3);
    }

    #[test]
    fn remove_only_element() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        t.insert_into_tree(42, vec![1]);
        let v = t.remove(&42);
        assert_eq!(v, Some(vec![1]));
        assert_eq!(t.len(), 0);
        assert!(t.is_empty());
        assert_eq!(t.get(&42), None);
    }

    #[test]
    fn remove_returns_correct_value() {
        let mut t = build(&[10, 20, 30]);
        let v = t.remove(&20);
        assert_eq!(v, Some(vec![20]));
        assert_eq!(t.len(), 2);
        assert_eq!(t.get(&20), None);
        assert_eq!(t.get(&10), Some(vec![10]));
        assert_eq!(t.get(&30), Some(vec![30]));
    }

    #[test]
    fn remove_first_key_in_leaf_updates_structure() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        // Insert enough to cause splits (MAX_KEYS=3, so >=4 triggers one).
        for k in 0..20_i64 {
            t.insert_into_tree(k, vec![k as u64]);
        }
        // Remove keys in ascending order (repeatedly removes first key in leaves).
        for k in 0..20_i64 {
            assert_eq!(t.remove(&k), Some(vec![k as u64]));
            assert_eq!(t.get(&k), None);
        }
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn remove_last_key_in_leaf_updates_structure() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        for k in 0..20_i64 {
            t.insert_into_tree(k, vec![k as u64]);
        }
        // Remove in descending order (repeatedly removes last key in leaves).
        for k in (0..20_i64).rev() {
            assert_eq!(t.remove(&k), Some(vec![k as u64]));
        }
        assert_eq!(t.len(), 0);
    }

    #[test]
    fn remove_all_keys_then_reinsert() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        for k in 0..30_i64 {
            t.insert_into_tree(k, vec![k as u64]);
        }
        for k in 0..30_i64 {
            t.remove(&k);
        }
        assert!(t.is_empty());

        // Tree should be reusable after being emptied.
        for k in 100..130_i64 {
            assert_eq!(t.insert_into_tree(k, vec![k as u64]), None);
        }
        assert_eq!(t.len(), 30);
        t.validate();
        for k in 100..130_i64 {
            assert_eq!(t.get(&k), Some(vec![k as u64]));
        }
    }

    #[test]
    fn remove_triggers_redistribute_and_validate_passes() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        for k in 0..50_i64 {
            t.insert_into_tree(k, vec![k as u64]);
        }
        t.validate();
        // Remove every other key to trigger redistribute/merge paths.
        for k in (0..50_i64).step_by(2) {
            t.remove(&k);
        }
        t.validate();
        // Remaining keys (odd ones) should still be present.
        for k in (1..50_i64).step_by(2) {
            assert_eq!(t.get(&k), Some(vec![k as u64]));
        }
    }

    #[test]
    fn remove_same_key_twice_second_returns_none() {
        let mut t = build(&[5, 10, 15]);
        assert_eq!(t.remove(&10), Some(vec![10]));
        assert_eq!(t.remove(&10), None);
        assert_eq!(t.len(), 2);
    }

    #[test]
    fn range_unbounded_returns_all_sorted() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        for k in 0..30_i64 {
            t.insert_into_tree(k, vec![k as u64]);
        }
        let all = t.range(None, None);
        assert_eq!(all.len(), 30);
        for (i, (k, v)) in all.iter().enumerate() {
            assert_eq!(*k, i as i64);
            assert_eq!(*v, vec![i as u64]);
        }
    }

    #[test]
    fn range_bounded_exclusive_end() {
        let t = build(&[0, 10, 20, 30, 40, 50]);
        // [10, 40) => 10, 20, 30
        let r = t.range(Some(&10), Some(&40));
        let keys: Vec<i64> = r.into_iter().map(|(k, _)| k).collect();
        assert_eq!(keys, vec![10, 20, 30]);
    }

    #[test]
    fn range_unbounded_start() {
        let t = build(&[10, 20, 30, 40]);
        let r = t.range(None, Some(&25));
        let keys: Vec<i64> = r.into_iter().map(|(k, _)| k).collect();
        assert_eq!(keys, vec![10, 20]);
    }

    #[test]
    fn range_unbounded_end() {
        let t = build(&[10, 20, 30, 40]);
        let r = t.range(Some(&25), None);
        let keys: Vec<i64> = r.into_iter().map(|(k, _)| k).collect();
        assert_eq!(keys, vec![30, 40]);
    }

    #[test]
    fn range_empty_when_start_equals_end() {
        let t = build(&[10, 20, 30]);
        let r = t.range(Some(&20), Some(&20));
        assert!(r.is_empty());
    }

    #[test]
    fn range_empty_when_start_greater_than_end() {
        let t = build(&[10, 20, 30]);
        let r = t.range(Some(&30), Some(&10));
        assert!(r.is_empty());
    }

    #[test]
    fn range_start_before_first_key_includes_first() {
        let t = build(&[10, 20, 30]);
        let r = t.range(Some(&5), Some(&25));
        let keys: Vec<i64> = r.into_iter().map(|(k, _)| k).collect();
        assert_eq!(keys, vec![10, 20]);
    }

    #[test]
    fn range_end_after_last_key_includes_last() {
        let t = build(&[10, 20, 30]);
        let r = t.range(Some(&15), Some(&999));
        let keys: Vec<i64> = r.into_iter().map(|(k, _)| k).collect();
        assert_eq!(keys, vec![20, 30]);
    }

    #[test]
    fn leftmost_leaf_on_empty_tree_is_root() {
        let t: BPlusTree<i64> = BPlusTree::default();
        let ll = t.leftmost_leaf(t.root.clone());
        let guard = ll.lock().unwrap();
        assert!(matches!(&*guard, Node::Leaf(_)));
    }

    #[test]
    fn leftmost_leaf_contains_minimum_key() {
        let t = build(&[50, 10, 30, 20, 40]);
        let ll = t.leftmost_leaf(t.root.clone());
        let guard = ll.lock().unwrap();
        let Node::Leaf(ln) = &*guard else {
            panic!("not a leaf")
        };
        // First key in leftmost leaf must be the global minimum.
        assert_eq!(ln.keys[0], 10);
    }

    #[test]
    fn leaf_walker_rows_empty_tree() {
        let t: BPlusTree<i64> = BPlusTree::default();
        let ll = t.leftmost_leaf(t.root.clone());
        let guard = ll.lock().unwrap();
        let Node::Leaf(ln) = &*guard else { panic!() };
        let rows = t.leaf_walker_rows(ln.clone()).unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn leaf_walker_rows_returns_all_values_in_key_order() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        for k in 0..20_i64 {
            t.insert_into_tree(k, vec![k as u64, k as u64 + 100]);
        }
        let ll = t.leftmost_leaf(t.root.clone());
        let guard = ll.lock().unwrap();
        let Node::Leaf(ln) = &*guard else { panic!() };
        let rows = t.leaf_walker_rows(ln.clone()).unwrap();

        // Should return all 20 Vec<u64> values, one per key.
        assert_eq!(rows.len(), 20);
        for (i, v) in rows.iter().enumerate() {
            assert_eq!(v[0], i as u64);
            assert_eq!(v[1], i as u64 + 100);
        }
    }

    #[test]
    fn validate_empty_tree_does_not_panic() {
        let t: BPlusTree<i64> = BPlusTree::default();
        t.validate(); // should not panic
    }

    #[test]
    fn validate_single_key_does_not_panic() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        t.insert_into_tree(1, vec![1]);
        t.validate();
    }

    #[test]
    fn validate_after_many_inserts_does_not_panic() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        for k in 0..200_i64 {
            t.insert_into_tree(k, vec![k as u64]);
        }
        t.validate();
    }

    #[test]
    fn validate_after_mixed_inserts_and_removes_does_not_panic() {
        let mut t: BPlusTree<i64> = BPlusTree::default();
        for k in 0..100_i64 {
            t.insert_into_tree(k, vec![k as u64]);
        }
        for k in (0..100_i64).step_by(3) {
            t.remove(&k);
        }
        t.validate();
    }

    #[test]
    fn insert_remove_matches_btreemap_oracle() {
        use std::collections::BTreeMap;

        let mut t: BPlusTree<i64> = BPlusTree::default();
        let mut m: BTreeMap<i64, Vec<u64>> = BTreeMap::new();

        // Deterministic LCG pseudo-random sequence (no external crates needed).
        let mut x: u64 = 0xDEAD_BEEF;
        let mut next_key = || {
            x = x
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((x >> 33) % 200) as i64
        };

        // Insert phase: 400 operations, keys in [0, 200).
        for id in 0..400_u64 {
            let k = next_key();
            let v = vec![id];
            let old_t = t.insert_into_tree(k, v.clone());
            let old_m = m.insert(k, v);
            assert_eq!(old_t, old_m, "insert mismatch at id={id}, key={k}");
        }
        assert_eq!(t.len(), m.len());
        t.validate();

        // Verify range() matches BTreeMap ordering.
        let tree_all: Vec<i64> = t.range(None, None).into_iter().map(|(k, _)| k).collect();
        let map_all: Vec<i64> = m.keys().copied().collect();
        assert_eq!(tree_all, map_all);

        // Remove phase: 300 operations.
        for _ in 0..300 {
            let k = next_key();
            let rt = t.remove(&k);
            let rm = m.remove(&k);
            assert_eq!(rt, rm, "remove mismatch at key={k}");
        }
        assert_eq!(t.len(), m.len());
        t.validate();

        // Final full cross-check.
        let tree_final: Vec<(i64, Vec<u64>)> = t.range(None, None);
        let map_final: Vec<(i64, Vec<u64>)> = m.into_iter().collect();
        assert_eq!(tree_final, map_final);
    }
}
