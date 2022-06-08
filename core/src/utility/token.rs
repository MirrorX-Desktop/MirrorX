use anyhow::bail;

pub fn parse_register_token(token: &str) -> anyhow::Result<(String, u32, String)> {
    let splited: Vec<&str> = token.split(".").collect();
    if splited.len() != 3 {
        bail!("parse_register_token: token format is invalid");
    }

    // todo: check device_id format
    let expiration = u32::from_str_radix(splited[1], 10)?;

    Ok((splited[0].to_owned(), expiration, splited[2].to_owned()))
}
