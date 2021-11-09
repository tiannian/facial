pub struct ChunkIndex {
    pub id: [u8; 32],
    pub hash: [u8; 32],
    pub public_key: [u8; 32],
}

pub struct ChunkData {
    pub data: Vec<u8>,
}

pub enum FileData {
    ChunkIndexList(Vec<ChunkIndex>),
    ChunkData(ChunkData),
}

pub struct File {
    pub root: [u8; 32],
    pub signature: [u8; 32],
    pub length: u64,
    pub data: FileData,
}
