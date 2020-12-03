use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() -> io::Result<()> {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("input a miner address: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut miner_addr)?;
    println!("Difficulty: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut difficulty)?;
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("we need an integer");
    println!("generating genesis block!");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine Block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush()?;
        choice.clear();
        io::stdin().read_line(&mut choice)?;
        println!("");

        match choice.trim().parse() {
            Err(e) => println!("Parse err: {:?}", e),
            Ok(c) => match c {
                0 => {
                    println!("exiting!");
                    process::exit(0);
                }
                1 => {
                    let mut sender = String::new();
                    let mut reciever = String::new();
                    let mut amount = String::new();

                    println!("Enter sender address: ");
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut sender)?;
                    println!("Enter reciever address: ");
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut reciever)?;
                    println!("Enter amount: ");
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut amount)?;

                    let res = chain.new_transaction(
                        sender.trim().to_string(),
                        reciever.trim().to_string(),
                        amount.trim().parse().unwrap(),
                    );
                    match res {
                        true => println!("Transaction added"),
                        false => println!("Transaction failed"),
                    }
                }
                2 => {
                    println!("Generating block");
                    let res = chain.generate_new_block();
                    match res {
                        true => println!("Block generated successfully"),
                        false => println!("Block generation failed"),
                    }
                }
                3 => {
                    let mut new_diff = String::new();
                    println!("Enter new difficulty: ");
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut new_diff)?;
                    let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                    match res {
                        true => println!("Updated Difficulty"),
                        false => println!("Failed Updated Difficulty"),
                    }
                }
                4 => {
                    let mut new_reward = String::new();
                    println!("Enter new reward: ");
                    io::stdout().flush()?;
                    io::stdin().read_line(&mut new_reward)?;
                    let res = chain.update_reward(new_reward.trim().parse().unwrap());
                    match res {
                        true => println!("Updated reward"),
                        false => println!("Failed Update reward"),
                    }
                }
                _ => println!("invalid option please retry"),
            },
        }
    }
}
