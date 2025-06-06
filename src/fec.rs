use std::collections::HashMap;

pub struct FuckExCel {
    cells: HashMap<String, i32>,
    cache: HashMap<String, i32>,
}

pub struct Cell<'a> {
    name: String,
    book: &'a mut FuckExCel,
}

impl FuckExCel {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            cache: HashMap::new(),
        }
    }

    pub fn cell(&mut self, name: &str) -> Cell {
        Cell {
            name: name.to_string(),
            book: self,
        }
    }

    fn recalc(&mut self) {
        self.cache.clear(); // 今回はキャッシュは使ってないけど一応用意だけ
    }

    fn eval(&self, cell: &str) -> Option<i32> {
        self.cells.get(cell).copied()
    }
}

impl<'a> Cell<'a> {
    pub fn set(&mut self, val: i32) {
        self.book.cells.insert(self.name.clone(), val);
        self.book.recalc();
    }

    pub fn get(&mut self) -> Option<i32> {
        self.book.eval(&self.name)
    }
}
