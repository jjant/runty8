use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Modifier {
    Attack(i32),
    AttackSpeed(i32),
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Modifier::Attack(attack) => write!(f, "{:+} ATTACK", attack),
            Modifier::AttackSpeed(attack_speed) => write!(f, "{:+} ATTACK SPEED", attack_speed),
        }
    }
}
