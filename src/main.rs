use chrono::{Datelike, Local};
use rand;
use rand::seq::SliceRandom;

#[derive(Debug, Copy, Clone, PartialEq)]
enum ExchangePool {
    IslandLife,
    Grabergazureson,
    Pets,
}

fn letter_for_pool(pool: ExchangePool) -> char {
    let mut rng = rand::thread_rng();
    let mut letters = "ACDIJLMNORSTUXYZ".chars().collect::<Vec<char>>();
    letters.shuffle(&mut rng);

    match pool {
        ExchangePool::IslandLife => 'I',
        ExchangePool::Grabergazureson => letters[0],
        ExchangePool::Pets => 'P',
    }
}

#[derive(Debug, Default, Clone)]
struct Participant {
    name: String,
    exchange_pools: Vec<ExchangePool>,
    exclusions: Vec<String>,
}

impl Participant {
    fn new(name: String, exchange_pools: Vec<ExchangePool>, exclusions: Vec<&str>) -> Participant {
        let exclusions = exclusions.iter().map(|s| s.to_string()).collect();
        Participant {
            name,
            exchange_pools,
            exclusions,
        }
    }
}

fn contains_exclusions(participants: &Vec<&Participant>) -> bool {
    participants.iter().enumerate().any(|(i, p)| {
        let receiver = &participants[(i + 1) % participants.len()].name;
        p.exclusions.contains(&receiver)
    })
}

fn main() {
    let participants = vec![
        Participant::new(
            "Claire".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson],
            vec!["Duncan", "Meaghann"],
        ),
        Participant::new(
            "Grant".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson],
            vec!["Chris"],
        ),
        Participant::new(
            "Anne".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson],
            vec!["Eric", "K-Lee"],
        ),
        Participant::new(
            "Duncan".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson],
            vec!["Claire", "Steve"],
        ),
        Participant::new(
            "Noel".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson],
            vec!["K-Lee", "Linda"],
        ),
        Participant::new(
            "K-Lee".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson],
            vec!["Noel", "Kari"],
        ),
        Participant::new(
            "Steve".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson],
            vec!["Linda", "Alec"],
        ),
        Participant::new(
            "Linda".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson],
            vec!["Steve", "Anne"],
        ),
        Participant::new(
            "Chris".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson],
            vec!["Jim"],
        ),
        Participant::new(
            "Jim".to_string(),
            vec![ExchangePool::Grabergazureson],
            vec!["Kari", "Duncan"],
        ),
        Participant::new(
            "Kari".to_string(),
            vec![ExchangePool::Grabergazureson],
            vec!["Jim", "Grant"],
        ),
        Participant::new(
            "Meaghann".to_string(),
            vec![ExchangePool::Grabergazureson],
            vec!["Noel"],
        ),
        Participant::new(
            "Alec".to_string(),
            vec![ExchangePool::Grabergazureson],
            vec!["Claire"],
        ),
        Participant::new(
            "Eric".to_string(),
            vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson],
            vec!["Anne"],
        ),
        Participant::new("Stella".to_string(), vec![ExchangePool::Pets], vec!["Lily"]),
        Participant::new(
            "Bailey".to_string(),
            vec![ExchangePool::Pets],
            vec!["Kitty"],
        ),
        Participant::new("Kitty".to_string(), vec![ExchangePool::Pets], vec!["Daisy"]),
        Participant::new(
            "Charlie".to_string(),
            vec![ExchangePool::Pets],
            vec!["Stella"],
        ),
        Participant::new(
            "Astra".to_string(),
            vec![ExchangePool::Pets],
            vec!["Bailey"],
        ),
        Participant::new("Freya".to_string(), vec![ExchangePool::Pets], vec!["Astra"]),
        Participant::new("Lily".to_string(), vec![ExchangePool::Pets], vec!["Freya"]),
        Participant::new(
            "Daisy".to_string(),
            vec![ExchangePool::Pets],
            vec!["Charlie"],
        ),
        Participant::new("Luca".to_string(), vec![ExchangePool::Pets], vec![]),
        Participant::new("Kona".to_string(), vec![ExchangePool::Pets], vec![]),
    ];
    // get cli args
    let pool_arg = std::env::args().nth(1).expect("No pool specified");
    let pool = match pool_arg.as_str() {
        "island" => ExchangePool::IslandLife,
        "graber" => ExchangePool::Grabergazureson,
        "pets" => ExchangePool::Pets,
        _ => panic!("Invalid pool specified"),
    };

    let mut rng = rand::thread_rng();
    let mut filtered_participants = participants
        .iter()
        .filter(|p| p.exchange_pools.contains(&pool))
        .collect::<Vec<&Participant>>();

    // this is really dirty
    filtered_participants.shuffle(&mut rng);
    while contains_exclusions(&filtered_participants) {
        filtered_participants.shuffle(&mut rng);
    }
    filtered_participants.iter().enumerate().for_each(|(i, p)| {
        let receiver = &filtered_participants[(i + 1) % filtered_participants.len()].name;
        let sender = &p.name;
        println!("{} -> {}", sender, receiver);
    });
    let year = Local::now().year();
    println!("Letter for {}: {}", year, letter_for_pool(pool));
}
