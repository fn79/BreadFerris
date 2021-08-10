use breadferris::cmdlog;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::model::misc::Mentionable;
use serenity::prelude::Context;

#[command]
#[required_permissions(ban_members)]
#[aliases("밴", "차단")]
async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = ctx
        .http
        .get_user(
            args.single::<String>()?
                .replace("<", "")
                .replace(">", "")
                .replace("@", "")
                .replace("!", "")
                .parse::<u64>()?,
        )
        .await?;
    let r = args.single::<String>()?;
    if let Some(e) = msg.guild_id {
        e.ban_with_reason(ctx, user.clone(), 0, r.clone()).await?;
        msg.reply(
            ctx,
            format!(
                "{}을(를) 차단했습니다.\n\n사유:\n```\n{}\n```",
                user.clone().mention(),
                r.clone()
            ),
        )
        .await?;
    }
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[required_permissions(kick_members)]
#[aliases("킥", "추방")]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user = ctx
        .http
        .get_user(
            args.single::<String>()?
                .replace("<", "")
                .replace(">", "")
                .replace("@", "")
                .replace("!", "")
                .parse::<u64>()?,
        )
        .await?;
    let r = args.single::<String>()?;
    if let Some(e) = msg.guild_id {
        e.kick_with_reason(ctx, user.clone(), r.clone().as_str())
            .await?;
        msg.reply(
            ctx,
            format!(
                "{}을(를) 추방했습니다.\n\n사유:\n```\n{}\n```",
                user.clone().mention(),
                r.clone()
            ),
        )
        .await?;
    }
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}

#[command]
#[required_permissions(ban_members)]
#[aliases("언밴")]
async fn unban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let user_id = args
        .single::<String>()?
        .replace("<", "")
        .replace(">", "")
        .replace("@", "")
        .replace("!", "")
        .parse::<u64>()?;
    let user = ctx.http.get_user(user_id).await?;
    if let Some(e) = msg.guild_id {
        e.unban(ctx, user_id).await?;
        msg.reply(
            ctx,
            format!(
                "{}을(를) 차단해제 했습니다.",
                user.clone().mention(),
            ),
        )
        .await?;
    }
    cmdlog(msg.author.id.to_string(), msg.content.clone());
    Ok(())
}