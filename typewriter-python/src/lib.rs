//! # typewriter-python
//!
//! Python Pydantic v2 / dataclass emitter for the typewriter type sync SDK.
//! Generates `.py` files with Pydantic `BaseModel` classes.

mod emitter;
mod mapper;

pub use mapper::PythonMapper;
