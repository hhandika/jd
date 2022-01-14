use std::path::PathBuf;

const LSU_PROXIE: &str = "http://libezp.lib.lsu.edu/login?url=";
const CUSTOM_PROXY_PATH: &str = "ezproxy.txt";

pub fn generate_proxy_link(url: &str) -> String {
    let custom_path = get_custom_proxy_path();
    if custom_path.is_file() {
        let proxy_url = std::fs::read_to_string(custom_path)
            .expect("Failed to read proxy.txt")
            .trim()
            .to_string();
        println!("Using proxy: {}", proxy_url);
        format!("{}{}", proxy_url, url)
    } else {
        println!("Using default proxy: {}", LSU_PROXIE);
        format!("{}{}", LSU_PROXIE, url)
    }
}

pub fn set_proxy(url: &str) {
    let proxy_path = get_custom_proxy_path();
    std::fs::write(proxy_path, url).expect("Failed to write ezproxy.txt");
}

pub fn reset_proxy() {
    let proxy_path = get_custom_proxy_path();
    std::fs::remove_file(proxy_path).expect("Failed to remove ezproxy.txt");
}

fn get_custom_proxy_path() -> PathBuf {
    std::env::current_exe()
        .expect("Failed to get current exe")
        .parent()
        .expect("Failed to get parent directory")
        .join(CUSTOM_PROXY_PATH)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_proxy_link() {
        let url = "http://www.google.com";
        let proxy_url = generate_proxy_link(url);
        assert_eq!(
            proxy_url,
            "http://libezp.lib.lsu.edu/login?url=http://www.google.com"
        );
    }
}
