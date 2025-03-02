pub enum MemoryError {

}

pub trait Memory {
    fn fetch(requester_id: u32, address: u32) -> Result<u32, MemoryError>;
    fn store(requester_id: u32, address: u32, value: u32) -> Result<(), MemoryError>;
}