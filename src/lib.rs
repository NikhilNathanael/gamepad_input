mod winapi;
mod wrapper;
pub use wrapper::*;

#[cfg(test)]
mod test {
	use windows_error_handling::*;
	use super::wrapper::*;
	#[test]
	fn test () {
		println!("{:?}", get_last_error_message());
		let mut x: [_; 4] = std::array::from_fn(|i| XInputGamepad::new(i as u32));
		loop {
			x.iter_mut().for_each(|x| {
				x.update();
				x.set_vibration(50000, 50000);
			});
			eprintln!("{:?}", x);
			std::io::stdin().read_line(&mut String::new()).expect("could not read line");
		}
	}
}
