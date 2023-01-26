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
            '+' => old + sub,
            '*' => old * sub,
            _ => 0,
        }
    }

    pub fn get_sub(&self) -> usize {
        let sub;
        if self.sub.to_string() == "old" { sub = 1; } 
        else { sub = self.sub.parse::<usize>().unwrap(); }
        sub
    }
}

impl Monkey<'_> {
    pub fn test_item(&self, item: usize) -> usize {
        item % self.test
    }
}