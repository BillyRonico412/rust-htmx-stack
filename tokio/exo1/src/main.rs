use std::{println, time::Duration};

use tokio::{join, time::sleep};

struct User {
    id: u32,
    name: String,
    age: u32,
    team_id: u32,
}

struct Team {
    id: u32,
    name: String,
}

async fn fetch_users() -> Vec<User> {
    let user_1 = User {
        name: String::from("Jane"),
        id: 1,
        age: 20,
        team_id: 1,
    };
    let user_2 = User {
        name: String::from("John"),
        id: 2,
        age: 30,
        team_id: 2,
    };
    let user_3 = User {
        name: String::from("Jack"),
        id: 3,
        age: 40,
        team_id: 1,
    };
    sleep(Duration::from_millis(2000)).await;
    return vec![user_1, user_2, user_3];
}

async fn fetch_teams() -> Vec<Team> {
    let team_1 = Team {
        name: String::from("Team A"),
        id: 1,
    };
    let team_2 = Team {
        name: String::from("Team B"),
        id: 2,
    };
    sleep(Duration::from_millis(3000)).await;
    return vec![team_1, team_2];
}

async fn logging() {
    println!("Logging...");
    sleep(Duration::from_millis(2000)).await;
    println!("Done logging");
}

#[tokio::main]
async fn main() {
    logging().await;
    let (users, teams) = join!(fetch_users(), fetch_teams());
    users.iter().for_each(|user| {
        let Some(team) = teams.iter().find(|&t| t.id == user.team_id) else {
            return;
        };
        println!(
            "{}. {} has {} years old and {} team's member.",
            user.id, user.name, user.age, team.name,
        )
    });
}
