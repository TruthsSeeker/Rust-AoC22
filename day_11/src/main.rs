fn main() {
    println!("Hello, world!");
}

struct Monkey {
    items: Vec<i64>,
    operation: fn (i64) -> i64,
    test: (i64, usize, usize),
}

impl Monkey {
    pub fn new(items: Vec<i64>, operation: fn (i64) -> i64, test: (i64, usize, usize)) -> Monkey {
        Monkey {
            items,
            operation,
            test,
        }
    }

    fn evaluate_item(&self, idx: usize) -> i64 {
        (self.operation)(self.items[idx])
    }

    fn test_item(&self, idx: usize) -> usize {
        let (test, monkey_true, monkey_false) = self.test;
        if self.items[idx] % test == 0 {
            monkey_true
        } else {
            monkey_false
        }
    }

    

}