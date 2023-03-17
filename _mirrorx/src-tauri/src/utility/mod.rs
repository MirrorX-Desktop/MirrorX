pub fn format_device_id(device_id: i64) -> String {
    let mut device_id = format!("{device_id:0>10}");
    device_id.insert(2, '-');
    device_id.insert(7, '-');
    device_id
}
