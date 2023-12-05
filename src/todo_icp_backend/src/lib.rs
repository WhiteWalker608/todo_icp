use candid::types::number::Nat;
use ic_cdk::{
    export::candid::{CandidType, Deserialize},
    query, update,
};
use std::cell::RefCell;
use std::collections::BTreeMap;

type TodoStore = BTreeMap<Nat, String>;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct Todo {
    pub id: Nat,
    pub data: String,
}

thread_local! {
    static TODO_STORE: RefCell<TodoStore> = RefCell::default();
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0));
}

#[update]
fn add(todo: String) -> Nat {
    TODO_STORE.with(|todo_store| {
        COUNTER.with(|counter| {
            *counter.borrow_mut() += 1;
            let key = (*counter.borrow()).clone();
            todo_store.borrow_mut().insert(key.clone(), todo);
            key
        })
    })
}

#[query]
fn get(id: Nat) -> Todo {
    let data = TODO_STORE.with(|todo_store| todo_store.borrow().get(&id).cloned());

    Todo {
        id,
        data: data
            .unwrap_or("todo-id does not exist".to_string())
            .to_string(),
    }
}

#[query(name = "getAll")]
fn get_all(page_idx: Nat, page_size: Nat) -> Vec<Todo> {
    TODO_STORE.with(|todo_store| {
        let mut result: Vec<Todo> = Vec::new();
        let start = page_idx * page_size.clone();
        let end = start.clone() + page_size;
        let mut idx = Nat::from(0);
        for (key, value) in todo_store.borrow().iter() {
            if idx < start {
                idx += 1;
            } else if idx < end {
                result.push(Todo {
                    id: key.clone(),
                    data: value.clone(),
                });
                idx += 1;
            } else {
                break;
            }
        }
        result
    })
}

#[update]
fn update(id: Nat, todo: String) {
    TODO_STORE.with(|todo_store| todo_store.borrow_mut().insert(id, todo));
}

#[update]
fn delete(id: Nat) {
    TODO_STORE.with(|todo_store| todo_store.borrow_mut().remove(&id));
}
