# Tournament Management
This is the application that manages all of the tournaments and teams present in the system. Teams are identified by their FIRST-assigned number, with an optional display name/affiliation.

## API
The API needs to provide Team information and be able to accept team information. A team is defined as follows:

```json
Team {
    number: BigInt,
    name: Option<String>,
    affiliation: Option<String>,
}
```

Teams are organized into Tournaments, which are defined as follows:
```json
Tournament {
    id: Int,
    name: String,
    teams: Team[],
    current_stage: String,
}
```
