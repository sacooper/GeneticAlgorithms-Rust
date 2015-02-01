#![feature(rand)]
extern crate parallel;
use std::rand::{self, Rng};
use std::num::{Float, Int};
use std::sync::{Arc, Mutex, Barrier};
use std::thread::Thread;
use std::cmp::Ordering;

const MAX : f32 = 10.0; 	// Maximum coefficient
const MIN : f32 = -10.0; // Minimum coefficient

#[derive(Clone, Debug)]
struct Gene {
	fitness : f32,
	eq : Vec<(i32, f32)>
}

fn f(eq : &Vec<(i32, f32)>, at : f32) -> f32 {
	let mut result : f32 = 0.0;
	for &(exp, coeff) in eq.iter(){
		result += coeff * at.powi(exp);
	};
	result
}

impl Gene {
	fn new(eq : Vec<(i32, f32)>) -> Gene {
		Gene{fitness : 0.0, eq : eq}
	}

	fn compute_at(&self, x : f32) -> f32{
		f(&self.eq, x)
	}
}



fn compare(a : &Gene, b : &Gene) -> Ordering {
	match b.fitness - a.fitness {
		n if n > 0.0 => {Ordering::Greater},
		n if n < 0.0 => {Ordering::Less},
		_ 		     => {Ordering::Equal}
	}
}

fn main() {
	let solution = vec![(3, 3.0), (2, -1.0), (0, 5.0)];		// f(x) = 3x^3 - x^2 + 0x + 5

	let iterations = 1000;		// iterations to perform
	let size : i32 = 64;

	let mut rng = rand::thread_rng();

	let mut current = Vec::new();

	println!("{}", "Generating initial population...");
	for _ in 0..size {
		current.push(Gene::new(vec![(3, rng.gen_range(MIN, MAX)), (2, rng.gen_range(MIN, MAX)), 
						(1, rng.gen_range(MIN, MAX)),(0, rng.gen_range(MIN, MAX))]));
	}

	
	for i in 0..iterations {
		println!("Beginning iteration {}", i);
		let mut old = current.clone();
		let mut fitted : Arc<Mutex<Vec<Gene>>> = Arc::new(Mutex::new(Vec::new()));

		current.clear();
		current.shrink_to_fit();

		let mut tests : Vec<(f32, f32)> = Vec::new();

		for _ in 0..250 {
			let x = rng.gen_range(-10000.0, 10000.0);
			let y = f(&solution, x);
			tests.push((x, y));
		}
		
		let len = old.len();

		let task = |&:chunk : &mut [Gene], _|{
			for ref mut g in chunk.iter_mut(){
				for &(x, y) in tests.iter(){
					g.fitness += 1.0 / (((g.compute_at(x) - y)/y).abs() as f32);
				}
				fitted.lock().unwrap().push(g.clone());};};

		parallel::divide(old.as_mut_slice(), len/8, task);

		println!("Completed fitting");

		let mut old = fitted.lock().unwrap();

		old.sort_by(compare);

		let mut last = None;
		let mut sum = old.iter.fold(0, |acc, g|, acc += g.fitness);


		for _ in 0..size {

		}


		// for x in old.iter_mut() {
		// 	match last {
		// 		None 	   => {last = Some(x)},
		// 		Some(prev) => {
		// 			let mut eq1 = prev.eq.clone();
		// 			let ref eq2 = x.eq;

		// 			for x in eq1.iter_mut() {
		// 				let (exp, coeff) = *x;
		// 				for y in eq2.iter(){
		// 					let (e2, c2) = *y;
		// 					if exp == e2 {
		// 						*x = (exp, (coeff + c2) / 2.0);
		// 						break;
		// 					} 
		// 				}
		// 			}
		// 			if (rng.gen_range(0, 1000) == 1){
		// 				for x in eq1.iter_mut() {
		// 					let (exp, coeff) = *x;
		// 					*x = (exp, coeff + rng.gen_range(-0.5, 0.5));
		// 				}	
		// 			}

		// 			current.push(Gene::new(eq1));

		// 			last = None
		// 		}
		// 	}
		// }
	}

	for _ in 0..1000{
		let x = rng.gen_range(-1000.0, 1000.0);
		let y = f(&solution, x);
		
		for ref mut g in current.iter_mut() {
			g.fitness = ((g.compute_at(x) - y)/y).abs();
		}
	}

	current.sort_by(compare);
	println!("{:?}", current[0]);
	
}
