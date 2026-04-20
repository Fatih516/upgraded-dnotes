#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, String, Symbol, Vec, Address
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    id: u64,
    title: String,
    content: String,
    timestamp: u64,
}

// Storage Keys
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {

    // 🔹 Get Notes per User
    pub fn get_notes(env: Env, user: Address) -> Vec<Note> {
        user.require_auth();

        env.storage()
            .instance()
            .get(&user)
            .unwrap_or(Vec::<Note>::new(&env))
    }

    // 🔹 Create Note
    pub fn create_note(env: Env, user: Address, title: String, content: String) -> String {
        user.require_auth();

        if title.len() == 0 || content.len() == 0 {
            return String::from_str(&env, "Title/content tidak boleh kosong");
        }

        let mut notes: Vec<Note> = env.storage()
            .instance()
            .get(&user)
            .unwrap_or(Vec::<Note>::new(&env));

        // ID counter global
        let mut id: u64 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        id += 1;
        env.storage().instance().set(&COUNTER, &id);

        let note = Note {
            id,
            title,
            content,
            timestamp: env.ledger().timestamp(),
        };

        notes.push_back(note);
        env.storage().instance().set(&user, &notes);

        String::from_str(&env, "Note berhasil ditambahkan")
    }

    // 🔹 Delete Note
    pub fn delete_note(env: Env, user: Address, id: u64) -> String {
        user.require_auth();

        let mut notes: Vec<Note> = env.storage()
            .instance()
            .get(&user)
            .unwrap_or(Vec::<Note>::new(&env));

        for i in 0..notes.len() {
            if notes.get(i).unwrap().id == id {
                notes.remove(i).unwrap();
                env.storage().instance().set(&user, &notes);
                return String::from_str(&env, "Note berhasil dihapus");
            }
        }

        String::from_str(&env, "Note tidak ditemukan")
    }
}