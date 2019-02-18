#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Bucket {
    One,
    Two,
}

#[derive(Debug, Eq, PartialEq)]
pub struct BucketStats {
    pub goal_bucket: Bucket,
    pub moves: u8,
    pub other_bucket: u8,
}

impl BucketStats {
    pub fn new(names: (Bucket, Bucket), index: usize, amounts: (u8, u8), first: bool) -> Self {
        Self {
            goal_bucket: if first { names.0 } else { names.1 },
            moves: 1 + index as u8,
            other_bucket: if first { amounts.1 } else { amounts.0 },
        }
    }
}
