use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
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

#[derive(Clone)]
pub enum ImplicitModifier {
    AttackDamage { min: i32, max: i32 },
}

impl ImplicitModifier {
    pub fn reroll(&mut self) {
        match self {
            ImplicitModifier::AttackDamage { min, max } => {
                *min = thread_rng().gen_range(1..=5);
                *max = *min + thread_rng().gen_range(1..=3);
            }
        }
    }
}

impl fmt::Display for ImplicitModifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImplicitModifier::AttackDamage { min, max } => {
                write!(f, " {}-{} ATTACK DAMAGE", min, max)
            }
        }
    }
}
