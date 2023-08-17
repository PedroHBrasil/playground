use chrono;

struct Block<T> {
    index: usize,
    timestamp: i64,
    proof: u64,
    prev_hash: u64,
    data: T,
}

impl<T> Block<T> {
    /// Builds a new block
    fn build(index: usize, proof: u64, prev_hash: u64, data: T) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        Self {
            index,
            timestamp,
            proof,
            prev_hash,
            data,
        }
    }
}

struct Blockchain<T> {
    chain: Vec<Block<T>>
}

impl<T> Blockchain<T> {

    fn build() -> Self {
        unimplemented!()
    }
}