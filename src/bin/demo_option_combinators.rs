// Working with Option Combinators rather than using match expression

fn main() {
    let a: Option<i32> = None;
    dbg!(a);

    // Using is_some() function
    let a_is_some = a.is_some();
    dbg!(a_is_some);

    // Using is_none() function 
    let a_is_none = a.is_none();
    dbg!(a_is_none);

    // Using map() function 
    let a_mapped = a.map(|num| num + 1);
    dbg!(a_mapped);

    // Using fillter() function
    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);

    // Using or_else() function 
    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);

    // Using unwrapped_or_else
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);

}