use std::rc::Rc;

use super::movement::*;

pub enum PathElement {
	Cons(Move, Rc<PathElement>),
	Nil,
}

use PathElement::{Cons, Nil};

impl PathElement {
	pub fn build_string(&self) -> String {
		let mut curr_element = self;
		let mut path_array = Vec::new();

		loop {
			match curr_element {
				Cons(m, next) => {
					path_array.push(m.to_string());
					curr_element = next;
				},
				Nil => break,
			}
		}

		let path = path_array.join(", ");

		path
	}
}