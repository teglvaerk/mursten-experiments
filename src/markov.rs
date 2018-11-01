use markov::Chain;

pub fn main() {
    let mut chain = Chain::of_order(2);
    chain.feed(vec![1u8, 2, 3, 5]).feed(vec![3u8, 9, 2]);
    println!("{:?}", chain.generate());
    for line in chain.iter_for(5) {
        println!("{:?}", line);
    }
}

