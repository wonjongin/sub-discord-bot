extern crate dotenv;

use std::env;

use dotenv::dotenv;

use serenity::{
  async_trait,
  model::{channel::Message, gateway::Ready, user::User},
  prelude::*,
};

mod mention;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  // Set a handler for the `message` event - so that whenever a new message
  // is received - the closure (or function) passed will be called.
  //
  // Event handlers are dispatched through a threadpool, and so multiple
  // events can be dispatched simultaneously.
  async fn message(&self, ctx: Context, msg: Message) {
    // println!("message: {}", msg.content);
    if msg.content == "//ping" {
      // Sending a message can fail, due to a network error, an
      // authentication error, or lack of permissions to post in the
      // channel, so log to stdout when some error happens, with a
      // description of it.
      if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
      }
    } else if msg.content == "//sub" {
      if let Err(why) = msg.channel_id.say(&ctx.http, "Good").await {
        println!("Error sending message: {:?}", why);
      }
    }
    if mention::has_mention(&msg.content[..]) {
      //   println!(
      //     "mention: {}",
      //     mention::ext_user_id_from_mention(&msg.content[..])
      //   );
      let receiver: &User = &msg.mentions[0];
      let sender_name = &msg.author.name;
      match receiver
        .direct_message(&ctx.http, |m| {
          m.content("").embed(|e| {
            e.title("🛎 멘션되었어요!").description(format!(
              "📩  {} 에게 멘션되었어요! 메시지를 확인하세요!!",
              sender_name
            ))
          })
        })
        .await
      {
        Ok(_) => {
          let _ = msg.react(&ctx.http, '👌');
        }
        Err(why) => {
          println!("Err sending help: {:?}", why);

          let _ = msg.reply(&ctx.http, "There was an error DMing you help.");
        }
      }
      if let Err(why) = msg
        .channel_id
        .say(&ctx.http, format!("멘션을 DM으로도 보냈어요! 📨"))
        .await
      {
        println!("Error sending message: {:?}", why);
      }
    }
  }

  // Set a handler to be called on the `ready` event. This is called when a
  // shard is booted, and a READY payload is sent by Discord. This payload
  // contains data like the current user's guild Ids, current user data,
  // private channels, and more.
  //
  // In this case, just print what the current user's username is.
  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

#[tokio::main]
async fn main() {
  // Configure the client with your Discord bot token in the environment.
  dotenv().ok();
  let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

  // Create a new instance of the Client, logging in as a bot. This will
  // automatically prepend your bot token with "Bot ", which is a requirement
  // by Discord for bot users.
  let mut client = Client::builder(&token)
    .event_handler(Handler)
    .await
    .expect("Err creating client");

  // Finally, start a single shard, and start listening to events.
  //
  // Shards will automatically attempt to reconnect, and will perform
  // exponential backoff until it reconnects.
  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}
