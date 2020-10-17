pub const fn modulo_i16(a: i16, b: i16) -> i16 {
	((a % b) + b) % b
}

pub const fn modulo_i32(a: i32, b: i32) -> i32 {
	((a % b) + b) % b
}

#[cfg(test)]
mod test {
	use assert2::assert;

	#[test]
	fn modulo_i32() {
		assert!(super::modulo_i32(8, 12) == 8);
		assert!(super::modulo_i32(20, 12) == 8);
		assert!(super::modulo_i32(-4, 12) == 8);
		assert!(super::modulo_i32(-16, 12) == 8);

		assert!(super::modulo_i32(12, 12) == 0);
		assert!(super::modulo_i32(24, 12) == 0);
		assert!(super::modulo_i32(-12, 12) == 0);
		assert!(super::modulo_i32(-24, 12) == 0);
	}
}
