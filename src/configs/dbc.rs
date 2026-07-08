pub trait DBCConfig {
    fn get_db_url(&self) -> String;
    fn should_create_on_missing(&self) -> bool;
}
