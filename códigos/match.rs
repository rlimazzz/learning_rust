fn main(){
    let age :u32 = 32;
    //in match case is important to us to cover all the possible inputs, else the rust compilator
    //is gonna give us errors, note the _ case in the match, it's to handle every other input that
    //are not in the range of the 2 first others
    match age{
        1..=18 => println!("Important birthday!"),
            19..=30 => println!("Sad birthday"),
        _ => println!("Super sad birthday!"),
    };


}
