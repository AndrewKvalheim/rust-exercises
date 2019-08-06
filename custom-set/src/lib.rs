#[derive(Clone, Debug, PartialEq)]
pub struct CustomSet<T>(Vec<T>);

impl<T> CustomSet<T> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Ord + PartialEq> CustomSet<T> {
    pub fn add(&mut self, element: T) {
        if let Err(position) = self.0.binary_search(&element) {
            self.0.insert(position, element);
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.0.binary_search(element).is_ok()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        match (self.0.len(), other.0.len()) {
            (0, _) | (_, 0) => true,
            (s, o) => self.0[s - 1] < other.0[0] || other.0[o - 1] < self.0[0],
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        match (self.0.len(), other.0.len()) {
            (0, _) => true,
            (s, o) if s > o => false,
            (s, _) => other
                .0
                .binary_search(&self.0[0])
                .map(|i| &other.0[i..s] == self.0.as_slice())
                .unwrap_or(false),
        }
    }
}

impl<T: Clone + Ord + PartialEq> CustomSet<T> {
    pub fn new(elements: &[T]) -> Self {
        let mut elements = elements.to_vec();

        elements.sort();
        elements.dedup();

        Self(elements)
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .filter(|e| !other.contains(e))
                .cloned()
                .collect(),
        )
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self(
            self.0
                .iter()
                .filter(|e| other.contains(e))
                .cloned()
                .collect(),
        )
    }

    pub fn union(&self, other: &Self) -> Self {
        other.0.iter().fold(self.clone(), |mut set, element| {
            if let Err(position) = self.0.binary_search(element) {
                set.0.insert(position, element.clone());
            }

            set
        })
    }
}
