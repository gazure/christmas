use anyhow::Result;
use std::collections::HashMap;

use chrono::{Datelike, Local};

mod giftexchange;
// mod persist;
mod ui;

use giftexchange::ExchangePool;

fn letter_for_pool(pool: ExchangePool) -> char {
    let letters = "ACDIJLMNORSTUXYZ".chars().collect::<Vec<char>>();

    match pool {
        ExchangePool::IslandLife => 'I',
        ExchangePool::Grabergishimazureson => fastrand::choice(letters.iter()).unwrap().clone(),
        ExchangePool::Pets => 'P',
    }
}

#[derive(Debug, Default, Clone)]
struct Participant {
    name: String,
    exchange_pools: Vec<ExchangePool>,
    exclusions: Vec<String>,
}

#[derive(Debug, Default)]
struct ParticipantGraph {
    edges: HashMap<String, Vec<String>>,
    participants: HashMap<String, Participant>,
}

impl ParticipantGraph {
    pub fn new() -> Self {
        Self {
            participants: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn from_participants(participants: Vec<Participant>) -> Self {
        let mut graph = Self::new();
        participants.iter().for_each(|p| {
            graph.add_participant(p.clone());
        });
        graph.link_participants();
        graph
    }

    pub fn add_participant(&mut self, participant: Participant) {
        self.participants
            .insert(participant.name.clone(), participant);
    }

    pub fn link_participants(&mut self) {
        for (name, participant) in &self.participants {
            let mut possible_receivers = self
                .participants
                .iter()
                .filter(|(n, p)| {
                    *n != name
                        && !participant.exclusions.contains(n)
                        && participant
                            .exchange_pools
                            .iter()
                            .any(|pool| p.exchange_pools.contains(pool))
                })
                .map(|(n, _)| n.clone())
                .collect::<Vec<String>>();
            fastrand::shuffle(&mut possible_receivers);
            self.edges.insert(name.clone(), possible_receivers);
        }
    }

    pub fn build_exchange(&self) -> Vec<(String, String)> {
        let mut exchange = vec![];
        let num_participants = self.participants.len();
        let first = self.participants.iter().next().unwrap().0.clone();
        let mut current = vec![first.clone()];
        let mut visited = vec![first.clone()];

        while current.len() < num_participants {
            let receivers = self.edges.get(current.last().unwrap()).unwrap();
            let receiver = receivers.iter().find(|r| !visited.contains(r));
            if let Some(receiver) = receiver {
                current.push(receiver.clone());
                visited.push(receiver.clone());
            } else {
                let c = current.pop().unwrap();
                visited.retain(|v| *v != c);
            }

            if current.is_empty() {
                eprintln!("No way to construct ordering for current restrictions");
                break;
            }
        }

        current.as_slice().windows(2).for_each(|pair| {
            exchange.push((pair[0].clone(), pair[1].clone()));
        });
        let first = exchange.first().unwrap().0.clone();
        let last = exchange.last().unwrap().1.clone();
        exchange.push((last, first));

        exchange
    }
}

impl Participant {
    pub fn new(name: String, exchange_pools: Vec<ExchangePool>, exclusions: Vec<&str>) -> Participant {
        let exclusions = exclusions.iter().map(|s| s.to_string()).collect();
        Participant {
            name,
            exchange_pools,
            exclusions,
        }
    }
}

fn build_exchange() -> Result<()> {
    let participants = vec![
        Participant::new(
            "Claire".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergishimazureson],
            vec!["Duncan", "Chris"],
        ),
        Participant::new(
            "Grant".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergishimazureson],
            vec!["Noel"],
        ),
        Participant::new(
            "Anne".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergishimazureson],
            vec!["Eric", "Kari"],
        ),
        Participant::new(
            "Duncan".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergishimazureson],
            vec!["Claire", "Chris"],
        ),
        Participant::new(
            "Noel".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergishimazureson],
            vec!["K-Lee", "Claire"],
        ),
        Participant::new(
            "K-Lee".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergishimazureson],
            vec!["Noel", "Jim"],
        ),
        Participant::new(
            "Steve".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergishimazureson],
            vec!["Linda", "Duncan"],
        ),
        Participant::new(
            "Linda".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergishimazureson],
            vec!["Steve", "Alec"],
        ),
        Participant::new(
            "Chris".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergishimazureson],
            vec!["Eric"],
        ),
        Participant::new(
            "Jim".to_string(),
            vec![ExchangePool::Grabergishimazureson],
            vec!["Kari", "Anne"],
        ),
        Participant::new(
            "Kari".to_string(),
            vec![ExchangePool::Grabergishimazureson],
            vec!["Jim", "Linda"],
        ),
        Participant::new(
            "Meaghann".to_string(),
            vec![ExchangePool::Grabergishimazureson],
            vec!["Steve"],
        ),
        Participant::new(
            "Alec".to_string(),
            vec![ExchangePool::Grabergishimazureson],
            vec!["Meaghann"],
        ),
        Participant::new(
            "Eric".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergishimazureson],
            vec!["Anne", "K-Lee"],
        ),
        Participant::new("Stella".to_string(), vec![ExchangePool::Pets], vec!["Daisy"]),
        Participant::new(
            "Bailey".to_string(),
            vec![ExchangePool::Pets],
            vec!["Luca"],
        ),
        Participant::new("Kitty".to_string(), vec![ExchangePool::Pets], vec!["Bailey"]),
        Participant::new(
            "Charlie".to_string(),
            vec![ExchangePool::Pets],
            vec!["Kona"],
        ),
        Participant::new(
            "Astra".to_string(),
            vec![ExchangePool::Pets],
            vec!["Lily"],
        ),
        Participant::new("Freya".to_string(), vec![ExchangePool::Pets], vec!["Stella"]),
        Participant::new("Lily".to_string(), vec![ExchangePool::Pets], vec!["Kitty"]),
        Participant::new(
            "Daisy".to_string(),
            vec![ExchangePool::Pets],
            vec!["Astra"],
        ),
        Participant::new("Luca".to_string(), vec![ExchangePool::Pets], vec!["Charlie"]),
        Participant::new("Kona".to_string(), vec![ExchangePool::Pets], vec!["Freya"]),
    ];
    // get cli args

    let pool_arg = std::env::args().nth(1).expect("No pool specified");
    let pool = match pool_arg.as_str() {
        "island" => ExchangePool::IslandLife,
        "graber" => ExchangePool::Grabergishimazureson,
        "pets" => ExchangePool::Pets,
        _ => panic!("Invalid pool specified"),
    };

    let filtered_participants = participants
        .iter()
        .filter(|p| p.exchange_pools.contains(&pool))
        .cloned()
        .collect::<Vec<Participant>>();

    let graph = ParticipantGraph::from_participants(filtered_participants);
    let exchange = graph.build_exchange();
    exchange.iter().for_each(|(sender, receiver)| {
        println!("{} -> {}", sender, receiver);
    });
    let year = Local::now().year();
    println!("Letter for {}: {}", year, letter_for_pool(pool));
    // let mut conn = persist::init_db("./drawings.db".into())?;
    // let exchange_ids = persist::add_exchange(
    //     &mut conn,
    //     &[
    //         ExchangePool::Grabergishimazureson,
    //         ExchangePool::IslandLife,
    //         ExchangePool::Pets,
    //     ],
    // )?;

    // let current_exchange_id = match pool {
    //     ExchangePool::Grabergishimazureson => exchange_ids[0],
    //     ExchangePool::IslandLife => exchange_ids[1],
    //     ExchangePool::Pets => exchange_ids[2],
    // };

    // let mut participant_name_to_id = HashMap::new();

    // for p in participants {
    //     let id = persist::add_participant(&mut conn, &p)?;
    //     participant_name_to_id.insert(p.name.clone(), id);
    // }

    // persist::reset_pairs_for_exchange(&mut conn, current_exchange_id)?;

    // for (sender, receiver) in exchange {
    //     let sender_id = participant_name_to_id
    //         .get(&sender)
    //         .expect("Sender not found");
    //     let receiver_id = participant_name_to_id
    //         .get(&receiver)
    //         .expect("Receiver not found");
    //     persist::add_exchange_pair(&mut conn, *sender_id, *receiver_id, current_exchange_id)?;
    // }
    Ok(())
}


fn main() -> Result<()> {
    // Check if CLI args are provided
    if std::env::args().len() > 1 {
        // Run CLI version
        build_exchange()?;
    } else {
        // Run Dioxus web app
        dioxus::launch(ui::app);
    }
    Ok(())
}
