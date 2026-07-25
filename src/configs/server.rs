pub trait ServerConfig {
    fn get_addr(&self) -> String;
    fn get_export_dir(&self) -> &str;
}
