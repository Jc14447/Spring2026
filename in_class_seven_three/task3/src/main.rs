//Write a function process_vector that applies a closure to 
//transform each element of a vector. Implement it in both ways:

//Using map and collect
//Using a for loop

fn process_vector_with_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x)); // Apply the closure
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];

    println!("Using map/collect: ");
    let doubled_map = process_vector_with_map(numbers.clone(), |x| x * 2);
    let replaced_map = process_vector_with_map(numbers.clone(), |x| if x > 2 { 0 } else { x });
    
    println!("Doubled: {:?}", doubled_map);
    println!("Replaced: {:?}", replaced_map);

    println!("Using for loop: ");
    let doubled_loop = process_vector_with_for_loop(numbers.clone(), |x| x * 2);
    let replaced_loop = process_vector_with_for_loop(numbers, |x| if x > 2 { 0 } else { x });

    println!("Doubled: {:?}", doubled_loop);
    println!("Replaced: {:?}", replaced_loop);
}