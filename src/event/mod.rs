// function mitt(all) {
// 	all = all || Object.create(null);
//
// 	return {
// 		on(type, handler) {
// 			(all[type] || (all[type] = [])).push(handler);
// 		},
// 		off(type, handler) {
// 			let e = all[type] || (all[type] = []);
// 			e.splice(e.indexOf(handler) >>> 0, 1);
// 		},
// 		emit(type, evt) {
// 			(all[type] || []).map((handler) => { handler(evt); });
// 			(all['*'] || []).map((handler) => { handler(type, evt); });
// 		}
// 	};
// }
//
// let emitter = mitt()
// // listen to an event
// emitter.on('foo', e => console.log('foo', e) )
// // listen to all events
// emitter.on('*', (type, e) => console.log(type, e) )
// // fire an event
// emitter.emit('foo', { a: 'b' })
// // working with handler references:
// function onFoo() {}
// emitter.on('foo', onFoo)   // listen
// emitter.off('foo', onFoo)  // unlisten

use bincode::{deserialize, serialize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Listener {
  callback: Box<dyn Fn(Vec<u8>) + Sync + Send + 'static>,
  id: i64,
}

pub struct EventEmitter {
  pub listeners: HashMap<String, Vec<Listener>>,
  count: i64,
}

impl EventEmitter {
  pub fn new() -> Self {
    Self {
      listeners: HashMap::new(),
      count: 0,
    }
  }

  pub fn on<T, C>(&mut self, t: String, callback: C) -> i64
  where
    for<'a> T: Deserialize<'a>,
    C: Fn(T) + 'static + Sync + Send, {
    let id = self.count + 1;

    let cb = move |bytes: Vec<u8>| {
      let value: T = deserialize(&bytes).unwrap();
      callback(value);
    };

    let listener = Listener {
      id: id.clone(),
      callback: Box::new(cb),
    };

    match self.listeners.get_mut(&t) {
      Some(callbacks) => {
        callbacks.push(listener);
      }
      None => {
        self.listeners.insert(t, vec![listener]);
      }
    }

    return id;
  }

  pub fn off<F>(&mut self, id: i64) -> Option<i64> {
    for (_, listeners) in self.listeners.iter_mut() {
      if let Some(index) =
        listeners.iter().position(|listener| listener.id == id)
      {
        listeners.remove(index);
        return Some(id);
      }
    }

    return None;
  }

  pub async fn emit<T>(&mut self, t: String, value: T)
  where
    T: Serialize, {
    if let Some(listeners) = self.listeners.get_mut(&t) {
      let bytes: Vec<u8> = serialize(&value).unwrap();

      for listener in listeners.iter_mut() {
        (listener.callback)(bytes.clone())
      }
    }
  }
}
