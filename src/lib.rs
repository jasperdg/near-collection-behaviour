#[cfg(feature = "wee_alloc")]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
#[allow(dead_code)]
use near_sdk::{
	near_bindgen,
	json_types::{
		U128, 
		U64
	},
	collections::{
		UnorderedMap,
		TreeMap,
	}
};
use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshDeserialize, BorshSerialize)]
struct MilkyWay {
	id: u64,
	lifeforms: UnorderedMap<u64, String>,
	nonce: u64,
}

impl MilkyWay {
	pub fn new(
		galaxy_id: u64,
		id: u64
	) -> Self {
		Self {
			id,
			lifeforms: UnorderedMap::new(format!("galaxy:{}milkyway::{}::lifeforms", galaxy_id, id).as_bytes().to_vec()),
			nonce: 0
		}
		
	}

	pub fn spawn_lifeform(
		&mut self, 
		lifeform_name: String
	) {
		self.lifeforms.insert(&self.nonce, &lifeform_name);
		self.nonce += 1;
	}

	pub fn cross_breed(
		&mut self, 
		lifeform_id_1: u64,
		lifeform_id_2: u64,
	) {
		let lifeform_1 = self.lifeforms.get(&lifeform_id_1).expect("lifeform doesn't exist");
		let lifeform_2 = self.lifeforms.get(&lifeform_id_2).expect("lifeform doesn't exist");
		let new_species = format!("{}{}", lifeform_1, lifeform_2).to_string();
		println!("new species is: {}", new_species);
		self.spawn_lifeform(new_species);
	}

	pub fn get_lifeform(
		&mut self,
		id: u64
	) -> String {
		return self.lifeforms.get(&id).expect("lifeform with this id doesn't exist");
	}
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Galaxy {
	id: u64,
	milkyways: UnorderedMap<u64, MilkyWay>,
	nonce: u64
}

impl Galaxy {
	pub fn new(
		id: u64
	) -> Self {
		Self {
			id,
			milkyways: UnorderedMap::new(format!("galaxy:{}:milkyways", id).as_bytes().to_vec()),
			nonce: 0
		}
	}

	pub fn spawn_milkyway(&mut self) {
		let milkyway = MilkyWay::new(self.id, self.nonce);
		self.milkyways.insert(&self.nonce, &milkyway);
		self.nonce += 1;
	}

	pub fn spawn_lifeform(&mut self, milkyway_id: u64, lifeform_name: String) {
		let mut milkyway = self.milkyways.get(&milkyway_id).expect("milkyway doesn't exist");
		milkyway.spawn_lifeform(lifeform_name);
		self.milkyways.insert(&milkyway_id, &milkyway);
	}

	pub fn get_milkyway(
		&self,
		id: u64
	) -> MilkyWay {
		return self.milkyways.get(&id).expect("milkyway with id doesn't exist");
	}
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
struct Universe {
	galaxies: UnorderedMap<u64, Galaxy>,
	nonce: u64
}


#[near_bindgen]
impl Universe {
	pub fn spawn_galaxy(&mut self) {
		let galaxy = Galaxy::new(self.nonce);
		self.galaxies.insert(&self.nonce, &galaxy);
		self.nonce += 1;
	}

	pub fn spawn_milkyway(&mut self, galaxy_id: u64) {
		let mut galaxy = self.galaxies.get(&galaxy_id).expect("galaxy with id doesn't exist");
		galaxy.spawn_milkyway();
		self.galaxies.insert(&galaxy_id, &galaxy);
	}
	
	pub fn spawn_lifeform(&mut self, galaxy_id: u64, milkyway_id: u64, lifeform_name: String) {
		let mut galaxy = self.galaxies.get(&galaxy_id).expect("galaxy with id doesn't exist");
		let mut milkyway = galaxy.get_milkyway(milkyway_id);
		milkyway.spawn_lifeform(lifeform_name);
	}

	pub fn spawn_lifeform_2(&mut self, galaxy_id: u64, milkyway_id: u64, lifeform_name: String) {
		let mut galaxy = self.galaxies.get(&galaxy_id).expect("galaxy with id doesn't exist");
		galaxy.spawn_lifeform(milkyway_id, lifeform_name);
		// self.galaxies.insert(&galaxy_id, &galaxy);
	}

	pub fn cross_breed(&mut self, galaxy_id: u64, milkyway_id: u64, lifeform_id_1: u64, lifeform_id_2: u64) {
		let mut galaxy = self.galaxies.get(&galaxy_id).expect("galaxy with id doesn't exist");
		let mut milkyway = galaxy.get_milkyway(milkyway_id);
		milkyway.cross_breed(lifeform_id_1, lifeform_id_2);
	}

	pub fn get_galaxy(
		&self,
		id: u64
	) -> Galaxy {
		return self.galaxies.get(&id).expect("galaxy with id doesn't exist");
	}

	pub fn get_milkyway(
		&self,
		galaxy_id: u64,
		milkyway_id: u64,
	) -> MilkyWay {
		let galaxy = self.galaxies.get(&galaxy_id).expect("galaxy with id doesn't exist");
		return galaxy.milkyways.get(&milkyway_id).expect("milkyway with id doesn't exist");
	}

	pub fn get_lifeform(
		&self,
		galaxy_id: u64,
		milkyway_id: u64,
		id: u64
	) -> String {
		let galaxy = self.galaxies.get(&galaxy_id).expect("galaxy with id doesn't exist");
		let milkyway = galaxy.milkyways.get(&milkyway_id).expect("milkyway with id doesn't exist");
		return milkyway.lifeforms.get(&id).expect("lifeform with id doesn't exist");
	}
}

impl Default for Universe {
	fn default() -> Self {
		Self {
			galaxies: UnorderedMap::new(b"galaxies".to_vec()),
			nonce: 0
		}
	}
}


#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
	use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    use super::*;

    fn alice() -> String {
        "alice.near".to_string()
    }
    fn bob() -> String {
        "bob.near".to_string()
    }
    fn carol() -> String {
        "carol.near".to_string()
    }

    fn get_context(predecessor_account_id: String) -> VMContext {
        VMContext {
            current_account_id: alice(),
            signer_account_id: bob(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "test".to_string(),
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1_000_000_000_000_000_000_000_000_000u128,
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
	}
	
	#[test]
    fn first_questionable_behaviour() {
        let context = get_context(carol());
        testing_env!(context);
		let mut contract = Universe::default();
	
		contract.spawn_galaxy();
		contract.spawn_milkyway(0);
		contract.spawn_lifeform_2(0, 0, "dog".to_string()); // Due to the missing ".insert"'s in the parent structs method this is throws: "The collection is an inconsistent state. Did previous smart contract execution terminate unexpectedly?"
		// I'd expect that if there's no insert in the parent struct that the childs storage pointers wouldnt be mutaded _at all_ instead there's some incosistency
	}
	
	#[test]
	fn second_questionable_behaviour() {
        let context = get_context(carol());
        testing_env!(context);
		let mut contract = Universe::default();
		contract.spawn_galaxy();
		contract.spawn_milkyway(0);
		// We are able to spawn life forms now and cross breed them (see print statement)
		// That's strange because the parent doesn't re-insert state - this becomes a problem when you try to `get` the new species but the modifying of the new state works fine
		// _without reinserting_ this is causing weird bugs in large projects like Flux
		// This makes it seem like there _is_ some reference being mutaded
		contract.spawn_lifeform_2(0, 0, "cat".to_string()); 
		contract.spawn_lifeform_2(0, 0, "dog".to_string()); 
		contract.cross_breed(0, 0, 0, 1); 
	}

}