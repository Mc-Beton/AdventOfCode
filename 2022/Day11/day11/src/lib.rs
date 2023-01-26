#[derive(Debug)]
pub struct Monkey<'a> {
    pub items: Vec<usize>,
    pub operation: Operation<'a>,
    pub test: usize,
    pub pass: usize,
    pub fail: usize,
    pub inspections: usize,
}

#[derive(Debug)]
pub struct Operation<'a> {
    pub sub: &'a str,
    pub op: char
}

impl Operation<'_> {
    pub fn get_worry_level(&self, old: usize) -> usize {
        let sub;
        if self.sub.to_string() == "old" { sub = old; } 
        else { sub = self.sub.parse::<usize>().unwrap(); }

        match self.op {
            '+' => (old + sub)/3,
            '*' => (old * sub)/3,
            _ => 0,
        }
    }
}

impl Monkey<'_> {
    pub fn test_item(&self, item: usize) -> usize {
        item % self.test
    }
}