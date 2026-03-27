#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Env, Address, Symbol, String
};

#[derive(Clone)]
#[contracttype]
pub struct Task {
    pub creator: Address,
    pub worker: Option<Address>,
    pub description: String,
    pub reward: i128,
    pub completed: bool,
}

#[contract]
pub struct MicroTaskContract;

#[contractimpl]
impl MicroTaskContract {

    // Create a new task
    pub fn create_task(env: Env, creator: Address, description: String, reward: i128) {
        creator.require_auth();

        let count_key = Symbol::short("COUNT");
        let mut count: u32 = env.storage().persistent().get(&count_key).unwrap_or(0);

        let task = Task {
            creator: creator.clone(),
            worker: None,
            description,
            reward,
            completed: false,
        };

        let task_key = (Symbol::short("TASK"), count);
        env.storage().persistent().set(&task_key, &task);

        count += 1;
        env.storage().persistent().set(&count_key, &count);
    }

    // Accept a task
    pub fn accept_task(env: Env, worker: Address, task_id: u32) {
        worker.require_auth();

        let key = (Symbol::short("TASK"), task_id);
        let mut task: Task = env.storage().persistent().get(&key).unwrap();

        if task.worker.is_some() {
            panic!("Task already taken");
        }

        task.worker = Some(worker);
        env.storage().persistent().set(&key, &task);
    }

    // Complete task
    pub fn complete_task(env: Env, worker: Address, task_id: u32) {
        worker.require_auth();

        let key = (Symbol::short("TASK"), task_id);
        let mut task: Task = env.storage().persistent().get(&key).unwrap();

        if task.worker != Some(worker.clone()) {
            panic!("Not assigned worker");
        }

        task.completed = true;
        env.storage().persistent().set(&key, &task);
    }

    // Approve task
    pub fn approve_task(env: Env, creator: Address, task_id: u32) {
        creator.require_auth();

        let key = (Symbol::short("TASK"), task_id);
        let task: Task = env.storage().persistent().get(&key).unwrap();

        if task.creator != creator {
            panic!("Not task creator");
        }

        if !task.completed {
            panic!("Task not completed");
        }

        // 🚧 Token transfer should go here in real implementation
    }

    // Get task
    pub fn get_task(env: Env, task_id: u32) -> Task {
        let key = (Symbol::short("TASK"), task_id);
        env.storage().persistent().get(&key).unwrap()
    }
}