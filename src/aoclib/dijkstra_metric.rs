#[derive(Debug, Clone, Copy)]
pub enum DijkstraMetric<V>
where
    V: std::fmt::Debug + Clone + Copy,
{
    Finite(V),
    Infinite,
}

impl<V: PartialEq + std::fmt::Debug + Clone + Copy> std::cmp::PartialEq for DijkstraMetric<V> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Finite(_), Self::Infinite) => false,
            (Self::Infinite, Self::Finite(_)) => false,
            (Self::Finite(a), Self::Finite(b)) => a.eq(b),
            (Self::Infinite, Self::Infinite) => true,
        }
    }
}

impl<V: Eq + std::fmt::Debug + Clone + Copy> std::cmp::Eq for DijkstraMetric<V> {}

impl<V: PartialOrd + std::fmt::Debug + Clone + Copy + PartialEq + Eq> PartialOrd
    for DijkstraMetric<V>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;

        match (self, other) {
            (Self::Finite(_), Self::Infinite) => Some(Less),
            (Self::Infinite, Self::Finite(_)) => Some(Greater),
            (Self::Finite(a), Self::Finite(b)) => a.partial_cmp(b),
            (Self::Infinite, Self::Infinite) => Some(Equal),
        }
    }
}

impl<V: Ord + std::fmt::Debug + Clone + Copy + PartialEq + Eq> Ord for DijkstraMetric<V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        match (self, other) {
            (Self::Finite(_), Self::Infinite) => Less,
            (Self::Infinite, Self::Finite(_)) => Greater,
            (Self::Finite(a), Self::Finite(b)) => a.cmp(b),
            (Self::Infinite, Self::Infinite) => Equal,
        }
    }
}

impl<V: std::fmt::Debug + Clone + Copy> DijkstraMetric<V> {
    pub fn unwrap(&self) -> V {
        match self {
            Self::Finite(a) => *a,
            Self::Infinite => panic!("expected finite value"),
        }
    }
}
