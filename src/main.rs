use rand::Rng;

fn main() {
    let mut i = 1;
    let dices: [i32; 7] = [4, 6, 8, 10, 10, 12, 20];
    let perfection = dices.iter().sum();

    let mut sum = 0;
    while sum != perfection {
        sum = 0;
        for dice in dices.iter() {
            let random = rand::thread_rng().gen_range(1, dice + 1);
            if random != *dice {
                continue;
            }
            sum += random;
        }
        i += 1;
        if i % 500_000 == 0 {
            println!("{} attemps made", i);
        }
    }

    println!("Done in {} attempts", i);
}
