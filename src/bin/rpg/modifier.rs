use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Modifier {
    Attack(i32),
    AttackSpeed(i32),
    CastSpeed(i32),
}

impl Distribution<Modifier> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Modifier {
        let which = rng.gen_range(0..=2);
        let bonus = rng.gen_range(-5..=5);

        match which {
            0 => Modifier::Attack(bonus),
            1 => Modifier::AttackSpeed(bonus),
            2 => Modifier::CastSpeed(bonus),
            _ => panic!("Wrong index"),
        }
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modifier::Attack(attack) => write!(f, "{:+} ATTACK", attack),
            Modifier::AttackSpeed(attack_speed) => write!(f, "{:+} ATTACK SPEED", attack_speed),
            Modifier::CastSpeed(cast_speed) => write!(f, "{:+} CAST SPEED", cast_speed),
        }
    }
}
