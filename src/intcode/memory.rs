use std::cell::RefCell;

pub struct MemoryManager {
    memory: RefCell<Vec<isize>>,
}

pub trait ReadOnlyMemoryManager {
    fn read(&self, at: usize) -> isize;
    fn read_address(&self, at: usize) -> usize;
    fn dump(&self) -> Vec<isize>;
}

pub trait MutableMemoryManager: ReadOnlyMemoryManager {
    fn write(&self, at: usize, val: isize);
}

impl MemoryManager {
    pub fn new(init: &Vec<isize>) -> MemoryManager {
        let memory = RefCell::new(init.clone());
        MemoryManager { memory }
    }
}

impl ReadOnlyMemoryManager for MemoryManager {
    fn read(&self, at: usize) -> isize {
        self.memory.borrow()[at]
    }

    fn read_address(&self, at: usize) -> usize {
        let pos = self.read(at);

        if pos < 0 || (pos as usize) >= self.memory.borrow().len() {
            panic!(format!("Invalid memory referenced {}", pos));
        }

        pos as usize
    }

    fn dump(&self) -> Vec<isize> {
        self.memory.borrow().clone()
    }
}

impl MutableMemoryManager for MemoryManager {
    fn write(&self, at: usize, val: isize) {
        self.memory.borrow_mut()[at] = val;
    }
}
