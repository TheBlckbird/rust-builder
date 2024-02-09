struct MyNumber {
    pub num: i32,
}

impl MyNumber {
    pub fn new() -> Self {
        Self { num: 0 }
    }

    pub fn add_one(mut self) -> Self {
        self.num += 1;

        self
    }

    pub fn add_two(mut self) -> Self {
        self.num += 2;

        self
    }

    pub fn add_three(mut self) -> Self {
        self.num += 3;

        self
    }

    pub fn subtract_one(mut self) -> Self {
        self.num -= 1;

        self
    }

    pub fn subtract_two(mut self) -> Self {
        self.num -= 2;

        self
    }

    pub fn subtract_three(mut self) -> Self {
        self.num -= 3;

        self
    }

    pub fn get(self) -> i32 {
        self.num
    }
}

fn main() {
    let builder_struct = MyNumber::new();

    let num = builder_struct
        .add_one()
        .subtract_three()
        .add_two()
        .subtract_two()
        .add_three()
        .subtract_two()
        .add_three()
        .add_two()
        .subtract_one()
        .get();

    println!("{num}");
}
