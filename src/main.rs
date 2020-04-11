pub mod mos;

fn main() {
    let mos = mos::get("KFIT").unwrap();
    // let lines = mos.raw_mos.split("\n").map(|line| line);
    // for line in lines {
    //     println!("{:?}", line);
    // }
    for entry in mos.entries.iter() {
        println!("{:?}", entry);
    }
}
