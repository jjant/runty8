use crate::Item;
use rand::Rng;

use super::modifier::Modifier;

pub enum Currency {
    Chaos,
}

impl Currency {
    pub fn apply(&self, item: &mut Item) {
        match self {
            Currency::Chaos => chaos_orb(item),
        }
    }
}

// TODO: This can currently pick duplicate mods
fn chaos_orb(item: &mut Item) {
    let mut rng = rand::thread_rng();
    let how_many_mods = rng.gen_range(1..=3);

    let mods = vec![0; how_many_mods]
        .into_iter()
        .map(|_| {
            let modifier: Modifier = rng.gen();

            modifier
        })
        .collect();

    item.mods = mods;
}
