use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};

pub struct Id<T> {
    index: u32,
    generation: u32,
    _phantom_data: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(index: u32, generation: u32) -> Self {
        Self {
            index,
            generation,
            _phantom_data: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct SparseSet<T> {
    data: Vec<Option<T>>,
    sparse_to_dense: Vec<Sparse>,
    dense_to_sparse: Vec<Dense>,
    free_sparse_indices: Vec<u32>,
    next_free_id: u32,
}

#[derive(Debug, Clone, Copy)]
struct Sparse {
    dense: u32,
    /// When this is None, there is no element at this index.
    generation: u32,
}

#[derive(Debug, Clone, Copy)]
struct Dense {
    sparse: u32,
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        Self {
            data: vec![],
            sparse_to_dense: vec![],
            dense_to_sparse: vec![],
            free_sparse_indices: vec![],
            next_free_id: 0,
        }
    }

    pub fn insert(&mut self, element: T) -> Id<T> {
        let (sparse_index, dense_index) = self.allocate_free_indices();
        self.data[sparse_index as usize] = Some(element);
        let generation = self.sparse_to_dense[sparse_index as usize].put(dense_index);
        self.dense_to_sparse[dense_index as usize] = Dense {
            sparse: sparse_index,
        };
        Id::new(sparse_index, generation)
    }

    /// Internal method used for deserialization.
    fn insert_at_sparse_index(
        &mut self,
        index: u32,
        element: T,
    ) -> Result<(), serialization::OverlappingId> {
        if let Some(Some(_)) = self.data.get(index as usize) {
            return Err(serialization::OverlappingId(index));
        }

        let index = index as usize;
        while self.data.len() <= index {
            self.data.push(None);
            self.sparse_to_dense.push(Sparse {
                dense: 0,
                generation: 0,
            });
        }
        let dense = self.dense_to_sparse.len();
        self.data[index] = Some(element);
        self.sparse_to_dense[index].put(dense as u32);
        self.dense_to_sparse.push(Dense {
            sparse: index as u32,
        });

        Ok(())
    }

    pub fn remove(&mut self, id: Id<T>) -> Option<T> {
        if let Some(&Sparse { dense, generation }) = self.sparse_to_dense.get(id.index as usize) {
            if id.generation == generation {
                let dense_index = dense as usize;

                let element = self.data[id.index as usize].take();
                self.sparse_to_dense[id.index as usize].next_generation();
                self.dense_to_sparse.swap_remove(dense_index);
                self.free_sparse_indices.push(id.index);

                return element;
            }
        }
        None
    }

    pub fn get(&self, id: Id<T>) -> Option<&T> {
        if let Some(&Sparse { generation, .. }) = self.sparse_to_dense.get(id.index as usize) {
            if id.generation == generation {
                return self.data[id.index as usize].as_ref();
            }
        }
        None
    }

    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        if let Some(&Sparse { generation, .. }) = self.sparse_to_dense.get(id.index as usize) {
            if id.generation == generation {
                return self.data[id.index as usize].as_mut();
            }
        }
        None
    }

    pub fn ids(&self) -> Ids<'_, T> {
        Ids {
            set: self,
            dense: self.dense_to_sparse.iter(),
        }
    }

    pub fn pairs(&self) -> Pairs<'_, T> {
        Pairs {
            set: self,
            ids: self.ids(),
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            set: self,
            dense: self.dense_to_sparse.iter(),
        }
    }

    /// Find and allocate a free sparse and dense index pair.
    fn allocate_free_indices(&mut self) -> (u32, u32) {
        (
            self.free_sparse_indices.pop().unwrap_or_else(|| {
                let i = self.next_free_id;
                self.next_free_id += 1;
                self.sparse_to_dense.push(Sparse {
                    dense: 0,
                    generation: 0,
                });
                i
            }),
            self.dense_to_sparse.len() as u32,
        )
    }
}

impl Sparse {
    /// Puts a new dense value in the mapping and returns the generation number.
    fn put(&mut self, dense: u32) -> u32 {
        self.dense = dense;
        self.generation
    }

    fn next_generation(&mut self) {
        self.generation += 1;
    }
}

impl<T> Index<Id<T>> for SparseSet<T> {
    type Output = T;

    fn index(&self, id: Id<T>) -> &Self::Output {
        match self.get(id) {
            Some(element) => element,
            None => panic!("element {id:?} was removed and isn't in the set anymore"),
        }
    }
}

impl<T> IndexMut<Id<T>> for SparseSet<T> {
    fn index_mut(&mut self, id: Id<T>) -> &mut Self::Output {
        match self.get_mut(id) {
            Some(element) => element,
            None => panic!("element {id:?} was removed and isn't in the set anymore"),
        }
    }
}

mod id {
    use std::fmt::Debug;

    use super::*;

    impl<T> Debug for Id<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}(#{})", self.index, self.generation)
        }
    }

    impl<T> Clone for Id<T> {
        fn clone(&self) -> Self {
            Self {
                index: self.index,
                generation: self.generation,
                _phantom_data: PhantomData,
            }
        }
    }

    impl<T> Copy for Id<T> {}

    impl<T> PartialEq for Id<T> {
        fn eq(&self, other: &Self) -> bool {
            self.index == other.index && self.generation == other.generation
        }
    }

    impl<T> Eq for Id<T> {}
}

mod serialization {
    use std::fmt::Display;

    use serde::{
        de::{self, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    use super::*;

    /// Serializing an ID loses its generation, because all serialized sparse sets use generation 0.
    impl<T> Serialize for Id<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            u32::serialize(&self.index, serializer)
        }
    }

    /// Deserializing an ID results in generation 0, because all serialized sparse sets use that.
    impl<'de, T> Deserialize<'de> for Id<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(Self::new(u32::deserialize(deserializer)?, 0))
        }
    }

    /// Serializing a sparse set loses generations from its elements. Each element is serialized
    /// as a struct with an added field `{ "id": 0 }`.
    impl<T> Serialize for SparseSet<T>
    where
        T: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            #[derive(Serialize)]
            struct Intermediate<'a, T> {
                id: Id<T>,
                #[serde(flatten)]
                element: &'a T,
            }

            serializer.collect_seq(
                self.pairs()
                    .map(|(id, element)| Intermediate { id, element }),
            )
        }
    }

    impl<'de, T> Deserialize<'de> for SparseSet<T>
    where
        T: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct SetVisitor<T> {
                _phantom_data: PhantomData<T>,
            }

            impl<'de, T> Visitor<'de> for SetVisitor<T>
            where
                T: Deserialize<'de>,
            {
                type Value = SparseSet<T>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("data")
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: de::SeqAccess<'de>,
                {
                    #[derive(Deserialize)]
                    struct Intermediate<T> {
                        id: Id<T>,
                        #[serde(flatten)]
                        element: T,
                    }

                    let mut set = SparseSet::new();
                    while let Some(Intermediate { id, element }) = seq.next_element()? {
                        set.insert_at_sparse_index(id.index, element)
                            .map_err(de::Error::custom)?;
                    }
                    Ok(set)
                }
            }

            deserializer.deserialize_seq(SetVisitor::<T> {
                _phantom_data: PhantomData,
            })
        }
    }

    pub struct OverlappingId(pub u32);

    impl Display for OverlappingId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "set contains elements with duplicate IDs ({})", self.0)
        }
    }
}

pub struct Ids<'a, T> {
    set: &'a SparseSet<T>,
    dense: std::slice::Iter<'a, Dense>,
}

impl<'a, T> Iterator for Ids<'a, T> {
    type Item = Id<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.dense.next().map(|&Dense { sparse: index }| {
            let generation = self.set.sparse_to_dense[index as usize].generation;
            Id::new(index, generation)
        })
    }
}

pub struct Pairs<'a, T> {
    set: &'a SparseSet<T>,
    ids: Ids<'a, T>,
}

impl<'a, T> Iterator for Pairs<'a, T> {
    type Item = (Id<T>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.ids
            .next()
            .map(|id| (id, self.set.data[id.index as usize].as_ref().unwrap()))
    }
}

pub struct Iter<'a, T> {
    set: &'a SparseSet<T>,
    dense: std::slice::Iter<'a, Dense>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.dense
            .next()
            .map(|&Dense { sparse: index }| self.set.data[index as usize].as_ref().unwrap())
    }
}

impl<'a, T> IntoIterator for &'a SparseSet<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
