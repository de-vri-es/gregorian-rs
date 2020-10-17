pub trait Modulo: Copy {
	fn modulo(self, other: Self) -> Self;
}

impl Modulo for i32 {
	fn modulo(self, b: Self) -> Self {
		((self % b) + b) % b
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use assert2::assert;

	#[test]
	fn modulo() {
		assert!((8).modulo(12) == 8);
		assert!((20).modulo(12) == 8);
		assert!((-4).modulo(12) == 8);
		assert!((-16).modulo(12) == 8);

		assert!((12).modulo(12) == 0);
		assert!((24).modulo(12) == 0);
		assert!((-12).modulo(12) == 0);
		assert!((-24).modulo(12) == 0);
	}
}
