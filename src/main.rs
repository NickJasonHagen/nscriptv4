mod nscriptengine {
    pub mod core;
    pub mod parser;
    pub mod storage;
    pub mod types;

}
pub use nscriptengine::core::*;
pub use nscriptengine::parser::*;
pub use nscriptengine::storage::*;
pub use nscriptengine::types::*;
fn main() {
    println!("Hello, world!");
}
