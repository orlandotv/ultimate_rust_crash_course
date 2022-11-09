const STARTING_MISSILES: i32 = 8;
const READT_AMOUNT: i32 = 2;
fn main() {
    let _x = 1;
    //let  (mut missiles, ready) = (STARTING_MISSILES, READT_AMOUNT);
    let  (missiles, ready) = (STARTING_MISSILES, READT_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    //missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}
