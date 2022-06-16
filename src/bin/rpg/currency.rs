use crate::Item;
use rand::Rng;

use super::{item::Wearable, modifier::Modifier};

#[derive(Clone, Debug)]
pub enum Currency {
    Chaos,
    Blessed,
}

impl Currency {
    pub fn apply(&self, item: &mut Item) {
        if let Some(wearable) = item.to_wearable_mut() {
            match self {
                Currency::Chaos => chaos_orb(wearable),
                Currency::Blessed => blessed_orb(wearable),
            }
        }
    }
}

// TODO: This can currently pick duplicate mods
fn chaos_orb(item: &mut Wearable) {
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

fn blessed_orb(item: &mut Wearable) {
    for implicit in item.implicits.iter_mut() {
        implicit.reroll()
    }
}
