use breadferris::cmdlog;
use rand::prelude::SliceRandom;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use bbangcat_encryption::bce::*;

#[command]
#[aliases("랜덤", "골라", "random")]
async fn choice(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let r = args.rest().split(", ").collect::<Vec<_>>();
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    msg.reply(
        ctx,
        format!(
            "`{}`",
            r.choose(&mut rand::thread_rng()).unwrap_or_else(|| &"")
        ),
    )
    .await?;
    Ok(())
}

#[command]
#[aliases("니트로")]
async fn nitro(ctx: &Context, msg: &Message) -> CommandResult {
    let mut v = String::new();
    (0..16).enumerate().for_each(|_| {
        v.push(
            *({
                let mut _v = ('a'..='z').collect::<Vec<_>>();
                _v.append(&mut ('A'..='Z').collect::<Vec<_>>());
                _v
            }
            .choose(&mut rand::thread_rng()))
            .unwrap_or_else(|| &' '),
        );
    });
    msg.reply(ctx, format!("`discord.gift/{}`", v)).await?;

    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[aliases("빵암호화", "빵켓암호화")]
async fn bce(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    msg.reply(ctx, to_bce::new(args.rest())).await?;
    Ok(())
}