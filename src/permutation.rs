use itertools::Itertools;
pub struct Permutation {
    mapping: Vec<usize>,
}

impl Permutation {
    pub fn new(mapping: Vec<usize>) -> Self {
        Self { mapping }
    }

    pub fn identity(n: usize) -> Self {
        Self::new((0..n).collect_vec())
    }

    pub fn all(n: usize) -> impl Iterator<Item = Self> {
        (0..n).permutations(n).map(|m| Self::new(m))
    }

    pub fn apply<T: Clone>(&self, sequence: &[T]) -> Vec<T> {
        let mut result = sequence.to_vec();
        for (from, &to) in self.mapping.iter().enumerate() {
            // println!("res[{to}] = seq[{from}]");
            result[to] = sequence[from].clone();
        }
        result
    }

    pub fn maps(&self, pos: usize) -> usize {
        self.mapping[pos]
    }
}

#[cfg(test)]
mod test {
    use crate::permutation::Permutation;

    #[test]
    fn perm0() {
        let perm = Permutation::new(vec![2, 1, 3, 0]);
        let sequence: Vec<_> = vec!["A", "B", "C", "D"];
        assert_eq!(perm.apply(&sequence), vec!["D", "B", "A", "C"]);
    }

    #[test]
    fn perm1() {
        let perm = Permutation::new(vec![2, 1, 3, 0]);
        assert_eq!(perm.maps(2), 3);
    }
}
