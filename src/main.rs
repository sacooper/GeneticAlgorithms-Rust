#![feature(rand)]
extern crate parallel;
use std::rand::{self, Rng};
use std::num::{Float};
use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::cmp::Ordering;

const MAX : f32 = 20.0; 	// Maximum coefficient
const MIN : f32 = -20.0; // Minimum coefficient

#[derive(Clone, Debug)]
struct EquationGene {
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

impl EquationGene {
	fn new(eq : Vec<(i32, f32)>) -> EquationGene {
		EquationGene{fitness : 0.0, eq : eq}
	}

	fn compute_at(&self, x : f32) -> f32{
		f(&self.eq, x)
	}
}



fn compare(a : &EquationGene, b : &EquationGene) -> Ordering {
	match b.fitness - a.fitness {
		n if n > 0.0 => {Ordering::Greater},
		n if n < 0.0 => {Ordering::Less},
		_ 		     => {Ordering::Equal}
	}
}

fn main() {
	let solution = vec![(3, 3.0), (2, -3.0), (0, 5.0)];		// f(x) = 3x^3 - x^2 + 0x + 5

	let iterations = 1000;		// iterations to perform
	let size = 128;

	let mut rng = rand::thread_rng();

	let mut current = Vec::new();

	println!("{}", "EquationGenerating initial population...");
	for _ in 0..size {
		current.push(EquationGene::new(vec![(3, rng.gen_range(MIN, MAX)), (2, rng.gen_range(MIN, MAX)), 
						(1, rng.gen_range(MIN, MAX)),(0, rng.gen_range(MIN, MAX))]));
	}

	
	for i in 0is..iterations {
		if i % 100 == 0 {
			println!("Beginning iteration {}", i);
		}
		let mut old = current.clone();
		let fitted : Arc<Mutex<Vec<EquationGene>>> = Arc::new(Mutex::new(Vec::new()));

		current.clear();
		current.shrink_to_fit();

		let mut tests : Vec<(f32, f32)> = Vec::new();

		for i in 0i32..10000 {
			let x = rng.gen_range(-((i + 1) as f32), i as f32);
			tests.push((x, f(&solution, x)));
		}
		
		let len = old.len();

		let task = |&:chunk : &mut [EquationGene], _|{
			for ref mut g in chunk.iter_mut(){
				for &(x, y) in tests.iter(){
					g.fitness += ((g.compute_at(x) - y)).abs() as f32;
				}
				fitted.lock().unwrap().push(g.clone());};};

		parallel::divide(old.as_mut_slice(), len/8, task);

		let mut old = fitted.lock().unwrap();

		for ref mut g in old.iter_mut(){
			g.fitness /= 1.0;
		}

		old.sort_by(compare);

		let sum = old.iter().fold(0.0, |&: acc, g| acc + g.fitness);
		let mut weights = Vec::new();

		for g in old.iter(){
			weights.push(g.fitness/sum);
		}

		while current.len() < size {
			let mut rand = rng.next_f32();
			let mut mate1 = EquationGene::new(Vec::new());
			let mut mate2 = EquationGene::new(Vec::new());

			for i in 0us..size {
				if weights[i] > rand {
					mate1 = old[i].clone();
				} else {
					rand -= weights[i];
				}
			}

			rand = rng.next_f32();
			for i in 0us..size {
				if weights[i] > rand {
					mate2 = old[i].clone();
				} else {
					rand -= weights[i];
				}
			}


			assert!(mate1.eq.len() != 0);
			assert!(mate2.eq.len() != 0);

			if rng.next_f32() < 0.6 {
				let mut a = Vec::new();
				let mut b = Vec::new();
				let len = mate1.eq.len();
				for i in 0..(len/2){
					a.push(mate1.eq[i]);
					b.push(mate2.eq[i]);
				}
				for i in (len/2)..len{
					a.push(mate2.eq[i]);
					b.push(mate1.eq[i]);
				}

				current.push(EquationGene::new(a));
				current.push(EquationGene::new(b));
			}
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

		// 			current.push(EquationGene::new(eq1));

		// 			last = None
		// 		}
		// 	}
		// }
	}

	for _ in 0..1000{
		let x = rng.gen_range(-1000.0, 1000.0);
		let y = f(&solution, x);
		
		for ref mut g in current.iter_mut() {
			g.fitness = ((g.compute_at(x) - y)).abs();
		}
	}

	current.sort_by(compare);
	println!("{:?}", current[0]);
	
}
