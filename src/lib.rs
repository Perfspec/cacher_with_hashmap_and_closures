use std::collections::HashMap;

pub struct Cacher<T, U, V>
where
	T: Fn(U) -> V,
	U: std::cmp::Eq,
	U: std::hash::Hash,
	U: std::marker::Copy,
	V: std::marker::Copy
{
	closure: T,
	values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where 
	T: Fn(U) -> V,
	U: std::cmp::Eq,
	U: std::hash::Hash,
	U: std::marker::Copy,
	V: std::marker::Copy
{
	pub fn new(closure: T) -> Cacher<T, U, V> {
		Cacher {
			closure,
			values: HashMap::new(),
		}
	}
	
	pub fn value(&mut self, arg: U) -> V {
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