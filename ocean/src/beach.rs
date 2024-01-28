use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    crabs: Vec<Crab>,
    clan_system: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        Beach { crabs: vec![], clan_system: ClanSystem::new() }
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crabs.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crabs.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        self.crabs.iter().max_by_key(|x| x.speed())
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        self.crabs.iter().filter(|x| x.name() == name).collect()
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        self.crabs.push(Crab::new(
            name,
            1,
            Color::cross(self.crabs[i].color(), self.crabs[j].color()),
            Diet::random_diet()
        ));
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.clan_system
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        self.clan_system.add_crab_name(clan_id, crab_name.to_string());
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let clan1 = self.clan_system.get_clan_member_names(id1);
        if clan1.is_empty() {
            return Err(format!("No clan named {}", id1));
        }
        let clan2 = self.clan_system.get_clan_member_names(id2);
        if clan2.is_empty() {
            return Err(format!("No clan named {}", id2));
        }
        let avg1 = self.get_crabs_avg_speed(&clan1);
        let avg2 = self.get_crabs_avg_speed(&clan2);
        return if avg1 == avg2 {
            Ok(None)
        } else if avg1 > avg2 {
            Ok(Some(id1.to_string()))
        } else {
            Ok(Some(id2.to_string()))
        }
    }

    pub fn get_crabs_avg_speed(&self, names: &Vec<String>) -> u32 {
        let mut speed_sum: u32 = 0;
        for name in names {
            if let Some(crab) = self.find_crabs_by_name(name).first() {
                speed_sum += crab.speed();
            }
        }
        let cnt = u32::try_from(names.len()).unwrap_or(0);
        return if cnt == 0 {
            0
        } else {
            speed_sum / cnt
        }
    }
}
