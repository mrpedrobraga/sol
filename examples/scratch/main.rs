struct MyNums {
    position: usize,
    nums: Vec<i32>
}

impl Iterator for MyNums {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.nums.get(self.position);
        self.position += 1;
        value.copied()
    }
}

fn main() {
    let nums = MyNums {
        position: 0,
        nums: vec![0, 1, 2, 3, 4, 5, 6]
    };

    let first_nums = nums.take(3).collect::<Vec<_>>();
}