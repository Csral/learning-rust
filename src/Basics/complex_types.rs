fn main() {

    /*
    
        * Tuples can be extracted quickly via: let (extraction 1,2,...,N) = tuple_var;
        * Tuples may or may not have full data mentions.
        * let tup: (dt1, dt2..., dtN) and let tup both work

        * Array are fixed size and unlike tuples, all elements must be of same data type.

        * Defined by: let arr: [dt_type ; size] or let arr
        * Quick set all elements -> let arr = [quick_set_val ; num_elements]
    
    */

    let arr : [i32;10] = [5;10];
    println!("All elements: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", arr[0],arr[1],arr[2],arr[3],arr[4],arr[5],arr[6],arr[7],arr[8],arr[9]);

}