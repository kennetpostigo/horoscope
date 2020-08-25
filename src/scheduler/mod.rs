// Blocking -> simple job runner until program is killed
// Background -> wrapping around some sort of server

pub mod blocking;
pub mod background;

pub trait Schedule {
  
}