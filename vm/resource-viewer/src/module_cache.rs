// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use starcoin_vm_types::{file_format::CompiledModule, language_storage::ModuleId};
use std::{cell::RefCell, collections::hash_map::HashMap, hash::Hash, rc::Rc};

#[derive(Clone)]
pub struct ModuleCacheImpl<K, V> {
    id_map: RefCell<HashMap<K, usize>>,
    modules: RefCell<Vec<Rc<V>>>,
}

impl<K, V> ModuleCacheImpl<K, V>
where
    K: Eq + Hash,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            id_map: RefCell::new(HashMap::new()),
            modules: RefCell::new(vec![]),
        }
    }

    pub fn insert(&self, key: K, module: V) -> Rc<V> {
        self.modules.borrow_mut().push(Rc::new(module));
        let idx = self.modules.borrow().len() - 1;
        self.id_map.borrow_mut().insert(key, idx);
        self.modules
            .borrow()
            .last()
            .expect("ModuleCache: last() after push() impossible failure")
            .clone()
    }

    pub fn get(&self, key: &K) -> Option<Rc<V>> {
        self.id_map
            .borrow()
            .get(&key)
            .and_then(|idx| self.modules.borrow().get(*idx).cloned())
    }
}

pub type ModuleCache = ModuleCacheImpl<ModuleId, CompiledModule>;
