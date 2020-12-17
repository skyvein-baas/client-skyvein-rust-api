use subxt::{
  system::System,
  EventsDecoder,
  Metadata,
  DefaultNodeRuntime,
};

/// 医疗影像存证
#[derive(Debug, Clone)]
pub struct Client {
  // 连接
  uri: String,
  // 用户种子
  pub seed: String,
  // 用户账户
  address: String,
  // 用户密码
  key: String,
}

impl Client {
  pub fn new(uri: String, seed: String) -> Self {
    Client {
      uri: uri,
      seed: seed,
      address: "".to_string(),
      key: "".to_string(),
    }
  }

  pub fn seed_get(&self) -> String {
    return self.seed.clone()
  }

  pub fn create_decoder(metadata: Metadata) -> EventsDecoder<DefaultNodeRuntime> {
    let mut decoder = EventsDecoder::<DefaultNodeRuntime>::new(metadata);
    decoder.register_type_size::<[u8; 16]>("ReportId");
    decoder
  }
}

