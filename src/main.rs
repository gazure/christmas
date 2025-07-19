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

    /// Builds a gift exchange by finding a Hamiltonian cycle in the participant graph.
    ///
    /// This algorithm attempts to create a cycle where:
    /// - Each person gives exactly one gift
    /// - Each person receives exactly one gift
    /// - All exclusion rules are respected
    ///
    /// The algorithm tries up to 100 times with different random starting points
    /// to find a valid Hamiltonian cycle. If no cycle is found, it falls back
    /// to a simpler pairing strategy.
    pub fn build_exchange(&self) -> Vec<(String, String)> {
        let num_participants = self.participants.len();
        if num_participants == 0 {
            return vec![];
        }

        // Try multiple times with different random starting points
        for _attempt in 0..100 {
            // Get a random starting participant
            let mut participants_list: Vec<String> = self.participants.keys().cloned().collect();
            fastrand::shuffle(&mut participants_list);

            if let Some(solution) = self.find_hamiltonian_cycle(&participants_list[0], num_participants) {
                // Convert the cycle to exchange pairs
                let mut exchange = vec![];
                for i in 0..solution.len() - 1 {
                    exchange.push((solution[i].clone(), solution[i + 1].clone()));
                }
                // Add the last edge to complete the cycle
                exchange.push((solution[solution.len() - 1].clone(), solution[0].clone()));
                return exchange;
            }
        }

        eprintln!("Warning: Could not find a perfect cycle after 100 attempts. Falling back to best-effort pairing.");
        self.fallback_exchange()
    }

    /// Attempts to find a Hamiltonian cycle starting from the given node.
    ///
    /// A Hamiltonian cycle visits each node exactly once and returns to the start.
    /// This ensures everyone gives and receives exactly one gift.
    fn find_hamiltonian_cycle(&self, start: &str, target_length: usize) -> Option<Vec<String>> {
        let mut path = vec![start.to_string()];
        let mut visited = std::collections::HashSet::new();
        visited.insert(start.to_string());

        if self.dfs_hamiltonian(&mut path, &mut visited, target_length, start) {
            Some(path)
        } else {
            None
        }
    }

    /// Depth-first search with backtracking to find a Hamiltonian cycle.
    ///
    /// This recursively explores paths, backtracking when it hits a dead end.
    /// The randomization of edge order helps find different valid cycles
    /// across multiple runs.
    fn dfs_hamiltonian(
        &self,
        path: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
        target_length: usize,
        start: &str,
    ) -> bool {
        if path.len() == target_length {
            // Check if we can return to the start
            let last = path.last().unwrap();
            if let Some(edges) = self.edges.get(last) {
                return edges.contains(&start.to_string());
            }
            return false;
        }

        let current = path.last().unwrap().clone();
        if let Some(edges) = self.edges.get(&current) {
            // Try edges in random order
            let mut shuffled_edges = edges.clone();
            fastrand::shuffle(&mut shuffled_edges);

            for next in shuffled_edges {
                if !visited.contains(&next) {
                    path.push(next.clone());
                    visited.insert(next.clone());

                    if self.dfs_hamiltonian(path, visited, target_length, start) {
                        return true;
                    }

                    // Backtrack
                    path.pop();
                    visited.remove(&next);
                }
            }
        }

        false
    }

    /// Fallback strategy when a Hamiltonian cycle cannot be found.
    ///
    /// This creates a simple valid exchange by trying to match givers to receivers
    /// while respecting exclusion rules. If that fails, it falls back to a
    /// simple rotation where each person gives to the next in the list.
    fn fallback_exchange(&self) -> Vec<(String, String)> {
        // Create a simple valid exchange by ensuring everyone gives and receives once
        let mut givers: Vec<String> = self.participants.keys().cloned().collect();
        let mut receivers: Vec<String> = givers.clone();
        let mut exchange = vec![];

        fastrand::shuffle(&mut givers);
        fastrand::shuffle(&mut receivers);

        for giver in &givers {
            // Find a valid receiver
            for (idx, receiver) in receivers.iter().enumerate() {
                if giver != receiver && self.can_give_to(giver, receiver) {
                    exchange.push((giver.clone(), receiver.clone()));
                    receivers.remove(idx);
                    break;
                }
            }
        }

        // If we couldn't match everyone, just do a simple rotation
        if exchange.len() < givers.len() {
            exchange.clear();
            for i in 0..givers.len() {
                let next = (i + 1) % givers.len();
                exchange.push((givers[i].clone(), givers[next].clone()));
            }
        }

        exchange
    }

    /// Checks if a giver can give to a receiver based on the exclusion rules.
    fn can_give_to(&self, giver: &str, receiver: &str) -> bool {
        if let Some(edges) = self.edges.get(giver) {
            edges.contains(&receiver.to_string())
        } else {
            false
        }
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
