fn main() {
    let mut contador = 0;

    let saida = loop { 
        if contador < 10 {
            contador += 1;
        }else { 
            break contador + 1;
        }
    };
    println!("{}", saida);
    //exemplo do uso de loop, retornando com break

    let mut vari = 5;

    while vari < 20 {
        vari += 1;
        println!("{}", vari);
    }
    //o interessante de usar-se o while é evitar linhas de código gigantescas, enquanto se pode
    //usar apenas uma condição ao lado direito de while, e quando essa condição é satisfeita, o
    //while retorna false, e o loop se finaliza
    
    let elementos = [10, 20, 30 , 40 , 50 , 60];
    for elemento in elementos.iter() {
        println!("O valor é: {}", elemento);
    }
    //aqui está um exemplo de como percorrer uma array usando o loop for, note que é parecido como
    //um for(auto x : lista) de c++ só que como for x in lista.iter(), iter é o que retorna o valor
    //de cada posição da array para elemento
    
    for numero in (1..4).rev() {
        println!("{}", numero);

    }
}
