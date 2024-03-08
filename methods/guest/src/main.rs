#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);


fn main() {
    // TODO: Implement your guest code here
    fn fibonacci(n: u32) -> u32 {
        let mut a = 0;
        let mut b = 1;
        let mut i = 0;
    
        while i < n {
            let tmp = b;
            b = a + b;
            a = tmp;
            i += 1;
        }
    
        a
    }
    // read the input
    let input: u32 = env::read();

    // TODO: do something with the input
    let output = fibonacci(input);
    // write public output to the journal
    env::commit(&output);
}
