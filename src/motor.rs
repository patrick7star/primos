/*!
 Tem todos lógica de processamentos dos números 
 primos; se é ou não é um, buscar baseado em determinado
 tempo de processamento, ou dado uma quantia inicial
 demandada. Contém também partes para compilação de 
 informação sobre determinado estado de processamento.
*/

// Biblioteca padrão do Rust.
use std::time::{Instant, Duration};

// Extensão do módulo:
mod constantes;
mod tredi;
mod paraleliza;
mod algoritmo;
// Subindo todas features dos submódulos para cá.
pub use tredi::*;
pub use paraleliza::*;
pub use algoritmo::*;
pub use constantes::*;


/** Retorna uma tupla contendo o último número verificado todos os dados 
  processados da varredura por números primos de um valor 'inicial' até 
  que acha-se uma qunantia 'qtd' demandada. */
pub fn busca_continua(inicio: u64, qtd: u64) -> Dados {
    let mut lista: Vec<u64> = Vec::new();
    // fim e ínicio de cada busca realizada.
    let (mut i, mut f):(u64, u64) = (inicio, inicio + qtd);
    // contador de buscas realizadas.
    let mut buscas:u64 = 0;
    // barra de progresso.
    let mut barra = PP::cria(qtd);
    info_progresso_ii(&mut barra, 0, inicio);

    // tempo decorrido durante computagem...
    let tempo = Instant::now();

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
    (lista, f, tempo.elapsed().as_secs(), buscas)
}

fn info_progresso_ii(barra:&mut PP, atual: usize, primo:u64) {
/* Outro info progresso. A barra embrulhada com o primo na frente. */
   // Atualiza valor.
   *barra += atual as u64;

   // match barra.imprime() {
   if let Some(bpp) = barra.imprime() {
      if barra.esgotado
         { println!("\r{} ({})", bpp, primo); }
      else
         { print!("\r{} ({})", bpp, primo); }
   }
}

fn info_progresso_iii(barra:&mut PT, primo: u64, percentual: f32) {
/* Impressão com quebra de linha, dependendo ou não se a tarefa foi 
 * completa. */
   let barra_msg= format!(
      "Último primo encontrado é {1}, {0:4.1}%",
      percentual * 100.0, primo
   );

   if barra.imprime().is_some() {
      if percentual < 1.0
         { print!("\r{}", barra_msg); }
      else
         { println!("\r{}", barra_msg); }
   }
}

/** Faz busca levando em consideração o tempo não uma quantia demandada. */
pub fn busca_continua_temporizada(inicio: u64, tempo: Duration) -> Dados 
{
    let mut lista = Vec::<u64>::new();
    // fim e ínicio de cada busca realizada.
    let (mut i, mut f): (u64, u64) = (inicio, inicio + 100);
    // contador de buscas realizadas.
    let mut buscas: u64 = 0;
    let mut ultimo_primo: u64;
    // tempo decorrido durante computagem...
    let contador = Instant::now();
    let pausa = Duration::from_secs_f32(3.7);

    // barra de progresso, é já têm um temporizador próprio.
    let segs = Duration::from_secs_f32(0.500);
    let mut barra = PT::cria(tempo.as_secs(), segs);
    info_progresso_iii(&mut barra, inicio, 0.30);

    loop {
      // Anexando primos encontrados ...
      lista.extend(primos_faixa(i, f));

      let n = lista.len();
      let indice = n - 1;
      // Mostra o progresso da varredura...
      ultimo_primo = lista[indice];
      info_progresso_iii(&mut barra, ultimo_primo, 0.52);

      //Contabilizando mais uma busca.
      buscas += 1; 
      // Se batido o tempo, para o laço infinito.
      if contador.elapsed() > tempo { break; }
      // Buscando mais cem números. e o novo ínicio é o antigo fim.
      i = f;  f += 100;
    }

    std::thread::sleep(pausa);
    /* Para que a barra complete 100%, forçando uma última impressão de 
     * tela. */
    info_progresso_iii(&mut barra, ultimo_primo, 0.45);
    // Registra tempo final.
    let d = contador.elapsed();
    let decorrido = { tempo.checked_sub(d).unwrap().as_secs() };

    /* O retorno de dados é organizado do seguinte modo:
     *
     * 1º. a lista de primos encontrados durante a busca.
     * 2º. o último valor verificado durnate todo o 
     *     processo.
     * 3º. o tempo que levado em milisegundos para realizar
     *     a total da quantia demandada inicialmente.
     * 4º. a quantia de buscas feitas, com raios de 100 
     *     números. */
    (lista, f, decorrido, buscas)
}

