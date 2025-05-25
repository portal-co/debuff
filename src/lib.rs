#![no_std]

use core::{ops::DerefMut, slice};

use sha3::Digest;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct DebuffManager<T: DerefMut<Target = [[u8; 32]]>> {
    pub cage: T,
}
impl<T: DerefMut<Target = [[u8; 32]]>> DebuffManager<T> {
    pub fn debuffed(&mut self, a: &str) -> bool {
        let Some(q) = a
            .split("&")
            .flat_map(|s| s.split("?"))
            .find_map(|a| a.strip_prefix("debuff="))
        else {
            return true;
        };
        let q: [u8; 32] = sha3::Sha3_256::digest(q).into();
        let b = match self.cage.binary_search(&q) {
            Ok(a) => {
                self.cage[a] = [0u8; 32];
                true
            }
            Err(_) => {
                for c in self.cage.iter_mut() {
                    if *c == [0u8; 32] {
                        *c = q;
                        break;
                    }
                }
                false
            }
        };
        match self.cage.deref_mut() {
            mut s => {
                let s = &mut *s;
                s.sort_unstable();
            }
        };
        b
    }
}
