fn main() {
    let mut v = vec![1, 2, 3];
    // Correct way to modify a vector element
    v[0] = 10; 
    println!("v: {:?}", v);
    
    //Alternatively, using iter_mut
    for i in v.iter_mut(){
        *i *=2; //Double the value of each element
    }
    println!("v: {:?}", v);
} 