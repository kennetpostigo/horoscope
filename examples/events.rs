use horoscope::event::EventEmitter;

#[async_std::main]
async fn main() {
  let mut emitter = EventEmitter::new();
  emitter.on(String::from("Test"), |v: String| println!("Worked {}!", v));
  emitter.emit(String::from("Test"), String::from("1")).await;
  emitter.emit(String::from("Test"), String::from("2")).await;
  emitter.emit(String::from("Test"), String::from("3")).await;
}
