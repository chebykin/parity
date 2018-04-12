// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Sharding state manager

use parking_lot::RwLock;

use {Database, ShardId, Head};

pub struct State {
	db: Box<Database>,
	shard_id: RwLock<ShardId>,
	head: RwLock<Head>,
}

impl State {
	pub fn new(shard_id: ShardId, db: Box<Database>) -> Self {
		let head = db.load_head(&shard_id).expect("todo: what is the initial head?");

		State {
			db: db,
			shard_id: RwLock::new(shard_id),
			head: RwLock::new(head),
		}
	}

	pub fn shard(&self) -> ShardId {
		*self.shard_id.read()
	}

	pub fn change_shard(&self, new_shard_id: ShardId) {
		let mut shard_id = self.shard_id.write();
		let mut head = self.head.write();

		*head = self.db.load_head(&shard_id).expect("todo: what is the initial head?");
		*shard_id = new_shard_id;
	}

	pub fn head(&self) -> Head {
		self.head.read().clone()
	}
}