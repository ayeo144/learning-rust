fn main() {
    // Try and do a basic implementation of the bubble sort algorithm.

    // Example unsorted list/array of integers
    let mut input_list = vec![1, 2, 4, 3, 7, 6, 5];

    println!("The input unsorted list: {:?}", input_list);

    let mut unsorted_until_index = input_list.len() - 1;

    println!("The list contains {:?} elements", unsorted_until_index);

    let mut sorted = false; 

    while sorted != true {

        sorted = true;

        for i in 0..unsorted_until_index {

            // If the value to the right of the current index is greater, we 
            // need to swap their position in the array.
            if input_list[i] > input_list[i + 1] {

                let list_i = input_list[i];
                let list_i_1 = input_list[i + 1];
                input_list[i] = list_i_1;
                input_list[i + 1] = list_i;

                sorted = false;

            }

        }

        unsorted_until_index = unsorted_until_index - 1

    }

    println!("The sorted list: {:?}", input_list);

}
