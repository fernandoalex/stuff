/// This is a simpler implementation of vec!
/// if we pass
///
/// ```
/// let v: Vec<u32> = custom_macro![1,2,3];
/// ```
///
/// This will be expanded to:
///
/// ```
/// {
///    let mut temp_vec = Vec::new();
///    temp_vec.push(1);
///    temp_vec.push(2);
///    temp_vec.push(3);
///    temp_vec
/// }
/// ```
///
/// This is a declarative macro
#[macro_export]
macro_rules! custom_macro {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v: Vec<u32> = custom_macro![1,2,3];
    println!("{}", v[2]);
}
