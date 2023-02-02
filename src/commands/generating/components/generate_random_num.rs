use rand::Rng;

pub async fn generate_randnum() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    let first_i: usize = rng.gen();
    let second_i: usize = rng.gen();
    (first_i, second_i)
}
