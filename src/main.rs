use std::{collections::HashMap, io::Stdout};
use anyhow::Context;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body

}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,

    #[serde(flatten)]
    payload: Payload
    //rest: HashMap<String, serde_json::Value>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo{
         echo: String
    }
}




fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let Stdout = std::io::stdout().lock();

    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();
    for input in inputs{
        let input = input.context("Maelstrom input from stdin could not be deserialized")?;

        

    }

    Ok(())
    //println!("Hello, world!");
}
