macro_rules! greet {
    ($name: expr) => {
        let name = $name;
        println!("Hello {name}");
    };
}

macro_rules! vectorize {
    ($size: expr) => {{
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let mut numbers = Vec::new();
        for _ in 0..$size {
            numbers.push(rng.gen::<u8>());
        }
        numbers
    }};
}

pub(crate) use greet;
pub(crate) use vectorize;
