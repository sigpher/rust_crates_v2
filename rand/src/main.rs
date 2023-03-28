use rand::{thread_rng, Rng};
use rand_distr::Alphanumeric;

fn main() {
    println!("{}", gen_random_string(10));
    println!("{}", gen_password_string(10));

    let a = (0..10).map(|i| 1 + i).collect::<Vec<_>>();
    println!("{:?}", a);

    let mut names = Vec::new();
    (0..10).for_each(|_| {
        let name = gen_random_name();
        names.push(name);
    });
    println!("{:?}",names);
}

fn gen_random_string(count: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(count)
        .map(char::from)
        .collect()
}

fn gen_password_string(pwlen: usize) -> String {
    let mut rng = thread_rng();
    const CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";

    let password = (0..pwlen)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    password
}

struct User {
    name: String,
    age: u8,
}

fn gen_random_name() -> String {
    let mut rng = thread_rng();
    let count = rng.gen_range(4..=10);

    let mut name = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(count)
        .map(char::from)
        .collect();
    name
}
