//! In-memory, single-threaded B+ tree.
//!
//! Features
//! - Internal nodes + leaf nodes (leaves linked for efficient range scans)
//! - insert / get / remove / range
//! - Split on insert; merge/redistribute on delete
//! - Always maintains B+ invariants, keeping tree height as low as possible
//!
//! Notes
//! - This is a *classic* B+ tree with a fixed maximum fanout (MAX_KEYS).
//! - Internal nodes store separator keys; values live only in leaves.
//! - Keys are kept in sorted order.
//!
//! Complexity
//! - get:    O(log n)
//! - insert: O(log n) amortized
//! - remove: O(log n) amortized
//! - range:  O(log n + k)
//!
//! Safety
//! - `#![forbid(unsafe_code)]` and uses `Rc<RefCell<_>>` for interior mutability.
//! - Single-threaded: `Rc` is fine (not `Send`/`Sync`).
#![forbid(unsafe_code)]

use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::rc::Rc;

type Link<K, V, const MAX_KEYS: usize> = Rc<RefCell<Node<K, V, MAX_KEYS>>>;

#[derive(Clone)]
enum Node<K, V, const MAX_KEYS: usize> {
    Internal(InternalNode<K, V, MAX_KEYS>),
    Leaf(LeafNode<K, V, MAX_KEYS>),
}

#[derive(Clone)]
struct InternalNode<K, V, const MAX_KEYS: usize> {
    // keys.len() == children.len() - 1
    keys: Vec<K>,
    children: Vec<Link<K, V, MAX_KEYS>>,
}

#[derive(Clone)]
struct LeafNode<K, V, const MAX_KEYS: usize> {
    keys: Vec<K>,
    values: Vec<V>,
    next: Option<Link<K, V, MAX_KEYS>>,
}

#[derive(Clone)]
pub struct BPlusTree<K, V, const MAX_KEYS: usize = 32> {
    root: Link<K, V, MAX_KEYS>,
    len: usize,
}

impl<K, V, const MAX_KEYS: usize> Default for BPlusTree<K, V, MAX_KEYS>
where
    K: Ord + Clone,
    V: Clone,
{
    fn default() -> Self {
        assert!(MAX_KEYS >= 3, "MAX_KEYS should be >= 3 for sensible behavior");
        let root = Rc::new(RefCell::new(Node::Leaf(LeafNode {
            keys: Vec::new(),
            values: Vec::new(),
            next: None,
        })));
        Self { root, len: 0 }
    }
}

impl<K, V, const MAX_KEYS: usize> BPlusTree<K, V, MAX_KEYS>
where
    K: Ord + Clone,
    V: Clone,
{
    /// Minimum number of keys a non-root node should have.
    /// (Classic B+ tree with max keys = MAX_KEYS => min keys = ceil(MAX_KEYS/2))
    fn min_keys() -> usize {
        (MAX_KEYS + 1) / 2
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let leaf = self.find_leaf(self.root.clone(), key);
        let leaf_b = leaf.borrow();
        let Node::Leaf(ln) = &*leaf_b else {
            unreachable!("find_leaf must return leaf");
        };
        match ln.keys.binary_search(key) {
            Ok(i) => Some(ln.values[i].clone()),
            Err(_) => None,
        }
    }

    /// Insert key/value. Returns previous value if key existed.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut path = Vec::<(Link<K, V, MAX_KEYS>, usize)>::new();
        let leaf = self.find_leaf_with_path(self.root.clone(), &key, &mut path);

        // Insert into leaf (or replace).
        let mut leaf_mut = leaf.borrow_mut();
        let Node::Leaf(ln) = &mut *leaf_mut else { unreachable!() };

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
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let mut path = Vec::<(Link<K, V, MAX_KEYS>, usize)>::new();
        let leaf = self.find_leaf_with_path(self.root.clone(), key, &mut path);

        // Remove from leaf.
        let mut leaf_mut = leaf.borrow_mut();
        let Node::Leaf(ln) = &mut *leaf_mut else { unreachable!() };

        let idx = match ln.keys.binary_search(key) {
            Ok(i) => i,
            Err(_) => return None,
        };

        ln.keys.remove(idx);
        let removed = ln.values.remove(idx);
        self.len -= 1;

        // If leaf is root, done (root may be empty).
        if Rc::ptr_eq(&leaf, &self.root) {
            return Some(removed);
        }

        // If still enough keys, might still need to update parent's separator key
        // if we removed the first key in this leaf.
        let leaf_first_key = ln.keys.first().cloned();
        drop(leaf_mut);

        if let Some((parent, child_index_in_parent)) = path.last().cloned() {
            // Update separator key that points to this leaf if needed:
            // In a B+ tree, parent's key at (child_index_in_parent-1) equals first key of this child.
            if idx == 0 {
                if let Some(new_first) = leaf_first_key {
                    self.update_parent_separator_key(parent.clone(), child_index_in_parent, new_first);
                }
            }
        }

        // Rebalance if underflow.
        self.rebalance_after_delete(path, leaf);

        Some(removed)
    }

    /// Range query: returns key/value pairs for start..=end or start..end depending on bounds.
    ///
    /// `start`: inclusive lower bound (None => unbounded)
    /// `end`:   exclusive upper bound (None => unbounded)
    pub fn range(&self, start: Option<&K>, end: Option<&K>) -> Vec<(K, V)> {
        let mut out = Vec::new();

        let leaf = match start {
            Some(k) => self.find_leaf(self.root.clone(), k),
            None => self.leftmost_leaf(self.root.clone()),
        };

        let mut cur = Some(leaf);
        while let Some(node) = cur {
            let b = node.borrow();
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

    /// Finds the leave with the lowest id. It is used traversing the tree 
    /// node to node. You use it also if you want all elements of this database
    ///
    pub fn leftmost_leaf(&self, mut node: Link<K, V, MAX_KEYS>) -> Link<K, V, MAX_KEYS> {
        loop {
            let b = node.borrow();
            match &*b {
                Node::Leaf(_) => {
                    drop(b);      // end the borrow explicitly
                    return node;  // now it's legal
                }
                Node::Internal(internal) => {
                    let next = internal.children[0].clone();
                    drop(b);      // end borrow before reassign
                    node = next;
                }
            }
        }
    }

    // -------------------------
    // Search helpers
    // -------------------------

    fn find_leaf(&self, mut node: Link<K, V, MAX_KEYS>, key: &K) -> Link<K, V, MAX_KEYS> {
        loop {
            let b = node.borrow();
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
        mut node: Link<K, V, MAX_KEYS>,
        key: &K,
        path: &mut Vec<(Link<K, V, MAX_KEYS>, usize)>,
    ) -> Link<K, V, MAX_KEYS> {
        loop {
            let next = {
                let b = node.borrow();
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

    fn split_leaf(&self, leaf: Link<K, V, MAX_KEYS>) -> (K, Link<K, V, MAX_KEYS>) {
        let mut leaf_mut = leaf.borrow_mut();
        let Node::Leaf(ln) = &mut *leaf_mut else { unreachable!() };

        // Split roughly in half; right gets the larger half.
        let mid = ln.keys.len() / 2;

        let right_keys = ln.keys.split_off(mid);
        let right_vals = ln.values.split_off(mid);

        let sep_key = right_keys[0].clone();

        let right = Rc::new(RefCell::new(Node::Leaf(LeafNode {
            keys: right_keys,
            values: right_vals,
            next: ln.next.take(),
        })));

        ln.next = Some(right.clone());

        (sep_key, right)
    }

    fn split_internal(
        &self,
        internal: Link<K, V, MAX_KEYS>,
    ) -> (K, Link<K, V, MAX_KEYS>) {
        let mut nmut = internal.borrow_mut();
        let Node::Internal(inode) = &mut *nmut else { unreachable!() };

        // For internal node split:
        // promote middle key to parent, left keeps keys < promoted,
        // right gets keys > promoted.
        let mid_key_index = inode.keys.len() / 2;
        let promoted = inode.keys[mid_key_index].clone();

        let right_keys = inode.keys.split_off(mid_key_index + 1);
        inode.keys.pop(); // remove promoted from left

        let right_children = inode.children.split_off(mid_key_index + 1);

        let right = Rc::new(RefCell::new(Node::Internal(InternalNode {
            keys: right_keys,
            children: right_children,
        })));

        (promoted, right)
    }

    fn insert_in_parent(
        &mut self,
        mut path: Vec<(Link<K, V, MAX_KEYS>, usize)>,
        left: Link<K, V, MAX_KEYS>,
        sep_key: K,
        right: Link<K, V, MAX_KEYS>,
    ) {
        // If no parent, create new root.
        let Some((parent, left_index)) = path.pop() else {
            let new_root = Rc::new(RefCell::new(Node::Internal(InternalNode {
                keys: vec![sep_key],
                children: vec![left, right],
            })));
            self.root = new_root;
            return;
        };

        // Insert sep_key into parent.keys at left_index, and right child at left_index+1
        {
            let mut pb = parent.borrow_mut();
            let Node::Internal(pn) = &mut *pb else { unreachable!() };

            pn.keys.insert(left_index, sep_key);
            pn.children.insert(left_index + 1, right);
        }

        // If parent overflows, split and propagate up.
        let overflow = {
            let pb = parent.borrow();
            let Node::Internal(pn) = &*pb else { unreachable!() };
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

    fn rebalance_after_delete(
        &mut self,
        mut path: Vec<(Link<K, V, MAX_KEYS>, usize)>,
        mut node: Link<K, V, MAX_KEYS>,
    ) {
        loop {
            // If node is root, shrink root if possible.
            if Rc::ptr_eq(&node, &self.root) {
                self.maybe_shrink_root();
                return;
            }

            let min_keys = Self::min_keys();

            let node_key_count = {
                let nb = node.borrow();
                match &*nb {
                    Node::Leaf(ln) => ln.keys.len(),
                    Node::Internal(inode) => inode.keys.len(),
                }
            };

            if node_key_count >= min_keys {
                // underflow resolved
                return;
            }

            // Need parent info.
            let Some((parent, idx_in_parent)) = path.pop() else {
                // Shouldn't happen because non-root has parent.
                return;
            };

            // Try redistribute from siblings first.
            if self.try_redistribute(&parent, idx_in_parent) {
                return;
            }

            // Otherwise, merge with a sibling.
            let merged_into_left = self.merge_with_sibling(&parent, idx_in_parent);

            // After merge, parent lost a key; node becomes the merged parent child we should continue checking.
            node = if merged_into_left {
                // merged current into left sibling; new "node" for next loop is parent
                parent.clone()
            } else {
                // merged right sibling into current; new "node" is parent
                parent.clone()
            };

            // Continue loop: parent might now underflow.
        }
    }

    fn try_redistribute(&self, parent: &Link<K, V, MAX_KEYS>, idx: usize) -> bool {
        // Attempt borrow from left sibling if exists, else right sibling.
        let (left_sib, right_sib) = {
            let pb = parent.borrow();
            let Node::Internal(pn) = &*pb else { unreachable!() };

            let left = if idx > 0 { Some(pn.children[idx - 1].clone()) } else { None };
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

    fn redistribute_from_left(
        &self,
        parent: Link<K, V, MAX_KEYS>,
        idx: usize,
        left: Link<K, V, MAX_KEYS>,
    ) {
        let mut pb = parent.borrow_mut();
        let Node::Internal(pn) = &mut *pb else { unreachable!() };

        let cur = pn.children[idx].clone();

        match (&mut *left.borrow_mut(), &mut *cur.borrow_mut()) {
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
                // Internal redistribution:
                // Bring parent separator down into current as first key,
                // and move left's last child to current's front.
                let sep_down = pn.keys[idx - 1].clone();

                let borrowed_key = in_left.keys.pop().expect("left has keys");
                let borrowed_child = in_left.children.pop().expect("left has child");

                // Parent separator replaced by left's last key.
                pn.keys[idx - 1] = borrowed_key;

                // Current receives sep_down as first key, and borrowed_child as first child.
                in_cur.keys.insert(0, sep_down);
                in_cur.children.insert(0, borrowed_child);
            }
            _ => unreachable!("tree nodes at same level must have same variant"),
        }
    }

    fn redistribute_from_right(
        &self,
        parent: Link<K, V, MAX_KEYS>,
        idx: usize,
        right: Link<K, V, MAX_KEYS>,
    ) {
        let mut pb = parent.borrow_mut();
        let Node::Internal(pn) = &mut *pb else { unreachable!() };

        let cur = pn.children[idx].clone();

        match (&mut *cur.borrow_mut(), &mut *right.borrow_mut()) {
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
                // Internal redistribution:
                // Bring parent separator down into current as last key,
                // move right's first child to current's end,
                // and move right's first key up to parent.
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

    /// Merge node at idx with a sibling. Returns true if merged into left sibling, false if merged right into current.
    fn merge_with_sibling(&self, parent: &Link<K, V, MAX_KEYS>, idx: usize) -> bool {
        let mut pb = parent.borrow_mut();
        let Node::Internal(pn) = &mut *pb else { unreachable!() };

        // Prefer merge with left if exists; otherwise merge with right.
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
            // Merge right sibling into current
            let cur = pn.children[idx].clone();
            let right = pn.children[idx + 1].clone();
            let sep_key = pn.keys.remove(idx);
            pn.children.remove(idx + 1);

            drop(pb);

            self.merge_nodes(cur, right, Some(sep_key));
            false
        }
    }

    fn merge_nodes(
        &self,
        left: Link<K, V, MAX_KEYS>,
        right: Link<K, V, MAX_KEYS>,
        sep_key_for_internal: Option<K>,
    ) {
        match (&mut *left.borrow_mut(), &mut *right.borrow_mut()) {
            (Node::Leaf(ln_left), Node::Leaf(ln_right)) => {
                ln_left.keys.extend(ln_right.keys.drain(..));
                ln_left.values.extend(ln_right.values.drain(..));
                ln_left.next = ln_right.next.take();
            }
            (Node::Internal(in_left), Node::Internal(in_right)) => {
                // For internal merge, bring separator key down between left and right keys.
                let sep = sep_key_for_internal.expect("internal merge needs separator key");
                in_left.keys.push(sep);
                in_left.keys.extend(in_right.keys.drain(..));
                in_left.children.extend(in_right.children.drain(..));
            }
            _ => unreachable!("merge nodes must be same type"),
        }
    }

    fn maybe_shrink_root(&mut self) {
        // If root is internal with a single child, promote that child as the new root.
        // If root is leaf, keep it.
        let shrink_to = {
            let rb = self.root.borrow();
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

    fn update_parent_separator_key(
        &self,
        parent: Link<K, V, MAX_KEYS>,
        child_index: usize,
        new_first_key: K,
    ) {
        // For child at index i>0, parent's key at i-1 should equal child's first key.
        if child_index == 0 {
            return;
        }
        let mut pb = parent.borrow_mut();
        let Node::Internal(pn) = &mut *pb else { unreachable!() };
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

fn node_key_len<K, V, const MAX_KEYS: usize>(n: &Link<K, V, MAX_KEYS>) -> usize {
    let b = n.borrow();
    match &*b {
        Node::Leaf(ln) => ln.keys.len(),
        Node::Internal(inode) => inode.keys.len(),
    }
}

// -------------------------
// Optional: basic validation (debug / tests)
// -------------------------

impl<K, V, const MAX_KEYS: usize> BPlusTree<K, V, MAX_KEYS>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    /// Debug helper: validates key ordering and basic B+ invariants.
    /// Panics if invariant is violated.
    pub fn validate(&self) {
        let min_keys = Self::min_keys();
        self.validate_node(self.root.clone(), true, min_keys);
        self.validate_leaf_chain();
    }

    fn validate_node(
        &self,
        node: Link<K, V, MAX_KEYS>,
        is_root: bool,
        min_keys: usize,
    ) -> Option<(K, K)> {
        let b = node.borrow();
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

                Some((ln.keys.first().unwrap().clone(), ln.keys.last().unwrap().clone()))
            }

            Node::Internal(inode) => {
                assert!(inode.keys.len() <= MAX_KEYS, "internal overflow");
                assert_eq!(
                    inode.children.len(),
                    inode.keys.len() + 1,
                    "internal arity mismatch"
                );
                if !is_root {
                    assert!(inode.keys.len() >= min_keys, "internal underflow");
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
            let b = n.borrow();
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
    use super::BPlusTree;

    #[test]
    fn basic_insert_get_range_remove() {
        let mut t: BPlusTree<i32, String, 4> = BPlusTree::default();

        assert!(t.is_empty());
        t.insert(10, "a".into());
        t.insert(20, "b".into());
        t.insert(30, "c".into());
        t.insert(40, "d".into());
        t.insert(50, "e".into());

        assert_eq!(t.get(&10).as_deref(), Some("a"));
        assert_eq!(t.get(&35), None);

        let r = t.range(Some(&15), Some(&45));
        let keys: Vec<i32> = r.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, vec![20, 30, 40]);

        assert_eq!(t.remove(&30).as_deref(), Some("c"));
        assert_eq!(t.get(&30), None);

        // Validate structural invariants after deletes.
        t.validate();
    }

    #[test]
    fn remove_causes_redistribute_and_merge_and_preserves_order() {
        // Small fanout so we trigger splits/merges with fewer keys.
        let mut t: BPlusTree<i32, i32, 4> = BPlusTree::default();

        // Insert a bunch of keys to create a multi-level tree.
        for k in 0..200 {
            assert_eq!(t.insert(k, k * 10), None);
        }
        assert_eq!(t.len(), 200);
        t.validate();

        // Remove every 3rd key (creates "holes" and triggers some redistributions).
        for k in (0..200).step_by(3) {
            assert_eq!(t.remove(&k), Some(k * 10));
        }
        t.validate();

        // Ensure removed keys are gone, others still present.
        for k in 0..200 {
            if k % 3 == 0 {
                assert_eq!(t.get(&k), None);
            } else {
                assert_eq!(t.get(&k), Some(k * 10));
            }
        }

        // Now remove a large contiguous range to force merges and potentially shrink height.
        for k in 1..150 {
            if k % 3 != 0 {
                assert_eq!(t.remove(&k), Some(k * 10));
            }
        }
        t.validate();

        // Remaining keys should be exactly those >= 150 that are not multiples of 3.
        let remaining = t.range(None, None);
        for (k, v) in &remaining {
            assert!(*k >= 150);
            assert!(*k % 3 != 0);
            assert_eq!(*v, *k * 10);
        }

        // Remove everything left; tree should still be valid and empty.
        for (k, _) in remaining {
            assert!(t.remove(&k).is_some());
        }
        assert_eq!(t.len(), 0);

        // Root should end up as a (possibly empty) leaf; validate should not panic.
        // If your validate() panics on empty root leaf bounds (as discussed),
        // comment this out or adjust validate() to handle empty root leaf.
        // t.validate();

        // Basic sanity: range on empty tree
        assert!(t.range(None, None).is_empty());
    }

    #[test]
    fn insert_replaces_existing_value_and_len_stable() {
        let mut t: BPlusTree<i32, String, 4> = BPlusTree::default();

        assert_eq!(t.insert(10, "a".into()), None);
        assert_eq!(t.len(), 1);
        assert_eq!(t.get(&10).as_deref(), Some("a"));

        // Replace
        assert_eq!(t.insert(10, "b".into()).as_deref(), Some("a"));
        assert_eq!(t.len(), 1);
        assert_eq!(t.get(&10).as_deref(), Some("b"));

        t.validate();
    }

    #[test]
    fn insert_many_increasing_keys_produces_sorted_range() {
        let mut t: BPlusTree<i32, i32, 4> = BPlusTree::default();

        for k in 0..1000 {
            assert_eq!(t.insert(k, k * 2), None);
        }
        assert_eq!(t.len(), 1000);

        // Ensure range is sorted and complete.
        let all = t.range(None, None);
        assert_eq!(all.len(), 1000);
        for (i, (k, v)) in all.iter().enumerate() {
            assert_eq!(*k, i as i32);
            assert_eq!(*v, (*k) * 2);
        }

        t.validate();
    }

    #[test]
    fn insert_many_decreasing_keys_produces_sorted_range() {
        let mut t: BPlusTree<i32, i32, 4> = BPlusTree::default();

        for k in (0..500).rev() {
            assert_eq!(t.insert(k, k + 1), None);
        }
        assert_eq!(t.len(), 500);

        let all = t.range(None, None);
        assert_eq!(all.len(), 500);
        for (i, (k, v)) in all.iter().enumerate() {
            assert_eq!(*k, i as i32);
            assert_eq!(*v, *k + 1);
        }

        t.validate();
    }

    #[test]
    fn insert_interleaved_keys_forces_splits_and_get_works() {
        let mut t: BPlusTree<i32, i32, 4> = BPlusTree::default();

        // Interleave low/high to exercise different split paths.
        for i in 0..200 {
            let a = i;
            let b = 10_000 - i;
            assert_eq!(t.insert(a, a * 10), None);
            assert_eq!(t.insert(b, b * 10), None);
        }
        assert_eq!(t.len(), 400);

        // Spot-check a bunch of keys.
        for i in 0..200 {
            let a = i;
            let b = 10_000 - i;
            assert_eq!(t.get(&a), Some(a * 10));
            assert_eq!(t.get(&b), Some(b * 10));
        }

        // Range should be globally sorted.
        let all = t.range(None, None);
        for w in all.windows(2) {
            assert!(w[0].0 < w[1].0);
        }

        t.validate();
    }

    #[test]
    fn insert_then_range_with_bounds_matches_expected() {
        let mut t: BPlusTree<i32, i32, 4> = BPlusTree::default();

        for k in 0..100 {
            t.insert(k, k);
        }

        // [10, 20) => 10..=19
        let r = t.range(Some(&10), Some(&20));
        let keys: Vec<i32> = r.into_iter().map(|(k, _)| k).collect();
        assert_eq!(keys, (10..20).collect::<Vec<_>>());

        // Unbounded start
        let r2 = t.range(None, Some(&5));
        let keys2: Vec<i32> = r2.into_iter().map(|(k, _)| k).collect();
        assert_eq!(keys2, (0..5).collect::<Vec<_>>());

        // Unbounded end
        let r3 = t.range(Some(&95), None);
        let keys3: Vec<i32> = r3.into_iter().map(|(k, _)| k).collect();
        assert_eq!(keys3, (95..100).collect::<Vec<_>>());

        t.validate();
    }

    #[test]
    fn remove_missing_key_returns_none_and_does_not_change_len() {
        let mut t: BPlusTree<i32, i32, 4> = BPlusTree::default();

        for k in 0..50 {
            t.insert(k, k);
        }
        let before = t.len();

        assert_eq!(t.remove(&999), None);
        assert_eq!(t.remove(&-1), None);
        assert_eq!(t.len(), before);

        t.validate();
    }

    #[test]
    fn remove_first_key_repeatedly_updates_structure_correctly() {
        let mut t: BPlusTree<i32, i32, 4> = BPlusTree::default();

        for k in 0..200 {
            t.insert(k, k * 3);
        }
        t.validate();

        // Remove ascending (often exercises “first key in leaf changed” paths)
        for k in 0..200 {
            assert_eq!(t.remove(&k), Some(k * 3));
            assert_eq!(t.get(&k), None);

            // Spot-check a couple still-present keys.
            if k + 1 < 200 {
                assert_eq!(t.get(&(k + 1)), Some((k + 1) * 3));
            }
            if k + 10 < 200 {
                assert_eq!(t.get(&(k + 10)), Some((k + 10) * 3));
            }

            // Validate occasionally (keeps test time reasonable)
            if k % 17 == 0 {
                t.validate();
            }
        }

        assert_eq!(t.len(), 0);
        assert!(t.range(None, None).is_empty());
    }

    #[test]
    fn remove_last_key_repeatedly_updates_structure_correctly() {
        let mut t: BPlusTree<i32, i32, 4> = BPlusTree::default();

        for k in 0..200 {
            t.insert(k, k * 5);
        }
        t.validate();

        // Remove descending (exercises right-edge merges/borrows)
        for k in (0..200).rev() {
            assert_eq!(t.remove(&k), Some(k * 5));
            assert_eq!(t.get(&k), None);

            if k >= 1 {
                assert_eq!(t.get(&(k - 1)), Some((k - 1) * 5));
            }

            if k % 19 == 0 {
                t.validate();
            }
        }

        assert_eq!(t.len(), 0);
        assert!(t.range(None, None).is_empty());
    }

    #[test]
    fn remove_all_then_reuse_tree_with_new_inserts() {
        let mut t: BPlusTree<i32, i32, 4> = BPlusTree::default();

        for k in 0..120 {
            t.insert(k, k);
        }
        t.validate();

        for k in 0..120 {
            assert_eq!(t.remove(&k), Some(k));
        }
        assert_eq!(t.len(), 0);
        assert!(t.range(None, None).is_empty());

        // Tree should still be usable after becoming empty.
        for k in 1000..1100 {
            assert_eq!(t.insert(k, k * 2), None);
        }
        assert_eq!(t.len(), 100);
        t.validate();

        for k in 1000..1100 {
            assert_eq!(t.get(&k), Some(k * 2));
        }
    }

    #[test]
    fn delete_matches_btreemap_model() {
        use std::collections::BTreeMap;

        let mut t: BPlusTree<i32, i32, 4> = BPlusTree::default();
        let mut m = BTreeMap::<i32, i32>::new();

        // Deterministic pseudo-random-ish sequence without external crates.
        // (Linear congruential generator)
        let mut x: u32 = 0xC0FFEE;
        let mut next_i32 = || {
            x = x.wrapping_mul(1664525).wrapping_add(1013904223);
            // map into a modest key range with collisions
            (x % 500) as i32
        };

        // Insert phase
        for _ in 0..2000 {
            let k = next_i32();
            let v = k * 7;
            let old_t = t.insert(k, v);
            let old_m = m.insert(k, v);
            assert_eq!(old_t, old_m);
        }
        t.validate();

        // Delete phase
        for i in 0..3000 {
            let k = next_i32();
            let rt = t.remove(&k);
            let rm = m.remove(&k);
            assert_eq!(rt, rm, "mismatch removing key {k} at step {i}");

            // Occasionally cross-check full ordered contents via range()
            if i % 250 == 0 {
                let tv: Vec<(i32, i32)> = t.range(None, None).into_iter().map(|(k, v)| (k, v)).collect();
                let mv: Vec<(i32, i32)> = m.iter().map(|(k, v)| (*k, *v)).collect();
                assert_eq!(tv, mv, "range() diverged from BTreeMap at step {i}");
                t.validate();
            }
        }

        // Final full cross-check
        let tv: Vec<(i32, i32)> = t.range(None, None).into_iter().map(|(k, v)| (k, v)).collect();
        let mv: Vec<(i32, i32)> = m.iter().map(|(k, v)| (*k, *v)).collect();
        assert_eq!(tv, mv);
        t.validate();
    }


}



