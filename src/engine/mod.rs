pub mod sled;
pub mod mem;

use crate::Result;


pub trait KvEngine: Clone + Send + 'static {

    fn set(&self, key: String, value: String) -> Result<()>;

    fn get(&self, key: String) -> Result<Option<String>>;

    fn remove(&self, key: String) -> Result<()>;
}