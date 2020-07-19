use serde::{Deserialize};
// use serde_json::{Result};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[derive(Deserialize, Debug)]
struct AdviceSlip {
    slip_id: u32,
    advice: String,
}

#[derive(Deserialize, Debug)]
struct AdviceSearch {
    total_results: u32,
    query: u32,
    slips: Vec<AdviceSlip>,
}

#[derive(Deserialize, Debug)]
struct AdviceMessage {
    r#type: String,
    text: String,
}

#[command]
fn advice(ctx: &mut Context, msg: &Message) -> CommandResult {
    let endpoint = "https://api.adviceslip.com/advice";
    let response = reqwest::blocking::get(endpoint)?;
    let slip: AdviceSlip = response.json()?;
    let results = format!("{:?}", slip);

    let _ = msg.channel_id.say(&ctx.http, results);
    Ok(())
}

#[command]
fn advice_id(_ctx: &mut Context, _msg: &Message, _args: Args) -> CommandResult {
    let _endpoint = "https://api.adviceslip.com/advice/{slip_id}";
    Ok(())
}

#[command]
fn advice_search(_ctx: &mut Context, _msg: &Message, _args: Args) -> CommandResult {
    let _endpoint = "https://api.adviceslip.com/advice/search/{query}";
    Ok(())
}