use bevy::prelude::*;
use rand::{RngCore, SeedableRng, rngs::SmallRng};

pub type LocalRng<'s> = Local<'s, Seeded<SmallRng>>;

pub struct Seeded<R>(R);

impl<R> Default for Seeded<R>
where
    R: SeedableRng,
{
    fn default() -> Self {
        Seeded(R::from_os_rng())
    }
}

impl<R> RngCore for Seeded<R>
where
    R: RngCore,
{
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    fn fill_bytes(&mut self, dst: &mut [u8]) {
        self.0.fill_bytes(dst)
    }
}
