use std::collections::VecDeque;

fn main() {
    let monkeys = vec![
        Monkey::new(vec![92.0, 73.0, 86.0, 83.0, 65.0, 51.0, 55.0, 93.0], |x| x * 5.0, (11.0, 3, 4)),
        Monkey::new(vec![99.0, 67.0, 62.0, 61.0, 59.0, 98.0], |x| x * x, (2.0, 6, 7)),
        Monkey::new(vec![81.0, 89.0, 56.0, 61.0, 99.0], |x| x * 7.0, (5.0, 1, 5)),
        Monkey::new(vec![97.0, 74.0, 68.0], |x| x + 1.0, (17.0, 2, 5)),
        Monkey::new(vec![78.0, 73.0], |x| x + 3.0, (19.0, 2, 3)),
        Monkey::new(vec![50.0], |x| x + 5.0, (7.0, 1, 6)),
        Monkey::new(vec![95.0, 88.0, 53.0, 75.0], |x| x + 8.0, (3.0, 0, 7)),
        Monkey::new(vec![50.0, 77.0, 98.0, 85.0, 94.0, 56.0, 89.0], |x| x + 2.0, (13.0, 4, 0)),
        ];
    let test_monkeys = vec![
        Monkey::new(vec![79.0, 98.0], |x| x * 19.0, (23.0, 2, 3)),
        Monkey::new(vec![54.0, 65.0, 75.0, 74.0], |x| x + 6.0, (19.0, 2, 0)),
        Monkey::new(vec![79.0, 60.0, 97.0], |x| x * x, (13.0, 1, 3)),
        Monkey::new(vec![74.0], |x| x + 3.0, (17.0, 0, 1)),
        ];

    part1(monkeys.clone());

    part2(monkeys.clone());
    
}

fn part1(monkeys: Vec<Monkey>) {
    let mut canopy = Canopy { monkeys: monkeys, relief: 3.0, common_denominator: None };
    for _ in 0..20 {
        canopy.run();
    }
    println!("Monkey business: {}", canopy.monkey_business());
}

fn part2(monkeys: Vec<Monkey>) {
    let mut canopy = Canopy::new(monkeys, 1.0);
    for _ in 0..10000 {
        canopy.run();
    }
    println!("Monkey business: {}", canopy.monkey_business());
}



#[derive(Clone)]
struct Monkey {
    items: VecDeque<f64>,
    operation: fn (f64) -> f64,
    test: (f64, usize, usize),
    pub activity: u64,
}

impl Monkey {
    pub fn new(items: Vec<f64>, operation: fn (f64) -> f64, test: (f64, usize, usize)) -> Monkey {
        Monkey {
            items: VecDeque::from(items),
            operation,
            test,
            activity: 0,
        }
    }

    fn evaluate_item(&mut self) -> f64 {
        self.activity += 1;
        (self.operation)(self.items[0])
    }

    fn test_item(&self, ) -> usize {
        let (test, monkey_true, monkey_false) = self.test;
        if self.items[0] % test == 0.0 {
            monkey_true
        } else {
            monkey_false
        }
    }

    pub fn process(&mut self, idx: usize, relief: f64, common_denominator: Option<f64>) -> (usize, f64) {
        let mut target_idx: usize = 0;
        if self.items.len() == 0 {
            panic!("Monkey has no items")
        }

        if common_denominator.is_some() {
            self.items[0] = self.evaluate_item() % common_denominator.unwrap();
            target_idx = self.test_item();
        } else {
            self.items[0] = (self.evaluate_item() / relief).floor();
            target_idx = self.test_item();
        }
        
        (target_idx, self.items.pop_front().unwrap())
    }

}

#[derive(Clone)]
struct Canopy {
    monkeys: Vec<Monkey>,
    relief: f64,
    pub common_denominator: Option<f64>,
}

impl Canopy {
    pub fn new(monkeys: Vec<Monkey>, relief: f64) -> Canopy {
        let mut canopy = Canopy {
            monkeys,
            relief,
            common_denominator: None,
        };
        canopy.set_common_denominator();
        canopy
    }
    fn take_turn(&mut self, idx: usize) {
        let mut monkey = self.monkeys[idx].clone();
        while monkey.items.len() > 0 {
            let (target_idx, item) = monkey.process(idx, self.relief, self.common_denominator);
            self.monkeys[target_idx].items.push_back(item);
        }
        self.monkeys[idx] = monkey;
    }

    pub fn run(&mut self) {
        let mut idx = 0;
        while idx < self.monkeys.len() {
            self.take_turn(idx);
            idx += 1;
        }
    }

    pub fn monkey_business(&self) -> u64 {
        let mut sorted_monkeys = self.monkeys.clone().into_iter().map(|monkey| monkey.activity).collect::<Vec<u64>>();
        sorted_monkeys.sort();
        sorted_monkeys[sorted_monkeys.len() - 2] * sorted_monkeys[sorted_monkeys.len() - 1]
    }

    // Needed a hint to figure this one out
    // I get it now, but I'm not sure I would have ever thought of it
    // Setting the worry level to the remainder of the modulo of the common denominator doesn't change the test outcomes
    // That's because, the common denominator being the product of all the test numbers, the remainder of the modulo of the common denominator
    // preserves the divisibility properties of the individual test. 
    // 
    fn set_common_denominator(&mut self) {
        self.common_denominator = self.monkeys.clone().into_iter().map(|monkey| monkey.test.0).reduce(|a, b| a * b);
    }
}
