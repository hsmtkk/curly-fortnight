pub fn required_string(key:&str) -> String {
    let msg = format!("{} environment variable must be defined", key);
    std::env::var(key).expect(&msg);
}

pub fn required_u16(key:&str) -> u16 {
    let s = required_string(key);
    let msg = format!("failed to parse {} as u16", s);
    s.parse::<u16>().expect(&msg)
}