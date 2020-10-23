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
    C: Fn(T) + Sync + Send + 'static, {
    let id = self.count + 1;
    self.count = self.count + 1;

    let deserialized_cb = move |bytes: Vec<u8>| {
      let value: T = deserialize(&bytes).unwrap();
      callback(value);
    };

    let listener = Listener {
      id: id.clone(),
      callback: Box::new(deserialized_cb),
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

  pub fn off(&mut self, id: i64) -> Option<i64> {
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
