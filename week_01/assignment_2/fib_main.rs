fn main(){

	let fib_index:i32 = 7;

	println!("Salida Iterativa: {}",fib_iterative(fib_index) );
	print!("Salida Recursiva: {}", fib(fib_index));

}	
fn fib_iterative(fib_index: i32) -> i32{
	let mut last = 0;
	let mut next: i32;
	let mut current = 1;

	for _ in 1..(fib_index) {

    	next = last + current;
		last = current;
		current = next;
	}

	current
}

fn fib(fib_index: i32) -> i32 {
	fib_recursive(0,1,fib_index-1)
}

fn fib_recursive(last: i32, current: i32, counter: i32) -> i32{
	let next = last + current;

	if (counter - 1) == 0{
		next
	}else{
		fib_recursive(current, next, counter - 1)
	}

}