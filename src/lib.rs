use std::collections::HashMap;

pub struct Cacher<T>
where T: Fn(usize) -> usize {
	closure: T,
	values: HashMap<usize, usize>,
}

impl<T> Cacher<T>
where T: Fn(usize) -> usize {
	pub fn new(closure: T) -> Cacher<T> {
		Cacher {
			closure,
			values: HashMap::new(),
		}
	}
	
	pub fn value(&mut self, arg: usize) -> usize {
		match self.values.get(&arg) {
			Some(v) => *v,
			None => {
				let v = (self.closure)(arg);
				self.values.insert(arg, v);
				v
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
    #[test]
	fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}