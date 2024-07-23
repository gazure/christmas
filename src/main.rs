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
        ExchangePool::IslandLife => 'I' ,
        ExchangePool::Grabergazureson => letters[0],
        ExchangePool::Pets => 'P',
    }
}

#[derive(Debug, Clone)]
struct Participant {
    name: String,
    exchange_pools: Vec<ExchangePool>,
}

impl Participant {
    fn new(name: String, exchange_pools: Vec<ExchangePool>) -> Participant {
        Participant {
            name,
            exchange_pools,
        }
    }
}

fn main() {
    let participants= vec![
        Participant::new("Claire".to_string(), vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson]),
        Participant::new("Grant".to_string(), vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson]),
        Participant::new("Anne".to_string(), vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson]),
        Participant::new("Duncan".to_string(), vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson]),
        Participant::new("Noel".to_string(), vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson]),
        Participant::new("K-Lee".to_string(), vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson]),
        Participant::new("Steve".to_string(), vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson]),
        Participant::new("Linda".to_string(), vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson]),
        Participant::new("Chris".to_string(), vec![ExchangePool::IslandLife, ExchangePool::Grabergazureson]),
        Participant::new("Jim".to_string(), vec![ExchangePool::Grabergazureson]),
        Participant::new("Kari".to_string(), vec![ExchangePool::Grabergazureson]),
        Participant::new("Meaghann".to_string(), vec![ExchangePool::Grabergazureson]),
        Participant::new("Alec".to_string(), vec![ExchangePool::Grabergazureson]),
        Participant::new("Eric".to_string(), vec![ExchangePool::IslandLife]),
        Participant::new("Stella".to_string(), vec![ExchangePool::Pets]),
        Participant::new("Bailey".to_string(), vec![ExchangePool::Pets]),
        Participant::new("Kitty".to_string(), vec![ExchangePool::Pets]),
        Participant::new("Charlie".to_string(), vec![ExchangePool::Pets]),
        Participant::new("Astra".to_string(), vec![ExchangePool::Pets]),
        Participant::new("Freya".to_string(), vec![ExchangePool::Pets]),
        Participant::new("Lily".to_string(), vec![ExchangePool::Pets]),
        Participant::new("Daisy".to_string(), vec![ExchangePool::Pets]),
        Participant::new("Luca".to_string(), vec![ExchangePool::Pets]),
        Participant::new("Kona".to_string(), vec![ExchangePool::Pets]),
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
    let mut filtered_participants = participants.iter()
        .filter(|p| p.exchange_pools.contains(&pool))
        .collect::<Vec<&Participant>>();

    filtered_participants.shuffle(&mut rng);
    filtered_participants.iter().enumerate()
        .for_each(|(i, p)| {
            let receiver = &filtered_participants[(i + 1) % filtered_participants.len()].name;
            let sender = &p.name;
            println!("{} -> {}", sender, receiver);
        });
    println!("Letter for 2023: {}", letter_for_pool(pool));
}
