/// Function to perform insertion sort on a mutable slice of i32 elements.
/// This sorts the elements in-place in ascending order.
///
/// # Arguments
///
/// * `arr` - A mutable slice of integers that needs to be sorted.

fn insertion_sort(arr: &mut [i32]){
    // Start from the second element as the first element is considered sorted.
    for i in 1..arr.len(){
        let key = arr[i]; // The element to be positioned correctly in the sorted part.
        let mut j = i; // Start with the current index.

        // Move elments of arr[0..i-1], that are greater than key, to one posiion ahead
        // of thair current position.
        while j > 0 && arr[j -1] > key{
            arr[j] = arr[j -1]; // Shift element to the right.
            j -= 1; // Move to the next element.
        }

        // Insert the key at the correct position.
        arr[j] = key;
    }
}

fn main() {
    let mut numbers = [23, 5, 17, 89, 12, 47, 3, 66, 29, 10, 54, 78, 31, 91, 44, 8, 15, 73, 38, 25]; // An unsorted array of inegers.
    insertion_sort(&mut numbers); // Call the insertion sort function.
    println!("Sorted array: {:?}", numbers) // Print the sorted array.
}
