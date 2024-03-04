use super::{
    model,
    solver,
};

impl model::Pairs {
    pub fn run(&self) {
        let input = self.parse();
        println!("from: {:?}", &input.from);
        println!("target: {:?}", input.target);
        let result = solver::find_sum(&input.from, input.target);
        let result: Vec<(i32, i32)> = result
            .iter()
            .map(|unordered_pair| unordered_pair.into_ordered_tuple())
            .collect();
        println!("result: {result:?}");
    }
}
