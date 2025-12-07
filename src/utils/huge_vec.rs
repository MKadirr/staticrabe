use std::ops::{Index, IndexMut, RangeBounds, Bound};
use std::convert::TryInto;

/// HugeVec: vecteur adressable par index u64 (hi: 32 bits, lo: 32 bits)
pub struct HugeVec<T> {
    chunks: Vec<Vec<T>>, // chunks[hi as usize][lo as usize]
    len: u64,            // longueur logique totale (peut aller jusqu'à u64::MAX)
}

impl<T> HugeVec<T> {
    pub fn new() -> Self {
        Self { chunks: Vec::new(), len: 0 }
    }

    /// calcule (hi, lo) à partir d'un u64
    #[inline]
    fn split_index(idx: u64) -> (u32, u32) {
        let hi = (idx >> 32) as u32;
        let lo = idx as u32;
        (hi, lo)
    }

    /// retourne Option<&T>
    pub fn get(&self, idx: u64) -> Option<&T> {
        let (hi, lo) = Self::split_index(idx);
        let hi_usize: usize = hi as usize;
        let lo_usize: usize = lo as usize;
        self.chunks.get(hi_usize).and_then(|chunk| chunk.get(lo_usize))
    }

    /// mutable
    pub fn get_mut(&mut self, idx: u64) -> Option<&mut T> {
        let (hi, lo) = Self::split_index(idx);
        let hi_usize: usize = hi as usize;
        let lo_usize: usize = lo as usize;
        self.chunks.get_mut(hi_usize).and_then(|chunk| chunk.get_mut(lo_usize))
    }

    /// set: remplace la valeur à idx (si existant). Retourne Err si hors-bornes.
    pub fn set(&mut self, idx: u64, value: T) -> Result<(), &'static str> {
        match self.get_mut(idx) {
            Some(slot) => {
                *slot = value;
                Ok(())
            }
            None => Err("index out of bounds"),
        }
    }


}