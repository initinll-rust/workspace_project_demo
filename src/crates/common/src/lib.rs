use rand;

pub fn common_print(caller: &str) {
    let x = rand::random::<u8>();
    println!("\t\tCommon invoked from {} | random no {}",caller, x);
}
