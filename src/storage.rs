use std::collections::HashMap;
use crate::program::Val;

// const VALID_FNS:  &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const VALID_VARS: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn is_var(c: &char) -> bool {
    VALID_VARS.contains(*c)
}

// pub fn is_func(c: &char) -> bool {
//     VALID_FNS.contains(*c)
// }

// pub fn is_delim(c: &char) -> bool {
//     *c == '\''
// }

pub struct Storage {
    data: HashMap<char, Val>,
}

impl Storage {
    pub fn new() -> Storage {
        let store = Storage {
            data: HashMap::new(),
        };
        // for var_name in VALID_VARS.chars() {
        //     store.data.insert(var_name, Val::zero());
        // }
        return store;
    }

    pub fn get_var(&mut self, var_name: char) -> Option<&Val> {
        if !is_var(&var_name) {
            return None;
        }
        let val = self.data.entry(var_name)
            .or_insert(Val::zero());
        Some(val)
    }

    pub fn set_var(&mut self, var_name: char, new_value: &Val) -> Result<(), String> {
        self.data.insert(var_name, (*new_value).clone());
        Ok(())
    }

    pub fn reset_var(&mut self, var_name: char) -> Result<(), String> {
        self.data.remove(&var_name);
        Ok(())
    }

    pub fn reset_all(&mut self) -> Result<(), String> {
        self.data.clear();
        Ok(())
    }

    pub fn copy(&mut self, from_var: char, to_var: char) -> Result<(), String> {
        let x = self.get_var(from_var).expect("Couldn't find variable");
        let y = (*x).clone();
        self.set_var(to_var, &y)
    }

    pub fn var_as_bool(&mut self, var_name: char) -> Option<bool> {
        let x = self.get_var(var_name).expect("Couldn't find variable");
        return match x {
            Val::Number(n) => Some(*n != 0.0),
            Val::Text(_) => Some(true),
        };
    }
}