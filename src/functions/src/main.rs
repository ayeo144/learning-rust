fn equation1(a: f64, b: f64) -> f64 {
    let c: f64 = (a - b) / (a + b);
    c
}


fn equation2(a: Vec<f64>) -> f64 {
    let mut sum = 0.0;

    let vector_length = a.len();

    for i in 0..vector_length {
        sum = sum + a[i];
    };

    sum
}


fn main() {
    let result1: f64 = equation1(12.01, 18.25);
    println!("Result1: {:?}", result1);

    let input_vec = vec![11.1, 0.76, 5.03];
    let result2: f64 = equation2(input_vec);
    println!("Result2: {:?}", result2);
}
