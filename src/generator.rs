use rand::Rng;
pub fn gen_ran() -> u8 {
    let mut rng = rand::thread_rng();
    // u8(unsigned int)をしていておくと0~255の整数を返してしてくれる
    let n: u8 = rng.gen();
    n
}
