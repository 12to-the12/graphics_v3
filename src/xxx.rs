struct mystruct {
    data: Vec<usize>,
}

impl mystruct {
    fn get_data(&mut self) -> impl Iterator<Item = &mut usize> {
        self.data.iter_mut()
    }
}

fn main() {
    println!("hello world!");
    let mut x = mystruct {
        data: vec![1, 2, 3],
    };
    for x in x.get_data() {
        *x += 1;
    }
    for x in x.get_data() {
        println!("{x}");
    }
}
