#[test]
fn os_detect() {
    let info = os_info::get();
    println!("{}", info);
    println!("Type: {}", info.os_type());
    println!("Version: {}", info.version());
    println!("Edition: {:?}", info.edition());
    println!("Codename: {:?}", info.codename());
    println!("Bitness: {}", info.bitness());
}
