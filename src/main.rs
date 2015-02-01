#![feature(rand)]
use std::rand::{self, Rng};
use std::num::{Float, Int};
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
	match a.fitness - b.fitness {
		n if n > 0.0 => {Ordering::Greater},
		n if n < 0.0 => {Ordering::Less},
		_ 		     => {Ordering::Equal}
	}
}

fn main() {
	let solution = vec![(3, 3.0), (2, -1.0), (0, 5.0)];		// f(x) = 3x^3 - x^2 + 0x + 5

	let iterations = 22;		// iterations to perform
	let size : i64 = 2.pow(iterations + 2);

	let mut rng = rand::thread_rng();

	let mut current : Vec<Gene> = Vec::new();

	println!("{}", "Generating initial population...");
	for _ in 0..size {
		current.push(Gene::new(vec![(3, rng.gen_range(MIN, MAX)), (2, rng.gen_range(MIN, MAX)), 
						(1, rng.gen_range(MIN, MAX)),(0, rng.gen_range(MIN, MAX))]));
	}

	for i in 0..iterations {
		println!("Beginning iteration {}", i);
		let mut old = current.clone();
		current.clear();
		current.shrink_to_fit();

		for _ in 0..200 {
			let x = rng.gen_range(-100.0, 100.0);
			let y = f(&solution, x);

			assert!(y == (3.0 * x.powi(3) - x.powi(2) + 5.0) as f32);

			for ref mut g in old.iter_mut() {
				g.fitness += ((g.compute_at(x) - y)/y).abs();
			}
		}

		old.sort_by(compare);

		let mut last : Option<&mut Gene> = None;
		let len = old.len();

		for x in old[0..(len/2)].iter_mut() {
			match last {
				None 	   => {last = Some(x)},
				Some(prev) => {
					let mut eq1 = prev.eq.clone();
					let mut eq2 = x.eq.clone();
					let len1 = eq1.len();
					let len2 = eq2.len();
					let mut a = eq1.split_off(2);
					let mut b = eq2.split_off(2);
					a.append(&mut eq2);
					eq1.append(&mut b);
					b = eq1;

					for x in a.iter_mut() {
						let (exp, coeff) = *x;
						*x = (exp, coeff + rng.gen_range(-1.0, 1.0));
					}

					for x in b.iter_mut() {
						let (exp, coeff) = *x;
						*x = (exp, coeff + rng.gen_range(-1.0, 1.0));
					}

					current.push(Gene::new(a));
					current.push(Gene::new(b));

					last = None
				}
			}
		}
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
