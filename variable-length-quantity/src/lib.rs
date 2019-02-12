mod iter_decoded;
mod iter_encoded;

pub use iter_decoded::Error;
use iter_decoded::IterDecoded;
use iter_encoded::IterEncoded;

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    IterDecoded::new(bytes.iter()).collect()
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    reversed(IterEncoded::new(values.iter().rev()).collect())
}

fn reversed<T>(mut items: Vec<T>) -> Vec<T> {
    items.reverse();

    items
}
