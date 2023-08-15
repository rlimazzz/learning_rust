struct User {
    name : String,
    age : u32,
    activate : bool,
}


fn main() {
    // this is an instance of the struct we created before
    let mut ryan = User {
        name : String::from("Ryan"),
        age : 17,
        activate : true,
    };
    
    // here we print the string, note that we can't print the struct with only one invocation
    println!("{}, {}, {}", ryan.name, ryan.age, ryan.activate);

    //here we show that because the instance of the struct is immutable we can change it values
    ryan.name = String::from("João");
    ryan.age = 32;
    ryan.activate = false;

    // and we print the changed values again
    println!("{}, {}, {}", ryan.name, ryan.age, ryan.activate);

    // in this line, we have a curious syntax called struct update syntax, where we can use values
    // of other struct just by specifying them with the ..name syntax, and we use his values
    // that are declared after name
    let caio = User { 
        name: String::from("Caio"),
        ..ryan
    };

    //and here we print these values
    println!("{}, {}, {}", caio.name, caio.age, caio.activate);

    //this is called tuple structs

    struct Cor(i32 , i32, i32);

    let Preto = Cor(0, 0 ,0);

    println!("{}", Preto[0]);
}
