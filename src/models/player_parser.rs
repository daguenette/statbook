use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct PlayerResponse {
    #[serde(rename = "playerStatsTotals", default)]
    pub players: Vec<Player>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct Player {
    #[serde(rename = "player", default)]
    pub player_info: PlayerInfo,
    #[serde(rename = "team", default)]
    #[allow(dead_code)]
    pub team_info: TeamInfo,
    #[serde(rename = "stats", default)]
    pub statistics: Statistics,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct PlayerInfo {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(rename = "firstName", default)]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", default)]
    pub last_name: Option<String>,
    #[serde(rename = "primaryPosition", default)]
    pub primary_position: Option<String>,
    #[serde(rename = "jerseyNumber", default)]
    pub jersey_number: Option<u32>,
    #[serde(rename = "currentTeam", default)]
    pub current_team: Option<TeamInfo>,
    #[serde(rename = "currentRosterStatus", default)]
    pub roster_status: Option<String>,
    #[serde(rename = "currentInjury", default)]
    pub injury: Option<String>,
    #[serde(default)]
    pub height: Option<String>,
    #[serde(default)]
    pub weight: Option<u32>,
    #[serde(rename = "birthDate", default)]
    pub birth_date: Option<String>,
    #[serde(default)]
    pub age: Option<u32>,
    #[serde(rename = "birthCity", default)]
    pub birth_city: Option<String>,
    #[serde(rename = "birthCountry", default)]
    pub birth_country: Option<String>,
    #[serde(default)]
    pub rookie: Option<bool>,
    #[serde(rename = "highSchool", default)]
    pub high_school: Option<String>,
    #[serde(default)]
    pub college: Option<String>,
    #[serde(skip)]
    pub handedness: Option<String>,
    #[serde(rename = "officialImageSrc", default)]
    pub official_image_src: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct TeamInfo {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub abbreviation: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct Statistics {
    #[serde(rename = "gamesPlayed", default)]
    pub games_played: Option<u64>,
    #[serde(rename = "passing", default)]
    pub passing_stats: Option<PassingStats>,
    #[serde(rename = "rushing", default)]
    pub rushing_stats: Option<RushingStats>,
    #[serde(rename = "receiving", default)]
    pub receiving_stats: Option<ReceivingStats>,
    #[serde(rename = "tackles", default)]
    pub tackles_stats: Option<TackleStats>,
    #[serde(rename = "interceptions", default)]
    pub interceptions_stats: Option<InterceptionStats>,
    #[serde(rename = "fumbles", default)]
    pub fumbles_stats: Option<FumbleStats>,
    #[serde(rename = "kickoffReturns", default)]
    pub kick_off_returns: Option<KickOffReturns>,
    #[serde(rename = "puntReturns", default)]
    pub punt_returns: Option<PuntReturns>,
    #[serde(rename = "miscellaneous", default)]
    pub miscellaneous_stats: Option<MiscellaneousStats>,
    #[serde(rename = "twoPointAttempts", default)]
    pub two_point_attempts: Option<TwoPointAttempts>,
    #[serde(rename = "snapCounts", default)]
    pub snap_counts: Option<SnapCounts>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct PassingStats {
    #[serde(rename = "passAttempts", default)]
    pub attempts: Option<i64>,
    #[serde(rename = "passCompletions", default)]
    pub completions: Option<i64>,
    #[serde(rename = "passPct", default)]
    pub percentage: Option<f64>,
    #[serde(rename = "passYards", default)]
    pub yards_total: Option<i64>,
    #[serde(rename = "passAvg", default)]
    pub yards_average: Option<f64>,
    #[serde(rename = "passYardsPerAtt", default)]
    pub yards_per_attempt: Option<f64>,
    #[serde(rename = "passTD", default)]
    pub touchdowns: Option<i64>,
    #[serde(rename = "passTDPct", default)]
    pub touchdowns_percentage: Option<f64>,
    #[serde(rename = "passInt", default)]
    pub interceptions: Option<i64>,
    #[serde(rename = "passIntPct", default)]
    pub interception_percentage: Option<f64>,
    #[serde(rename = "passLng", default)]
    pub longest_pass: Option<i64>,
    #[serde(rename = "pass20Plus", default)]
    pub twenty_plus_yards: Option<i64>,
    #[serde(rename = "pass40Plus", default)]
    pub forty_plus_yards: Option<i64>,
    #[serde(rename = "passSacks", default)]
    pub times_sacked: Option<i64>,
    #[serde(rename = "passSackY", default)]
    pub sack_yards_lost: Option<i64>,
    #[serde(rename = "qbRating", default)]
    pub quarterback_rating: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct RushingStats {
    #[serde(rename = "rushAttempts", default)]
    pub attempts: Option<i64>,
    #[serde(rename = "rushYards", default)]
    pub yards_total: Option<i64>,
    #[serde(rename = "rushAverage", default)]
    pub yards_average: Option<f64>,
    #[serde(rename = "rushTD", default)]
    pub touchdowns: Option<i64>,
    #[serde(rename = "rushLng", default)]
    pub longest_rush_yards: Option<i64>,
    #[serde(rename = "rush1stDowns", default)]
    pub first_downs: Option<i64>,
    #[serde(rename = "rush1stDownsPct", default)]
    pub first_downs_percentage: Option<f64>,
    #[serde(rename = "rush20Plus", default)]
    pub twenty_plus_yards: Option<i64>,
    #[serde(rename = "rush40Plus", default)]
    pub forty_plus_yards: Option<i64>,
    #[serde(rename = "rushFumbles", default)]
    pub fumbles: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct ReceivingStats {
    #[serde(rename = "targets", default)]
    pub targets: Option<i64>,
    #[serde(rename = "receptions", default)]
    pub receptions: Option<i64>,
    #[serde(rename = "recYards", default)]
    pub yards_total: Option<i64>,
    #[serde(rename = "recAverage", default)]
    pub yards_average: Option<f64>,
    #[serde(rename = "recTD", default)]
    pub touchdowns: Option<i64>,
    #[serde(rename = "recLng", default)]
    pub longest_reception_yards: Option<i64>,
    #[serde(rename = "rec1stDowns", default)]
    pub first_downs: Option<i64>,
    #[serde(rename = "rec20Plus", default)]
    pub twenty_plus_yards: Option<i64>,
    #[serde(rename = "rec40Plus", default)]
    pub forty_plus_yards: Option<i64>,
    #[serde(rename = "recFumbles", default)]
    pub fumbles: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct TackleStats {
    #[serde(rename = "tackleSolo", default)]
    pub tackles: Option<i64>,
    #[serde(rename = "tackleTotal", default)]
    pub tackles_total: Option<i64>,
    #[serde(rename = "tackleAst", default)]
    pub assisted_tackles: Option<i64>,
    #[serde(rename = "sacks", default)]
    pub sacks: Option<f64>,
    #[serde(rename = "sackYds", default)]
    pub sack_yards_lost: Option<i64>,
    #[serde(rename = "tacklesForLoss", default)]
    pub tackles_for_loss: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct InterceptionStats {
    #[serde(rename = "interceptions", default)]
    pub interceptions: Option<i64>,
    #[serde(rename = "intTD", default)]
    pub touchdowns: Option<i64>,
    #[serde(rename = "intYds", default)]
    pub yards_todal: Option<i64>,
    #[serde(rename = "intAverage", default)]
    pub yards_average: Option<f64>,
    #[serde(rename = "intLng", default)]
    pub longest_interception: Option<i64>,
    #[serde(rename = "passesDefended", default)]
    pub passes_defended: Option<i64>,
    #[serde(rename = "stuffs", default)]
    pub stuffs_at_line: Option<i64>,
    #[serde(rename = "stuffYds", default)]
    pub stuff_yards_prevented: Option<i64>,
    #[serde(rename = "safeties", default)]
    pub safeties_scored: Option<i64>,
    #[serde(rename = "kB", default)]
    pub knockdowns: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct FumbleStats {
    #[serde(rename = "fumbles", default)]
    pub fumbles: Option<i64>,
    #[serde(rename = "fumLost", default)]
    pub lost: Option<i64>,
    #[serde(rename = "fumForced", default)]
    pub forced: Option<i64>,
    #[serde(rename = "fumOwnRec", default)]
    pub own_recovered: Option<i64>,
    #[serde(rename = "fumOppRec", default)]
    pub opponent_recovered: Option<i64>,
    #[serde(rename = "fumRecYds", default)]
    pub recovery_yards_total: Option<i64>,
    #[serde(rename = "fumTotalRec", default)]
    pub recovered_total: Option<i64>,
    #[serde(rename = "fumTD", default)]
    pub recovery_touchdowns: Option<i64>,
    #[serde(rename = "offFumTD", default)]
    pub opponent_fumble_touchdowns: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct KickOffReturns {
    #[serde(rename = "krRet", default)]
    pub returns: Option<i64>,
    #[serde(rename = "krYds", default)]
    pub yards_total: Option<i64>,
    #[serde(rename = "krAvg", default)]
    pub yards_average: Option<f64>,
    #[serde(rename = "krLng", default)]
    pub longest_kickoff_return: Option<i64>,
    #[serde(rename = "krTD", default)]
    pub touchdowns: Option<i64>,
    #[serde(rename = "kr20Plus", default)]
    pub twenty_plus_yards: Option<i64>,
    #[serde(rename = "kr40Plus", default)]
    pub forty_plus_yards: Option<i64>,
    #[serde(rename = "krFC", default)]
    pub catches: Option<i64>,
    #[serde(rename = "krFum", default)]
    pub fumbles: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct PuntReturns {
    #[serde(rename = "prRet", default)]
    pub punt_returns: Option<i64>,
    #[serde(rename = "prYds", default)]
    pub yards_total: Option<i64>,
    #[serde(rename = "prAvg", default)]
    pub yards_average: Option<f64>,
    #[serde(rename = "prLng", default)]
    pub longest_punt_return: Option<i64>,
    #[serde(rename = "prTD", default)]
    pub touchdowns: Option<i64>,
    #[serde(rename = "pr20Plus", default)]
    pub twenty_plus_yards: Option<i64>,
    #[serde(rename = "pr40Plus", default)]
    pub forty_plus_yards: Option<i64>,
    #[serde(rename = "prFC", default)]
    pub catches: Option<i64>,
    #[serde(rename = "prFum", default)]
    pub fumbles: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct MiscellaneousStats {
    #[serde(rename = "gamesStarted", default)]
    pub games_started: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct SnapCounts {
    #[serde(rename = "offenseSnaps", default)]
    pub offense_snaps: Option<i64>,
    #[serde(rename = "defenseSnaps", default)]
    pub defense_snaps: Option<i64>,
    #[serde(rename = "specialTeamSnaps", default)]
    pub special_team_snaps: Option<i64>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub(crate) struct TwoPointAttempts {
    #[serde(rename = "twoPtAtt", default)]
    pub two_point_attempts_total: Option<i64>,
    #[serde(rename = "twoPtMade", default)]
    pub two_point_conversions_made: Option<i64>,
    #[serde(rename = "twoPtPassAtt", default)]
    pub two_point_pass_attempts: Option<i64>,
    #[serde(rename = "twoPtPassMade", default)]
    pub two_point_pass_conversions: Option<i64>,
    #[serde(rename = "twoPtPassRec", default)]
    pub two_point_pass_receptions: Option<i64>,
    #[serde(rename = "twoPtRushAtt", default)]
    pub two_point_rush_attempts: Option<i64>,
    #[serde(rename = "twoPtRushMade", default)]
    pub two_point_rush_conversions: Option<i64>,
}
