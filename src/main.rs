struct MyNumber {
    num: i32,
}

impl MyNumber {
    pub fn new() -> Self {
        Self { num: 0 }
    }

    pub fn add(mut self, num: i32) -> Self {
        self.num += num;

        self
    }

    pub fn subtract(mut self, num: i32) -> Self {
        self.num -= num;

        self
    }

    pub fn get(self) -> i32 {
        self.num
    }
}

fn main() {
    let num = MyNumber::new()
        .add(10)
        .subtract(5)
        .add(5)
        .add(100)
        .subtract(34)
        .add(275)
        .get();

    println!("{num}");
}
