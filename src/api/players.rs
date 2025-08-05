use crate::{client::StatbookClient, error::Result, models::PlayerSummary, utils::to_dash_case};

pub async fn get_player_by_name(
    client: &StatbookClient,
    name: &str,
) -> Result<Option<PlayerSummary>> {
    let dash_name = to_dash_case(name);
    let player_data = client.fetch_raw_player_data(&dash_name).await?;

    let player = match player_data.players.first() {
        Some(player) => player,
        None => return Ok(None),
    };

    let player_info = &player.player_info;

    let summary = PlayerSummary {
        first_name: player_info.first_name.clone().unwrap_or_default(),
        last_name: player_info.last_name.clone().unwrap_or_default(),
        primary_position: player_info.primary_position.clone().unwrap_or_default(),
        jersey_number: player_info.jersey_number.unwrap_or(0),
        current_team: player_info
            .current_team
            .as_ref()
            .and_then(|team| team.abbreviation.clone())
            .unwrap_or_default(),
        injury: player_info.injury.clone().unwrap_or_default(),
        rookie: player_info.rookie.unwrap_or(false),
        games_played: player.statistics.games_played.unwrap_or(0),
    };

    Ok(Some(summary))
}

