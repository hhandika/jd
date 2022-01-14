use std::path::Path;

const LSU_PROXIE: &str = "http://libezp.lib.lsu.edu/login?url=";
const COSTUM_PROXY_PATH: &str = "proxy.txt";

pub fn generate_proxy_link(url: &str) -> String {
    let costume_path = Path::new(COSTUM_PROXY_PATH);
    if costume_path.is_file() {
        let proxy_url = std::fs::read_to_string(costume_path)
            .expect("Failed to read proxy.txt")
            .trim()
            .to_string();
        println!("Using proxy: {}", proxy_url);
        format!("{}{}", proxy_url, url)
    } else {
        format!("{}{}", LSU_PROXIE, url)
    }
}
