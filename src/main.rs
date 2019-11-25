use std::sync::Mutex;

use http::status::StatusCode;
use serde::{Deserialize, Serialize};
use tide::{App, Context, EndpointResult, error::ResultExt, middleware::DefaultHeaders, response};

#[derive(Default)]
struct Database {
  contents: Mutex<Vec<Message>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
  author: Option<String>,
  contents: String,
}

impl Database {
  fn insert(&self, msg: Message) -> usize {
    let mut table = self.contents.lock().unwrap();
    table.push(msg);
    table.len() - 1
  }

  fn get(&self, id: usize) -> Option<Message> {
    self.contents.lock().unwrap().get(id).cloned()
  }
}

async fn new_message(mut cx: Context<Database>) -> EndpointResult<String> {
  let msg = cx.body_json().await.client_err()?;
  Ok(cx.state().insert(msg).to_string())
}

async fn get_message(cx: Context<Database>) -> EndpointResult {
  let id = cx.param("id").client_err()?;
  if let Some(msg) = cx.state().get(id) {
    Ok(response::json(msg))
  } else {
    Err(StatusCode::NOT_FOUND)?
  }
}

fn main() {
  let mut app = App::with_state(Database::default());

  app.middleware(
    DefaultHeaders::new()
      .header("X-Version", "1.0.0")
      .header("X-Server", "Tide"),
  );

  app.at("/").get(|_| async move { "ok" });
  app.at("/message").post(new_message);
  app.at("/message/:id").get(get_message);
  app.serve("127.0.0.1:8000").unwrap();
}
