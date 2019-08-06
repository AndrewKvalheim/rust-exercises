use std::iter::Extend;

#[derive(Clone, Debug)]
pub struct CustomSet<T>(Vec<T>);

impl<T> CustomSet<T> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T: Clone + PartialEq> CustomSet<T> {
    pub fn new(elements: &[T]) -> Self {
        let mut set = Self(Vec::new());

        set.extend(elements);

        set
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
        let mut set = self.clone();

        set.extend(&other.0);

        set
    }
}

impl<T: PartialEq> CustomSet<T> {
    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.0.push(element);
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.0.contains(element)
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.0.iter().all(|e| !other.contains(e))
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.0.iter().all(|e| other.contains(e))
    }
}

impl<'a, T: 'a + Clone + PartialEq> Extend<&'a T> for CustomSet<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, elements: I) {
        elements.into_iter().cloned().for_each(|e| self.add(e));
    }
}

impl<T: PartialEq> PartialEq for CustomSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len() && self.is_subset(other)
    }
}
