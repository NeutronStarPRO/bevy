mod gltf;

pub use self::gltf::load_gltf;

use std::{collections::HashMap, marker::PhantomData};

pub struct Handle<T> {
    pub id: usize,
    marker: PhantomData<T>,
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Handle {
            id: self.id.clone(),
            marker: PhantomData,
        }
    }
}

pub trait Asset<D> {
    fn load(descriptor: D) -> Self;
}

pub struct AssetStorage<T>
{
    assets: HashMap<usize, T>,
    names: HashMap<String, usize>,
    current_index: usize,
}

impl<T> AssetStorage<T>
{
    pub fn new() -> AssetStorage<T> {
        AssetStorage {
            assets: HashMap::new(),
            names: HashMap::new(),
            current_index: 0,
        }
    }

    pub fn get_named(&mut self, name: &str) -> Option<&mut T> {
        match self.names.get(name) {
            Some(id) => self.assets.get_mut(id),
            None => None,
        }
    }

    pub fn add(&mut self, asset: T) -> Handle<T> {
        let id = self.current_index;
        self.current_index += 1;
        self.assets.insert(id, asset);
        Handle {
            id,
            marker: PhantomData,
        }
    }

    pub fn add_named(&mut self, asset: T, name: &str) -> Handle<T> {
        let handle = self.add(asset);
        self.names.insert(name.to_string(), handle.id);
        handle
    }

    pub fn get(&mut self, id: usize) -> Option<&mut T> {
        self.assets.get_mut(&id)
    }
}
