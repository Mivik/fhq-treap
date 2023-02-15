use std::{iter::Peekable, cmp::Ordering};

struct NodeData<K: Ord, V> {
    left: TreapMap<K, V>,
    right: TreapMap<K, V>,
    size: u32,
    key: K,
    value: V,

    weight: u32,
}

impl<K: Ord, V> NodeData<K, V> {
    pub fn new(key: K, value: V) -> Box<Self> {
        Box::new(Self {
            left: TreapMap::new(),
            right: TreapMap::new(),
            size: 1,
            key,
            value,

            weight: rand::random(),
        })
    }

    #[inline]
    fn maintain(&mut self) {
        self.size = self.left.len() + self.right.len() + 1;
    }
}

pub struct TreapMap<K: Ord, V>(Option<Box<NodeData<K, V>>>);
impl<K: Ord, V> Default for TreapMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Ord, V> From<Box<NodeData<K, V>>> for TreapMap<K, V> {
    fn from(value: Box<NodeData<K, V>>) -> Self {
        Self(Some(value))
    }
}

impl<K: Ord, V> TreapMap<K, V> {
    fn new() -> Self {
        Self(None)
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.0.as_ref().map_or(0, |it| it.size)
    }

    pub fn split(self, key: &K) -> (Self, Self) {
        let Some(mut x) = self.0 else { return Default::default(); };
        if key <= &x.key {
            let (l, r) = x.left.split(key);
            x.left = r;
            (l, x.into())
        } else {
            let (l, r) = x.right.split(key);
            x.right = l;
            (x.into(), r)
        }
    }

    pub fn split_n(self, n: u32) -> (Self, Self) {
        let Some(mut x) = self.0 else { return Default::default(); };
        if n >= x.size {
            return (x.into(), Self::new());
        }
        let ls = x.left.len();
        if n <= ls {
            let (l, r) = x.left.split_n(n);
            x.left = r;
            (l, x.into())
        } else {
            let (l, r) = x.right.split_n(n - ls - 1);
            x.right = l;
            (x.into(), r)
        }
    }

    pub fn merge(x: Self, y: Self) -> Self {
        let Some(mut x) = x.0 else { return y };
        let Some(mut y) = y.0 else { return x.into() };
        if x.weight < y.weight {
            x.right = Self::merge(x.right, y.into());
            x.maintain();
            x.into()
        } else {
            y.left = Self::merge(x.into(), y.left);
            y.maintain();
            y.into()
        }
    }

    fn get_kv(&self, key: &K) -> Option<(&K, &V)> {
    	let mut x = self;
    	loop {
    		let Some(node) = &x.0 else { return None };
    		match key.cmp(&node.key) {
    			Ordering::Less => {
    				x = &node.left;
    			}
    			Ordering::Equal => {
    				return Some((&node.key, &node.value));
    			}
    			Ordering::Greater => {
    				x = &node.right;
    			}
    		}
    	}
    }

    fn get_kv_mut(&mut self, key: &K) -> Option<(&K, &mut V)> {
    	let mut x = self;
    	loop {
    		let Some(node) = &mut x.0 else { return None };
    		match key.cmp(&node.key) {
    			Ordering::Less => {
    				x = &mut node.left;
    			}
    			Ordering::Equal => {
    				return Some((&node.key, &mut node.value));
    			}
    			Ordering::Greater => {
    				x = &mut node.right;
    			}
    		}
    	}
    }

    #[inline]
    pub fn get(&self, key: &K) -> Option<&V> {
    	self.get_kv(key).map(|it| it.1)
    }

    #[inline]
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
    	self.get_kv_mut(key).map(|it| it.1)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let (l, mut r) = std::mem::take(self).split(&key);
        if let Some((k, v)) = r.min_mut() {
            if k == &key {
                let res = std::mem::replace(v, value);
                *self = Self::merge(l, r);
                return Some(res);
            }
        }
        let node = NodeData::new(key, value).into();
        *self = Self::merge(Self::merge(l, node), r);
        None
    }

    pub fn rank(&self, key: &K) -> u32 {
        let mut x = self;
        let mut r = 0;
        while let Some(node) = &x.0 {
            if key <= &node.key {
                x = &node.left;
            } else {
                r += node.left.len() + 1;
                x = &node.right;
            }
        }
        r
    }

    pub fn nth_kv(&self, mut n: u32) -> Option<(&K, &V)> {
        if n >= self.len() {
            return None;
        }
        let mut x = self;
        loop {
            let Some(node) = &x.0 else { unreachable!() };
            let ls = node.left.len();
            if n <= ls {
                x = &node.left;
            } else {
                n -= ls + 1;
                if n == 0 {
                    break Some((&node.key, &node.value));
                }
                x = &node.right;
            }
        }
    }

    pub fn nth_kv_mut(&mut self, mut n: u32) -> Option<(&K, &mut V)> {
        if n >= self.len() {
            return None;
        }
        let mut x = self;
        loop {
            let Some(node) = &mut x.0 else { unreachable!() };
            let ls = node.left.len();
            if n <= ls {
                x = &mut node.left;
            } else {
                n -= ls + 1;
                if n == 0 {
                    break Some((&node.key, &mut node.value));
                }
                x = &mut node.right;
            }
        }
    }

    #[inline]
    pub fn nth(&self, k: u32) -> Option<&V> {
        self.nth_kv(k).map(|it| it.1)
    }

    #[inline]
    pub fn nth_mut(&mut self, k: u32) -> Option<&mut V> {
        self.nth_kv_mut(k).map(|it| it.1)
    }

    fn min(&self) -> Option<(&K, &V)> {
        let Some(mut x) = self.0.as_ref() else { return None };
        while let Some(y) = &x.left.0 {
            x = y;
        }
        Some((&x.key, &x.value))
    }

    fn min_mut(&mut self) -> Option<(&K, &mut V)> {
        let Some(mut x) = self.0.as_mut() else { return None };
        while let Some(y) = &mut x.left.0 {
            x = y;
        }
        Some((&x.key, &mut x.value))
    }

    fn max(&self) -> Option<(&K, &V)> {
        let Some(mut x) = self.0.as_ref() else { return None };
        while let Some(y) = &x.right.0 {
            x = y;
        }
        Some((&x.key, &x.value))
    }

    fn max_mut(&mut self) -> Option<(&K, &mut V)> {
        let Some(mut x) = self.0.as_mut() else { return None };
        while let Some(y) = &mut x.right.0 {
            x = y;
        }
        Some((&x.key, &mut x.value))
    }
}

impl<K: Ord, V> TreapMap<K, V> {
    pub fn from_sorted_iter(iter: impl Iterator<Item = (K, V)>) -> Self {
        Self::from_unique_sorted_iter(DedupSortedIter(iter.peekable()))
    }

    pub fn from_unique_sorted_iter(iter: impl Iterator<Item = (K, V)>) -> Self {
        let mut stack: Vec<Box<NodeData<K, V>>> = Vec::new();
        for (key, value) in iter {
            let mut node = Box::new(NodeData {
                left: TreapMap::new(),
                right: TreapMap::new(),
                size: 1,
                key,
                value,

                weight: rand::random(),
            });
            while let Some(mut top) = stack.pop() {
                if node.weight < top.weight {
                    top.right = node.left;
                    top.maintain();
                    node.left = top.into();
                } else {
                    stack.push(top);
                    break;
                }
            }
            node.maintain();
            stack.push(node.into());
        }
        while let Some(top) = stack.pop() {
            let top = top.into();
            match stack.last_mut() {
                Some(x) => {
                    x.right = top;
                    x.maintain();
                }
                None => {
                    return top;
                }
            }
        }
        Self::new()
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for TreapMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut vec: Vec<(K, V)> = iter.into_iter().collect();
        if vec.is_empty() {
            return Self::new();
        }
        vec.sort_by(|x, y| x.0.cmp(&y.0));
        Self::from_unique_sorted_iter(vec.into_iter())
    }
}

struct DedupSortedIter<K, V, I: Iterator<Item = (K, V)>>(Peekable<I>);
impl<K: Eq, V, I: Iterator<Item = (K, V)>> Iterator for DedupSortedIter<K, V, I> {
    type Item = (K, V);

    fn next(&mut self) -> Option<(K, V)> {
        let next = self.0.next()?;
        loop {
            let Some(peek) = self.0.peek() else { return Some(next) };
            if next.0 != peek.0 {
                break;
            }
            self.0.next();
        }
        Some(next)
    }
}

#[repr(transparent)]
pub struct TreapSet<K: Ord>(TreapMap<K, ()>);
impl<K: Ord> Default for TreapSet<K> {
    fn default() -> Self {
        Self::new()
    }
}
impl<K: Ord> TreapSet<K> {
    #[inline]
    pub fn new() -> Self {
        Self(TreapMap::new())
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.0.len()
    }

    #[inline]
    pub fn split(self, key: &K) -> (Self, Self) {
        let (l, r) = self.0.split(key);
        (Self(l), Self(r))
    }

    #[inline]
    pub fn split_n(self, n: u32) -> (Self, Self) {
        let (l, r) = self.0.split_n(n);
        (Self(l), Self(r))
    }

    #[inline]
    pub fn merge(x: Self, y: Self) -> Self {
        Self(TreapMap::merge(x.0, y.0))
    }

    #[inline]
    pub fn insert(&mut self, key: K) -> bool {
        self.0.insert(key, ()).is_none()
    }

    #[inline]
    pub fn rank(&self, key: &K) -> u32 {
        self.0.rank(key)
    }

    #[inline]
    pub fn nth(&self, n: u32) -> Option<&K> {
        self.0.nth_kv(n).map(|it| it.0)
    }

    #[inline]
    pub fn min(&self) -> Option<&K> {
        self.0.min().map(|it| it.0)
    }

    #[inline]
    pub fn max(&self) -> Option<&K> {
        self.0.max().map(|it| it.0)
    }

    #[inline]
    pub fn get(&self, key: &K) -> Option<&K> {
    	self.0.get_kv(key).map(|it| it.0)
    }
}

impl<K: Ord> TreapSet<K> {
    #[inline]
    pub fn from_sorted_iter(iter: impl Iterator<Item = K>) -> Self {
        Self(TreapMap::from_sorted_iter(iter.map(|it| (it, ()))))
    }

    #[inline]
    pub fn from_unique_sorted_iter(iter: impl Iterator<Item = K>) -> Self {
        Self(TreapMap::from_unique_sorted_iter(iter.map(|it| (it, ()))))
    }
}
