pub struct Library {}

impl ftd::p2::Library for Library {
    fn get(&self, name: &str) -> Option<String> {
        if name == "fpm" {
            return Some(fpm::fpm_ftd().to_string());
        }
        if let Some(v) = std::fs::read_to_string(format!("./{}.ftd", name)).ok() {
            return Some(v);
        } else {
            return std::fs::read_to_string(format!("./.packages/{}.ftd", name)).ok();
        }
    }
}
