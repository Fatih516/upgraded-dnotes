#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec, Address
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    id: u64,
    title: String,
    completed: bool,
    created_at: u64,
}

// Storage key global counter
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct TaskContract;

#[contractimpl]
impl TaskContract {

    // 🔹 Ambil semua task milik user
    pub fn get_tasks(env: Env, user: Address) -> Vec<Task> {
        user.require_auth();

        env.storage()
            .instance()
            .get(&user)
            .unwrap_or(Vec::<Task>::new(&env))
    }

    // 🔹 Tambah task baru
    pub fn create_task(env: Env, user: Address, title: String) -> String {
        user.require_auth();

        if title.len() == 0 {
            return String::from_str(&env, "Title tidak boleh kosong");
        }

        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&user)
            .unwrap_or(Vec::<Task>::new(&env));

        let mut id: u64 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        id += 1;
        env.storage().instance().set(&COUNTER, &id);

        let task = Task {
            id,
            title,
            completed: false,
            created_at: env.ledger().timestamp(),
        };

        tasks.push_back(task);
        env.storage().instance().set(&user, &tasks);

        String::from_str(&env, "Task berhasil ditambahkan")
    }

    // 🔹 Tandai task selesai
    pub fn complete_task(env: Env, user: Address, id: u64) -> String {
        user.require_auth();

        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&user)
            .unwrap_or(Vec::<Task>::new(&env));

        for i in 0..tasks.len() {
            let mut task = tasks.get(i).unwrap();

            if task.id == id {
                task.completed = true;
                tasks.set(i, task);
                env.storage().instance().set(&user, &tasks);
                return String::from_str(&env, "Task selesai");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }

    // 🔹 Hapus task
    pub fn delete_task(env: Env, user: Address, id: u64) -> String {
        user.require_auth();

        let mut tasks: Vec<Task> = env.storage()
            .instance()
            .get(&user)
            .unwrap_or(Vec::<Task>::new(&env));

        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == id {
                tasks.remove(i).unwrap();
                env.storage().instance().set(&user, &tasks);
                return String::from_str(&env, "Task dihapus");
            }
        }

        String::from_str(&env, "Task tidak ditemukan")
    }
}