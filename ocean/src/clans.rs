use std::collections::HashMap;
use crate::crab::Crab;
use std::rc::Rc;

#[derive(Debug)]
pub struct ClanSystem {
    clans: HashMap<String, Vec<String>>
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem { clans: HashMap::new() }
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        if let Some(names) = self.clans.get(clan_id) {
            names.clone()
        } else {
            Vec::new()
        }
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        self.clans.get(clan_id).map_or(0, |v| v.len())
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        if let Some(res) = self.clans.iter().max_by(|x, y| x.1.len().cmp(&y.1.len())) {
            Some(res.0.clone())
        } else {
            None
        }
    }

    /**
     * Add the given crab name to the clan of the given clan_id.
     */
    pub fn add_crab_name(&mut self, clan_id: &str, crab_name: String) {
        if let Some(crabs) = self.clans.get_mut(clan_id) {
            crabs.push(crab_name);
        } else {
            self.clans.insert(clan_id.to_string(), vec![crab_name]);
        }
    }
}
