use skyvein_rust_api::{hander::Art, model::art::*, Client};
use std::sync::Arc;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // 创建连接
  let client: Arc<Client> = Arc::new(Client::new(
    "ws://127.0.0.1:9944".to_string(),
    "near rotate couch giant neck diary token roast come embark doll chaos".to_string(),
  ));
  // 构建参数
  let art = Art::new(Arc::clone(&client));
  let call_args: RegisterArtData = serde_json::from_str(
    "{\"id\":\"111\",\"props\":[{\"name\":\"xxx33333333333333333\",\"value\":\"222\"}],\"owner\":\"xxxxxx\",\"entity_ids\":[\"111\",\"2222\"],\"hash_method\":\"blake2\",\"hash\":\"xxxxxxxx\",\"dna_method\":\"\",\"dna\":\"\"}",
  )?;
  // 上链
  let res = art.register_art(call_args.clone()).await;
  match res {
    Ok(s) => println!("{:?}", s.as_str()),
    Err(e) => println!("{:?}", e),
  }
  Ok(())
}
