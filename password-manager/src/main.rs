use std::{collections::HashMap, marker::PhantomData};

struct Unlocked;
struct Locked;

#[derive(Default)]
struct PasswordManager<State = Locked> {
    master_pass: String,
    passwords: HashMap<String, String>,
    state: PhantomData<State>,
}

impl PasswordManager {
    pub fn new(master_pass: String) -> Self {
        PasswordManager {
            master_pass,
            passwords: Default::default(),
            state: Default::default(),
        }
    }
}

// 所有类型的 PasswordManager 都有这两个方法
impl<State> PasswordManager<State> {
    pub fn version(&self) -> String {
        todo!("version")
    }
    pub fn encryption(&self) -> String {
        todo!("encryption")
    }
}

impl PasswordManager<Locked> {
    pub fn unlock(self) -> PasswordManager<Unlocked> {
        PasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
            state: PhantomData::<Unlocked>,
        }
    }
}

// 为 Unlocked 类型实现 独有的方法
impl PasswordManager<Unlocked> {
    pub fn lock(self) -> PasswordManager<Locked> {
        PasswordManager {
            master_pass: self.master_pass,
            passwords: self.passwords,
            state: PhantomData::<Locked>,
        }
    }
    pub fn list_passwords(&self) -> &HashMap<String, String> {
        &self.passwords
    }

    pub fn add_password(&mut self, username: String, password: String) {
        self.passwords.insert(username, password);
    }
}

fn main() {
    let manager = PasswordManager::new("password123".to_owned());
    let mut manager = manager.unlock();
    manager.add_password("username123".to_owned(), "password456".to_owned());
    println!("{:?}", manager.list_passwords());
    manager.lock();
}
