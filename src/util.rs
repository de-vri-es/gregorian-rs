pub trait Modulo: Copy {
	fn modulo(self, other: Self) -> Self;
}

impl Modulo for i32 {
	fn modulo(self, b: Self) -> Self {
		((self % b) + b) % b
	}
}
