const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // example of warning for unused variable
    let unused = "aaa";
    // example of error dur to assigning to constant
    READY_AMOUNT = 1;
    let (missiles, ready):(i32,i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    //missiles = missiles - ready;
    println!("{} missiles left", missiles-ready)
}
