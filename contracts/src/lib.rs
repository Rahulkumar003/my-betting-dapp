#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Map, Vec, Address, token};

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

#[contracttype]
#[derive(Clone, PartialEq, Eq)]
pub enum DataKey {
    EventKey(Symbol),
    BetKey(Symbol, Address),
}

#[contractimpl]
impl BettingContract {
    /// creator can post new events
    pub fn create_event(
        env: Env,
        creator: Address,
        event_id: Symbol,
        name: Symbol,
        description: Symbol,
        outcomes: Vec<Symbol>,
        betting_deadline: u64,
    ) {
        creator.require_auth();
        let event = Event {
            name,
            description,
            outcomes,
            betting_deadline,
            outcome: None,
            bets: Map::new(&env),
            creator: creator.clone(),
        };
        env.storage().instance().set(&DataKey::EventKey(event_id.clone()), &event);
        env.events().publish((creator, "create_event"), (event_id,));
    }

    /// user can place bets
    pub fn place_bet(env: Env, user: Address, event_id: Symbol, outcome: Symbol, amount: u64) {
        user.require_auth();
        let mut event: Event = env.storage().instance().get(&DataKey::EventKey(event_id.clone())).expect("Event not found");
        assert!(env.ledger().timestamp() < event.betting_deadline, "Betting deadline has passed");

        let mut user_bets = event.bets.get(user.clone()).unwrap_or(Map::new(&env));
        let new_amount = user_bets.get(outcome.clone()).unwrap_or(0) + amount;
        user_bets.set(outcome.clone(), new_amount);
        event.bets.set(user.clone(), user_bets);

        env.storage().instance().set(&DataKey::EventKey(event_id.clone()), &event);
        env.events().publish((user, "place_bet"), (event_id, outcome, amount));
    }

    /// update the outcome and distribute winnings
    pub fn update_outcome_and_distribute(env: Env, creator: Address, event_id: Symbol, outcome: Symbol) {
        creator.require_auth();
        let mut event: Event = env.storage().instance().get(&DataKey::EventKey(event_id.clone())).expect("Event not found");
        assert!(creator == event.creator, "Only the event creator can update the outcome");

        event.outcome = Some(outcome.clone());
        let winning_outcome = outcome.clone();

        for (user, bets) in event.bets.iter() {
            if let Some(amount) = bets.get(winning_outcome.clone()) {
                // Assuming the token::Client is used for token transfers
                token::Client::new(&env, &creator).transfer(&creator, &user, &amount.into());

                env.events().publish((user.clone(), "distribute_winnings"), (event_id.clone(), amount));
            }
        }

        env.storage().instance().remove(&DataKey::EventKey(event_id.clone()));
        env.events().publish((creator, "update_outcome_and_distribute"), (event_id, outcome));
    }

    /// get the balance of a user (this assumes a token contract with balance_of method)
    pub fn get_balance(env: Env, user: Address) -> i128 {
        // Assuming a token contract with a balance_of method
        let token_client = token::Client::new(&env, &user);
        token_client.balance(&user)
    }

    /// get an event by id
    pub fn get_event(env: Env, event_id: Symbol) -> Option<Event> {
        env.storage().instance().get(&DataKey::EventKey(event_id))
    }

    /// check if a user has placed a bet on an event
    pub fn get_bet(env: Env, event_id: Symbol, user: Address) -> Option<Map<Symbol, u64>> {
        let event: Event = env.storage().instance().get(&DataKey::EventKey(event_id.clone())).expect("Event not found");
        event.bets.get(user)
    }
}
