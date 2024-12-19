use kalosm::language::*;

#[tokio::main]
async fn main() {
    let model = Llama::new_chat().await.unwrap();

    let mut chat = Chat::builder(model)
      .with_system_prompt("Assistant will act like an tour guide")
      .build();

    loop {
        chat.add_message(prompt_input("\n >").unwrap())
          .to_std_out()
          .await
          .unwrap();
    }
}
