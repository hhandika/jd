use std::io::Result;

const LSU_PROXIE: &str = "http://libezp.lib.lsu.edu/login?url=";
const COSTUM_PROXY_PATH: &str = "proxy.txt";

pub fn generate_proxy_link(url: &str) -> Result<String> {
    let costume_path = std::env::current_exe()?
        .parent()
        .expect("Failed to get parent directory")
        .join(COSTUM_PROXY_PATH);
    if costume_path.is_file() {
        let proxy_url = std::fs::read_to_string(costume_path)
            .expect("Failed to read proxy.txt")
            .trim()
            .to_string();
        println!("Using proxy: {}", proxy_url);
        Ok(format!("{}{}", proxy_url, url))
    } else {
        println!("Using default proxy: {}", LSU_PROXIE);
        Ok(format!("{}{}", LSU_PROXIE, url))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_proxy_link() {
        let url = "http://www.google.com";
        let proxy_url = generate_proxy_link(url).unwrap();
        assert_eq!(
            proxy_url,
            "http://libezp.lib.lsu.edu/login?url=http://www.google.com"
        );
    }
}
