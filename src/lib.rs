// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

// Gererate random values
mod generator;
pub fn print_number() {
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}
