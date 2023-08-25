/*!
 Tem todos lógica de processamentos dos números 
 primos; se é ou não é um, buscar baseado em determinado
 tempo de processamento, ou dado uma quantia inicial
 demandada. Contém também partes para compilação de 
 informação sobre determinado estado de processamento.
*/

// biblioteca padrão do Rust.
use std::time::Duration;

// biblioteca externa:
extern crate utilitarios;
use utilitarios::barra_de_progresso::{
   ProgressoTemporal, 
   ProgressoPercentual
};

// extensão do módulo:
mod turbina;
mod paraleliza;
mod primos_identificador;
mod temporizador;
// re-exportando ...
pub use turbina::{
   simultaneadade, 
   divide_intervalo,
   Primos, varre
};
pub use paraleliza::{
   filtra_intervalo, 
   gera_processo
};
pub use primos_identificador::e_primoI;
pub use temporizador::*;

/**
retorna 'verdadeiro' ou 'falso' se o número é primo. 

# Exemplos:
```
assert!(e_primo(97));
assert!(e_primo(59));
assert!(e_primo(3));
assert!(e_primo(7));

let nao_primo = 58;
assert_eq!(false, e_primo(nao_primo));
```
*/
#[allow(dead_code)]
pub fn e_primo(n:u64) -> bool{
    // se o valor for 1, já retorna como não-primo.
    if n == 1 || n == 0 { return false; }
    // o mesmo para 2, 3, 5 e 7 que são primos óbvios.
    else if n == 2 || n == 3 || n == 5 || n == 7
      { return true; }
    // testa a divisíbilidade de 1 à n.
    for d in 2..n {
        // se d for divisível, contabiliza-lô.
        if n % d == 0 { return false; }
    }
    // caso, ao pesquisar entre todos valores de 1 à n
    // a quantia de divisores for apenas 2, então
    // o número é primo.
    return true;
}

/** retorna uma array contendo todos primos na 
 faixa passada.

 # Exemplos:
 ```
 let primos_1_a_100 = primos(1,100);
 assert_eq!(
   vec![2, 3, 5, 7, 11, 13, 17, 19, 23,
   29, 31,37, 41, 43, 47, 53, 59,
   61, 67, 71, 73, 79, 83, 89, 97],
   primos_1_a_100
 );
 ```
*/
fn primos_faixa(inicio:u64, fim:u64) -> Vec<u64> {
    // "lista" de primos nesta faixa de intervalo.
    let mut primos:Vec<u64> = Vec::new();

    // testando se é primo um por um...
    for p in inicio..=fim {
        if e_primoI(p) 
        //if e_primo(p)
           { primos.push(p); }
    }

    return primos;
}

/* definindo um novo tipo de dados para a tupla
 * que representa a colêtanea produzida pela 
 * varredura. */
type Dados = (Vec<u64>, u64, u64, u64);


/** retorna uma tupla contendo o último número verificado
 todos os dados processados da varredura por números
 primos de um valor 'inicial' até que acha-se uma qunantia
 'qtd' demandada. */
pub fn busca_continua(inicio:u64, qtd:u64) -> Dados {
    let mut lista: Vec<u64> = Vec::new();
    // fim e ínicio de cada busca realizada.
    let (mut i, mut f):(u64, u64) = (inicio, inicio+qtd);
    // contador de buscas realizadas.
    let mut buscas:u64 = 0;
    // tempo decorrido durante computagem...
    let mut tempo = Cronometro::novo();
    // barra de progresso.
    let mut barra = PP::cria(qtd);
    info_progresso_ii(&mut barra, 0, inicio);

    // descontando mini-processamento acima.
    tempo.reseta();
    while (lista.len() as u64) < qtd {
      // adiciona os primos achados.
      lista.extend(primos_faixa(i, f));

      // barra de progresso.
      let tamanho = lista.len();
      let ultimo_primo = lista[tamanho-1];
      info_progresso_ii(&mut barra, tamanho, ultimo_primo);

      buscas += 1;  //contabilizando mais uma busca.
      // verificação intermediaria para interrupção.
      if (tamanho as u64) >= qtd { break; }
      /* buscando mais cem números.
       * e o novo ínicio é o antigo fim. */
      i = f; f += 100;
    }

    /* o retorno de dados é organizado do seguinte modo:
     * 1º) a lista de primos encontrados durante a busca.
     * 2º) o último valor verificado durnate todo o 
     *     processo.
     * 3º) o tempo que levado em milisegundos para realizar
     *     a total da quantia demandada inicialmente.
     * 4º) a quantia de buscas feitas, com raios de 100 
     *     números. */
    return (lista, f, tempo.marca().as_secs(), buscas);
}

/// informa o progresso da "busca continua".
#[allow(dead_code)]
fn info_progresso(qtd_atual:usize, qtd_demanda:u64, primo:u64) {
   // computa a porcentagem.
   let percentagem:f32 = (qtd_atual as f32) / (qtd_demanda as f32);
   /* impressão com quebra de linha, dependendo ou não
    * se a tarefa foi completa. */
   let barra_msg= format!(
      "último primo encontrado é {1}...{0:4.1}%",
      percentagem * 100.0,
      primo
   );

   if percentagem < 1.0 
      { print!("\r{}", barra_msg); }
   else 
      { println!("\r{}", barra_msg); }
}

type PP = ProgressoPercentual;
type PT = ProgressoTemporal;
/* outro info progresso. A barra 
 * embrulhada com o primo na frente. */
fn info_progresso_ii(barra:&mut PP, atual:usize, primo:u64) {
   // atualiza valor.
   *barra += atual as u64;
   match barra.imprime() {
      Some(bpp) => { 
         if barra.esgotado
            { println!("\r{} ({})", bpp, primo); }
         else
            { print!("\r{} ({})", bpp, primo); }
      } None => ()
   };
}

fn info_progresso_iii(barra:&mut PT, primo:u64) {
   /* impressão com quebra de linha, dependendo ou não
    * se a tarefa foi completa. */
   let barra_msg= format!(
      "último primo encontrado é {1}, {0:4.1}%",
      barra.percentual() * 100.0,
      primo
   );

   match barra.imprime() {
      Some(_) =>  
         if barra.percentual() < 1.0
            { print!("\r{}", barra_msg); }
         else
            { println!("\r{}", barra_msg); }
      None => ()
   };
}

/** faz busca levando em consideração o tempo
  não uma quantia demandada. */
pub fn busca_continua_temporizada(inicio:u64, tempo:Duration) -> Dados {
    let mut lista: Vec<u64> = Vec::new();
    // fim e ínicio de cada busca realizada.
    let (mut i, mut f):(u64, u64) = (inicio, inicio + 100);
    // contador de buscas realizadas.
    let mut buscas:u64 = 0;
    // tempo decorrido durante computagem...
    let contador = Temporizador::novo(tempo);

    // barra de progresso, é já têm um temporizador próprio.
    let mut barra = PT::cria(tempo.as_secs(), 500);
    info_progresso_iii(&mut barra, inicio);
    let mut ultimo_primo: u64;

    loop {
      // adiciona os primos achados.
      lista.extend(primos_faixa(i, f));

      // progresso da varredura.
      let indice = lista.len()-1;
      ultimo_primo = lista[indice];
      info_progresso_iii(&mut barra, ultimo_primo);

      //contabilizando mais uma busca.
      buscas += 1; 
      // se batido o tempo, para o laço infinito.
      if contador.esgotado() { break; }
      // buscando mais cem números.
      // e o novo ínicio é o antigo fim.
      i = f;  f += 100;
    }
    std::thread::sleep(Duration::from_secs_f32(3.7));
    /* para que a barra complete 100%, forçando
     * uma última impressão de tela. */
    info_progresso_iii(&mut barra, ultimo_primo);
    // registra tempo final.
    /* o retorno de dados é organizado do seguinte modo:
     * 1º. a lista de primos encontrados durante a busca.
     * 2º. o último valor verificado durnate todo o 
     *     processo.
     * 3º. o tempo que levado em milisegundos para realizar
     *     a total da quantia demandada inicialmente.
     * 4º. a quantia de buscas feitas, com raios de 100 
     *     números. */
    let decorrido = {
      tempo.checked_sub(contador.contagem())
      .unwrap().as_secs()
    };
    return (lista, f, decorrido, buscas);
}
