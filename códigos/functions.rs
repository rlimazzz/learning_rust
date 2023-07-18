fn cinco() -> i32 {
    println!("OLA MUNDO");
    let x : i32 = 10;
    x + 5
}
//O interessante aqui é ver que usar uma linha sem adicionar os ; é como se fosse um return de
//rust, então quando eu crio uma função e alguma linha está sem ; o programa não vai me gerar erro,
//mas sim a função vai me retornar até aquela linha.

fn normal() {
    println!("ADEUS MUNDO");
}
//note que, em uma função em que não está declarado o tipo de retorno, ou seja, em uma função
//"VOID" adicionamos o ; e nossa função desempenha da maneira que queremos, se retiramos isso
//não receberemos algum erro, porém se colocarmos ; na função que retorma receberemos o erro de
//mismatched types, expected i32 found ().

fn main() {
    let y = {
        let x = 3;
        x + 1
    };
    //o que observamos aqui é interessante pois o let pode ser usado como um bloco, perceba que é
    //parecido como uma função que retorna algum valor, se analisarmos a fundo realmente é algo que
    //retorna um valor y.
    println!("{}", cinco());
    normal();

    println!("The value of y is {}", y);
}
