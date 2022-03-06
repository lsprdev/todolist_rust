/*
Ideias:

No começo do programa será quando eu vou criar um "todo",
vou poder colocar o nome dele e tudo mais,
todos os to-do's serão colocados na mesma pasta,
assim quando eu iniciar o programa eu vou precisar delecionar
qual dos to-dos vou querer ver e etc.. 
Conseguir escrever/apagar algo em um .txt.
Mostrar o que está escrito nesse .txt
Cada linha será uma task

Pasta- todos:
            lista1
            lista2
            lista3
            ...

lista1:
    task1
    task2
    task3
    ...

Posso criar tasks, apagar, editar e marcar como concluídas.

Coisas que precisarei aprender:
Criar pasta, criar arquivos(editar e apagar),
Fazer uma cli funcional e bonita.

*/

use std::env; // Ler input dados junto com cargo run. Ex: Cargo run Hello World
use std::fs;

fn main() {
    // Pequeno teste com frases do Linus Torvalds
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("======================================================");

    println!("Quote:\n \n{}", contents);

}
