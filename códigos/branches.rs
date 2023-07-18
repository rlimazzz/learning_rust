fn main() {
    let number = 7;
    if number == 5 {
        println!("É IGUAL A 5");
    }
    else if number == 8 {
        println!("É IGUAL A 8");
    }else {
        println!("NÃO É IGUAL A 5");
    }
    //exemplo de uso do if,else e else if.
    let cond = true;
    let numb = if cond == true {
        6
    }else { 
        0
    };
    println!("{}", numb);
    //esse é um caso interessante, onde adicionamos um valor a variável somente se a condição é
    //verdadeira, podemos usar isso do lado direito da igualdade de let
    //como explicado anteriormente, quando uma função espera um tipo se retornarmos algo que não é
    //o tipo dela receberemos erro, nesse caso se colocarmos uma string como retorno no caso else
    //receberemos o erro de mismatch, note que o valor de if que define o que retornará como valor
    //da variável numb
    
    let saidaa = if cond == true {
        "ola"
    }else { 
        "saida"
    };
    println!("{}", saidaa);
}
