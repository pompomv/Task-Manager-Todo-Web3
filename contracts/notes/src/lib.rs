#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan task
#[contracttype]
#[derive(Clone, Debug)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub description: String,
}

// Storage key untuk data task
const TASK_DATA: Symbol = symbol_short!("TASK_DATA");

#[contract]
pub struct TaskContract;

#[contractimpl]
impl TaskContract {
    pub fn get_tasks(env: Env) -> Vec<Task> {
        // 1. ambil data task dari storage
        return env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk membuat task baru
    pub fn create_task(env: Env, title: String, description: String) -> String {
        // 1. ambil data task dari storage
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));

        // 2. Buat object task baru
        let task = Task {
            id: env.prng().gen::<u64>(),
            title: title,
            description: description,
        };

        // 3. tambahkan task baru ke tasks lama
        tasks.push_back(task);

        // 4. simpan tasks ke storage
        env.storage().instance().set(&TASK_DATA, &tasks);

        return String::from_str(&env, "Task berhasil ditambahkan");
    }

    // Fungsi untuk menghapus task berdasarkan id
    pub fn delete_task(env: Env, id: u64) -> String {
        // 1. ambil data task dari storage
        let mut tasks: Vec<Task> = env.storage().instance().get(&TASK_DATA).unwrap_or(Vec::new(&env));

        // 2. cari index task yang akan dihapus menggunakan perulangan
        for i in 0..tasks.len() {
            if tasks.get(i).unwrap().id == id {
                tasks.remove(i);

                env.storage().instance().set(&TASK_DATA, &tasks);
                return String::from_str(&env, "Berhasil hapus task");
            }
        }

        return String::from_str(&env, "Task tidak ditemukan");
    }
}

mod test;
