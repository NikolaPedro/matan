pub struct Sum {
    start: i64,
    end: i64,
    function: fn(i64) -> f64,
}

impl Sum {
    pub fn new(start: i64, end: i64, function: fn(i64) -> f64) -> Self {
        Self {
            start,
            end,
            function,
        }
    }

    pub fn get_value(&self, epsilon: f64) -> f64 {
        (self.start..self.end)
            .map(|n| (self.function)(n))
            .take_while(|y| y < &epsilon)
            .sum()
    }
}
