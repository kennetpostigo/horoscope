use async_channel::unbounded;
use async_std::task;
use horoscope::event::EventEmitter;
use k9::assert_equal;

#[test]
fn emit_test() {
  task::block_on(async {
    let (w, r) = unbounded();
    let mut emitter = EventEmitter::new();
    let mut counter: i32 = 0;

    let (w2, w3) = (w.clone(), w.clone());

    emitter.on(String::from("increment"), move |_v: ()| {
      task::block_on(async { w2.send("increment").await.unwrap() })
    });

    emitter.on(String::from("decrement"), move |_v: ()| {
      task::block_on(async { w3.send("decrement").await.unwrap() })
    });

    emitter.emit(String::from("increment"), ()).await;
    emitter.emit(String::from("increment"), ()).await;
    emitter.emit(String::from("decrement"), ()).await;
    emitter.emit(String::from("increment"), ()).await;

    while !r.is_empty() {
      match r.recv().await {
        Ok("increment") => counter = counter + 1,
        Ok("decrement") => counter = counter - 1,
        _ => (),
      }
    }

    assert_equal!(counter, 2, "Counter should equal 2");
  });
}

#[test]
fn emit_with_multi_listenertest() {
  task::block_on(async {
    let (w, r) = unbounded();
    let mut emitter = EventEmitter::new();
    let mut counter: i32 = 0;

    let (w2, w3) = (w.clone(), w.clone());

    emitter.on(String::from("increment"), move |_v: ()| {
      task::block_on(async { w2.send("increment").await.unwrap() })
    });

    emitter.on(String::from("increment"), move |_v: ()| {
      task::block_on(async {
        w3.send("increment").await.unwrap();
        w3.send("increment").await.unwrap();
      })
    });

    emitter.emit(String::from("increment"), ()).await;

    while !r.is_empty() {
      match r.recv().await {
        Ok("increment") => counter = counter + 1,
        _ => (),
      }
    }

    assert_equal!(
      counter,
      3,
      "Emit with multilistener should hit all listeners"
    );
  });
}

#[test]
fn off_test() {
  task::block_on(async {
    let (w, r) = unbounded();
    let mut emitter = EventEmitter::new();
    let mut counter: i32 = 0;
    let w2 = w.clone();

    let listener = emitter.on(String::from("increment"), move |_v: ()| {
      task::block_on(async { w2.send("increment").await.unwrap() })
    });

    emitter.emit(String::from("increment"), ()).await;
    emitter.off(listener);
    emitter.emit(String::from("increment"), ()).await;

    while !r.is_empty() {
      match r.recv().await {
        Ok("increment") => counter = counter + 1,
        _ => (),
      }
    }

    assert_equal!(counter, 1, "Emit should do nothing after off");
  });
}
