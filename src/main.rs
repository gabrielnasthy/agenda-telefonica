use std::io;

fn adicionar(vec: &mut Vec<String>, posi: &i32, valor: String) {
    // Verifica se a posição é válida
    if *posi >= 0 && *posi <= vec.len() as i32 {
        vec.insert(*posi as usize, valor);
        println!("Número adicionado com sucesso!");
    } else {
        println!("Posição inválida!");
    }
}

fn remover(vec: &mut Vec<String>, posi: &i32) {
    // Verifica se a posição é válida
    if *posi >= 0 && *posi < vec.len() as i32 {
        vec.remove(*posi as usize);
        println!("Número removido com sucesso!");
    } else {
        println!("Posição inválida!");
    }
}
fn consultar(vec: &Vec<String>, posi: &i32) {
    // Verifica se a posição é válida
    if *posi >= 0 && *posi < vec.len() as i32 {
        println!("O valor na posição {} é: {}", posi, vec[(*posi as usize)]);
    } else {
        println!("Posição inválida!");
    }
}
fn ler()-> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
        input.trim().to_string()
}

fn convertInt(input:String)-> i32{
    input.trim().parse::<i32>().expect("not a number")
}

fn main() {
    let mut agenda: Vec<String> = Vec::new();

    let mut posicao = 0;

    loop {
        println!("Seja Bem Vindo a sua Agenda Virtual");
        println!("escolha qual operação deseja: \n
          1 - Adicionar Número. \n
          2 - Remover Um Número. \n
          3 - Cunsultar. \n");
    let mut opcao = convertInt(ler());

        match opcao{
            1 =>{
                println!("Digite o número a adicionar:");
                let numero = ler();
                agenda.push(numero);
                println!("Número adicionado ao final da agenda!");
            }
            2 =>{
                println!("Qual posição deseja remover?");
                let posicao = convertInt(ler());
                remover(&mut agenda, &posicao);
            }
            3 =>{
                println!("Qual posição deseja consultar?");
                let posicao = convertInt(ler());
                consultar(&agenda, &posicao);
            }
            _ => {
                println!("Não tem essa opção ");
                break;
            }
        }
    }

}
