const LSU_PROXIE: &'static str = "http://libezp.lib.lsu.edu/login?url=";

pub fn generate_proxy_link(url: &str) -> String {
    format!("{}{}", LSU_PROXIE, url)
}