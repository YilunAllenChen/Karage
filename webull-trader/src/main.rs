use webull_rs::get_pretty_info;

fn main() {
    let res = get_pretty_info("AAPL").unwrap();
    println!("{res}");
}
