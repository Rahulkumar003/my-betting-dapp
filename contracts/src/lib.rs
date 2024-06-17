#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Map, Vec, Address, String};

#[contract]
pub struct BettingContract;

#[contracttype]
#[derive(Clone)]
pub struct Event {
    name: Symbol,
    description: Symbol,
    outcomes: Vec<Symbol>,
    betting_deadline: u64,
    outcome: Option<Symbol>,
    bets: Map<Address, Map<Symbol, u64>>, // Map from user to outcome to amount bet
    creator: Address, // Address of the event creator
}

#[contractimpl]
impl BettingContract {
    // Define the static creator address
    const CREATOR_ADDRESS: &'static str = "GCEQAJPBJFSKXWVEVIVRR634QA5GHW2LCJW6O2TVVIU5T4GMBRSL2MLS";

    // Helper method to get the static Address
    fn creator_address(env: &Env) -> Address {
        Address::from_string(&String::from(Self::CREATOR_ADDRESS))
    }

    pub fn create_event(
        env: Env,
        event_id: Symbol,
        name: Symbol,
        description: Symbol,
        outcomes: Vec<Symbol>,
        betting_deadline: u64,
    ) {
        let creator = Self::creator_address(&env); // Use the static address
        let event = Event {
            name,
            description,
            outcomes,
            betting_deadline,
            outcome: None,
            bets: Map::new(&env),
            creator: creator.clone(), // Store the creator's address
        };
        env.storage().instance().set(&event_id, &event);
        env.events().publish((creator, "create_event"), (event_id,));
    }

    pub fn place_bet(env: Env, user: Address, event_id: Symbol, outcome: Symbol, amount: u64) {
        let mut event: Event = env.storage().instance().get(&event_id).expect("Event not found");
        assert!(env.ledger().timestamp() < event.betting_deadline, "Betting deadline has passed");

        let mut user_bets = event.bets.get(user.clone()).unwrap_or(Map::new(&env));
        let new_amount = user_bets.get(outcome.clone()).unwrap_or(0) + amount;
        user_bets.set(outcome.clone(), new_amount);
        event.bets.set(user.clone(), user_bets);

        env.storage().instance().set(&event_id, &event);
        env.events().publish((user, "place_bet"), (event_id, outcome, amount));
    }

    pub fn update_outcome_and_distribute(env: Env, event_id: Symbol, outcome: Symbol) {
        let invoker = env.addrress();
        let mut event: Event = env.storage().get(&event_id).expect("Event not found");
        assert!(invoker == Self::creator_address(&env), "Only the event creator can update the outcome");
        
        event.outcome = Some(outcome.clone());

        let winning_outcome = outcome.clone();

        for (user, bets) in event.bets.iter() {
            if let Some(amount) = bets.get(winning_outcome.clone()) {
                env.transfer(&user, amount);
                env.events().publish((user.clone(), "distribute_winnings"), (event_id.clone(), amount));
            }
        }

        env.storage().instance().remove(&event_id);
        env.events().publish((invoker, "update_outcome_and_distribute"), (event_id, outcome));
    }

    pub fn get_balance(env: Env, user: Address) -> i128 {
        env.ledger().balance(&user)
    }
    
    pub fn get_event(env: Env, event_id: Symbol) -> Option<Event> {
        env.storage().instance().get(&event_id)
    }
}
