use std::{cell::{Cell, RefCell}, fmt, rc::Rc};

const MEMORY_SIZE: usize = 2usize.pow(16);
pub const fn raw_address(address: u32) -> usize {
    (address as usize) & 0xFFFF
}

const LINE_SIZE: usize = 4;

pub const fn line_offset(address: usize) -> usize {
    address & 0b11
}

const fn line_address(address: usize) -> usize {
    address >> 2
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MemoryError {
    Busy,
    AtNextLevel,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActionEnum {
    FETCH,
    STORE,
}

#[derive(Debug, Clone)]
pub struct Request {
    requester_id: u32,
    timer: usize,
    address: u32,
    action: ActionEnum,
    next_level: bool,
    next_level_done: bool,
}

pub trait FrontMemory: fmt::Debug {
    fn fetch(&mut self, requester_id: u32, address: u32) -> Result<[u32; LINE_SIZE], MemoryError>;
    fn store(&mut self, requester_id: u32, address: u32, value: u32) -> Result<(), MemoryError>;
    fn cancel(&mut self, requester_id: u32) -> Result<(), MemoryError>;
}

pub trait InnerMemory: fmt::Debug {
    fn read(&self, address: u32) -> Result<u32, MemoryError>;
    fn read_line(&self, address: u32) -> Result<[u32; LINE_SIZE], MemoryError>;
    fn write(&mut self, address: u32, value: u32) -> Result<(), MemoryError>;
    fn has(&mut self, address: u32) -> Result<bool, MemoryError>;
    fn update_line(&mut self, address: u32, line: &[u32; LINE_SIZE]) -> Result<(), MemoryError>;
    fn is_terminal(&self) -> bool;
}

pub trait Cache: fmt::Debug {
    fn raw_line(&self, line: usize) -> [u32; LINE_SIZE];
    fn line_metadata(&self, line: usize) -> CacheStruct;
}

#[derive(Debug)]
pub struct ClockedMemory<const T: usize, MEM: InnerMemory> {
    current_request: Option<Request>,
    memory: Rc<RefCell<MEM>>,
    next: Option<Rc<RefCell<dyn FrontMemory>>>
}

impl<const T: usize, MEM: InnerMemory> ClockedMemory<T, MEM> {
    pub fn new(memory: Rc<RefCell<MEM>>, next: Option<Rc<RefCell<dyn FrontMemory>>>) -> Self {
        assert_ne!(memory.borrow().is_terminal(), next.is_some());
        ClockedMemory {
            current_request: None,
            memory,
            next,
        }
    }

    pub fn current_request(&self) -> Option<Request> {
        self.current_request.clone()
    }

    pub fn inner(&mut self) -> Rc<RefCell<MEM>> {
        self.memory.clone()
    }

    fn request(&mut self, requester_id: u32, address: u32, action: ActionEnum) -> Result<(), MemoryError> {
        let t;

        match &mut self.current_request {
            Some(Request { requester_id: id, timer, address: addr, action: act, next_level, .. }) => {
                if *id != requester_id || *act != action || *addr != address {
                    return Err(MemoryError::Busy)
                } else if *next_level {
                    return Err(MemoryError::AtNextLevel)
                } else {
                    if *timer > 0 {
                        *timer -= 1;
                    }
                    t = *timer;
                }
            }
            None => {
                self.current_request = Some(Request { requester_id, timer: T-1, address, action, next_level: false, next_level_done: false });
                t = T-1;
            }
        }

        if t == 0 {
            match self.memory.borrow_mut().has(address) {
                Ok(true) => Ok(()),
                Ok(false) => {
                    self.current_request.as_mut().unwrap().next_level = true;
                    Err(MemoryError::AtNextLevel)
                }
                Err(e) => Err(e)
            }
        } else {
            Err(MemoryError::Busy)
        }
    }
}

impl<const T: usize, MEM: InnerMemory> FrontMemory for ClockedMemory<T, MEM> {
    fn fetch(&mut self, requester_id: u32, address: u32) -> Result<[u32; LINE_SIZE], MemoryError> {
        match self.request(requester_id, address, ActionEnum::FETCH) {
            Ok(_) => {
                let res = self.memory.borrow_mut().read_line(address);

                if res.is_ok() {
                    self.current_request = None;
                }
    
                res
            }

            Err(MemoryError::AtNextLevel) => match self.next.as_mut().expect("No Terminal Memory").borrow_mut().fetch(requester_id, address) {
                Ok(v) => {
                    self.current_request = None;
                    self.memory.borrow_mut().update_line(address, &v)?;
                    Ok(v)
                }

                Err(e) => Err(e)
            }

            Err(e) => {
                Err(e)
            }
        }
    }

    fn store(&mut self, requester_id: u32, address: u32, value: u32) -> Result<(), MemoryError> {
        let a = self.request(requester_id, address, ActionEnum::STORE);
        let b = match self.next.as_mut() {
            Some(mem) => {
                if self.current_request.as_ref().unwrap().next_level_done {
                    Some(Ok(()))
                } else {
                    let res = mem.borrow_mut().store(requester_id, address, value);

                    if res.is_ok() {
                        self.current_request.as_mut().unwrap().next_level_done = true;
                    }

                    Some(res)
                }
            },
            None => None,
        };
        
        match (a, b)  {
            (Ok(_), None) | (Ok(_), Some(Ok(()))) => {
                let res = self.memory.borrow_mut().write(address, value);
                if res.is_ok() {
                    self.current_request = None;
                }
    
                res
            }

            (Err(MemoryError::AtNextLevel), Some(Ok(()))) => {
                self.current_request = None;
                Ok(())
            }

            // (Ok(()), Some(Err(e))) => {
            //     self.current_request.as_mut().unwrap().next_level = true;
            //     Err(e)
            // }

            (Err(MemoryError::AtNextLevel), None) => panic!("No Terminal Memory"),
            (_, Some(Err(e))) => Err(e),
            (Err(e), _) => Err(e),
        }

        
    }

    fn cancel(&mut self, requester_id: u32) -> Result<(), MemoryError> {
        match &self.current_request {
            Some(Request { requester_id: id, next_level, .. }) => {
                if *id == requester_id {
                    if *next_level {
                        self.next.as_mut().expect("No Terminal Memory").borrow_mut().cancel(requester_id)?;
                    }

                    self.current_request = None;
                    Ok(())
                } else {
                    Err(MemoryError::Busy)
                }
            },
            None => {
                Err(MemoryError::Busy)
            }
        }
    }
}

#[derive(Debug)]
pub struct Memory {
    pub inner: [u32; MEMORY_SIZE]
}

impl Memory {
    pub fn new() -> Self {
        Self {
            inner: [0; MEMORY_SIZE],
        }
    }
}

impl InnerMemory for Memory {
    fn read(&self, address: u32) -> Result<u32, MemoryError> {
        Ok(self.inner[raw_address(address)])
    }

    fn write(&mut self, address: u32, value: u32) -> Result<(), MemoryError> {
        self.inner[raw_address(address)] = value;
        Ok(())
    }

    fn has(&mut self, _address: u32) -> Result<bool, MemoryError> {
        Ok(true)
    }
    
    fn update_line(&mut self, address: u32, line: &[u32; LINE_SIZE]) -> Result<(), MemoryError> {
        let base = raw_address(address) & (!0b11);
        self.inner[base..base+LINE_SIZE].copy_from_slice(line);
        Ok(())
    }
    
    fn read_line(&self, address: u32) -> Result<[u32; LINE_SIZE], MemoryError> {
        let base = raw_address(address) & (!0b11);
        let mut res = [0; LINE_SIZE];
        res.clone_from_slice(&self.inner[base..base+LINE_SIZE]);
        Ok(res)
    }
    
    fn is_terminal(&self) -> bool {
        true
    }
}

#[derive(Debug, Default, Clone)]
pub struct CacheStruct {
    pub tag: usize,
    pub dirty: bool,
    pub valid: bool,
}

#[derive(Debug)]
pub struct DirectCache<const C: usize> {
    cache: [[u32; LINE_SIZE]; C],
    data: [CacheStruct; C],
}

impl<const C: usize> DirectCache<C> {
    pub fn new() -> Self {
        Self {
            cache: [[0; LINE_SIZE]; C],
            data: core::array::from_fn(|_| CacheStruct::default()),
        }
    }

    fn decompose_address(&self, address: u32) -> (usize, usize, usize) {
        let address = raw_address(address);
        let line_addr = line_address(address);
        let line = line_addr % C;
        let offset = line_offset(address);
        let tag = line_addr / C;
        (tag, line, offset)
    }

    fn cache_has(&self, line: usize, tag: usize) -> bool {
        self.data[line].tag == tag && self.data[line].valid == true
    }
}

impl<const C: usize> InnerMemory for DirectCache<C> {
    fn read(&self, address: u32) -> Result<u32, MemoryError> {
        let (tag, line, offset) = self.decompose_address(address);

        if self.cache_has(line, tag) {
            Ok(self.cache[line][offset])
        } else {
            Err(MemoryError::AtNextLevel)
        }
    }

    fn write(&mut self, address: u32, value: u32) -> Result<(), MemoryError> {
        let (tag, line, offset) = self.decompose_address(address);

        if self.cache_has(line, tag) {
            self.cache[line][offset] = value;
            self.data[line].dirty = true;
            Ok(())
        } else {
            Err(MemoryError::AtNextLevel)
        }
    }

    fn has(&mut self, address: u32) -> Result<bool, MemoryError> {
        let (tag, line, _offset) = self.decompose_address(address);

        Ok(self.cache_has(line, tag))
    }
    
    fn update_line(&mut self, address: u32, line: &[u32; LINE_SIZE]) -> Result<(), MemoryError> {
        let (tag, line_index, _offset) = self.decompose_address(address);
        self.cache[line_index].copy_from_slice(line);
        self.data[line_index].valid = true;
        self.data[line_index].dirty = false;
        self.data[line_index].tag = tag;
        Ok(())
    }
    
    fn read_line(&self, address: u32) -> Result<[u32; LINE_SIZE], MemoryError> {
        let (tag, line, _offset) = self.decompose_address(address);

        if self.cache_has(line, tag) {
            Ok(self.cache[line])
        } else {
            Err(MemoryError::AtNextLevel)
        }
    }
    
    fn is_terminal(&self) -> bool {
        false
    }
}

impl<const C: usize> Cache for DirectCache<C> {
    fn raw_line(&self, line: usize) -> [u32; LINE_SIZE] {
        assert!(line < C);
        self.cache[line]
    }

    fn line_metadata(&self, line: usize) -> CacheStruct {
        assert!(line < C);
        self.data[line].clone()
    }
}