pub mod version_v16;

pub trait Paths {
    fn get(&self, key: &str) -> Option<&[&str]>;
}
