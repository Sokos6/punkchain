/// Inserts a key-value pair into the map.
///
/// Returns the previous value associated with the same key if any.
/// If the map did not have this key present, `None` is returned.
pub fn insert(&mut self, key: K, new_value: V) -> Option<V> { /* --snip-- */
}

/// Removes the key/value pair from the map associated with the given key.
///
/// - Returns the removed value if any.
pub fn take<Q>(&mut self, key: &Q) -> Option<V> { /* --snip-- */
}

/// Returns a shared reference to the value corresponding to the key.
///
/// The key may be any borrowed form of the map's key type,
/// but `Hash` and `Eq` on the borrowed form must match those for the key type.
pub fn get<Q>(&self, key: &Q) -> Option<&V> { /* --snip-- */
}

/// Returns a mutable reference to the value corresponding to the key.
///
/// The key may be any borrowed form of the map's key type,
/// but `Hash` and `Eq` on the borrowed form must match those for the key type.
pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V> { /* --snip-- */
}

/// Returns `true` if there is an entry corresponding to the key in the map.
pub fn contains_key<Q>(&self, key: &Q) -> bool { /* --snip-- */
}

/// Converts the OccupiedEntry into a mutable reference to the value in the entry
/// with a lifetime bound to the map itself.
pub fn into_mut(self) -> &'a mut V { /* --snip-- */
}

/// Gets the given key's corresponding entry in the map for in-place manipulation.
pub fn entry(&mut self, key: K) -> Entry<K, V> { /* --snip-- */
}
