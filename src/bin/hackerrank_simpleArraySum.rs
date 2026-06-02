/*
 * Complete the 'simpleArraySum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY ar as parameter.
 */

fn simple_array_sum(ar: &[i32]) -> i32 {
    let mut sum = 0;
    for element in ar {
        sum += element;
        println!("Element: {}, Sum: {}", element, sum);
    }
    sum
}

fn simple_array_sum_with_iter(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

fn main() {
    let ar = vec![1, 2, 3];

    println!("{}", simple_array_sum(&ar));
    println!("{}", simple_array_sum_with_iter(&ar));
}