use serde::{Deserialize, Serialize};

/**
 * Defines a Team as described
 * [here](https://man.sr.ht/~muirrum/fll-scoring/tournament_management/).
 */
#[derive(Serialize, Deserialize)]
pub struct Team {
    /// The FIRST-assigned number of the team.
    pub number: u32,
    /// The team-chosen name. If not set, should be an empty String ("")
    pub name: String,
    /// The team's affiliation, if provided. Often, this is the organization sponsoring the team,
    /// like a school.
    pub affiliation: String,
}
/**
    Defines a new team as received from a Form or API
*/
#[derive(Deserialize, Serialize)]
pub struct NewTeam {
    /// The number of the team. A string, so Forms can send it.
    pub number: String,
    /// The team-chosen name.
    pub name: String,
    /// The team's affiliation, if provided
    pub affiliation: String,
}
/**
 * Defines a new tournament as defined
 * [here](https://man.sr.ht/~muirrum/fll-scoring/tournament_management/). The notable difference is
 * that the ID is set by the software, and returned as part of the API response.
 */
#[derive(Deserialize)]
pub struct NewTournament {
    /// The name of the tournament, set by organizers.
    pub name: String,
    /// A vector of Teams, for new tournaments this can be empty.
    pub teams: Vec<u32>,
    /// The current stage of the tournament, most often this will be either "quals" or "playoffs"
    pub current_stage: String,
}

/**
 * Represents a Tournament as defined
 * [here](https://man.sr.ht/~muirrum/fll-scoring/tournament_management/api.md#tournament).
 */
#[derive(Serialize)]
pub struct Tournament {
    /// The application-assigned ID for the tournament.
    pub id: u32,
    /// The user-defined name for the tournament.
    pub name: String,
    /// A vector of numbers referencing Teams that are participating in this event.
    pub teams: Vec<u32>,
    /// The current stage of the event, either "quals" or "playoffs"
    pub current_stage: String,
}
