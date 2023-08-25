/*! 
 Mesmo o programa sendo um executável, se algum dia 
 tiver alguma utilidade para outro, então pode servir
 como uma biblioteca, com todas suas funções usadas 
 internamentes compartilhadas aqui.

 # Na biblioteca: 
 * Aqui são guardados todo o ferramental do programa. 

 * O motor de buscas dos números primos; 

 * As funções que carregam os dados produzidos
   e armazenam no banco de dados. `utilitarios` de

 * bibliotecas externas que são utilizados no 
   programa e, também ferramentas criadas, originalmente,
   para o próprio programa.
*/


// biblioteca externa:
extern crate utilitarios;
use utilitarios::{
   tabela_visualizacao::{Coluna, Tabela},
   lanca_prompt,
   legivel::tempo
};

// biblioteca do Rust:
use std::fs::read_to_string;
use std::time::Duration;
//use std::fmt::Error;
use std::process::{Child, Command};
use std::env::args;

// meus módulos:
use super::banco_de_dados::*;
#[doc(inline)]
use super::motor::{
   busca_continua, 
   busca_continua_temporizada,
   // novas inserções:
   filtra_intervalo,
   divide_intervalo,
   gera_processo,
   simultaneadade,
   Primos
};
use super::{Dados, A_BUSCAR};
use crate::computa_caminho;
// extensão do módulo.
mod tipo;
pub use tipo::{Argumentos, Funcao, transforma};
mod stream_serializado;
pub use stream_serializado::{despeja_bytes, colhe_resultado};

/**
 menu para gerenciar os argumentos passados 
 e, baseado neles, chamar as ferramentas
 certas na execução. 
*/
pub fn menu(argumento:Argumentos, pula_confirmacao:bool) {
   // baseado no tipo de argumento obtido...
   match argumento {
      // informação de ajuda:
      Argumentos::Ajuda => {
         let caminho = dbg!(computa_caminho("data/ajuda.txt"));
         let conteudo = read_to_string(caminho);
         println!("{}", conteudo.unwrap()); 
      } Argumentos::Infomarcao => 
         // mostra informações gerais do programa.
         { info_bd_binario(); },
      Argumentos::Procura => {
         // varre por uma quantia de primos.
         // obtem-se o número onde parou a última varredura.
         let ultimo = ultimo_numero_computado().unwrap();
         // busca a quantidade e obtem-se eles e seus metadados produzidos.
         let dados = busca_continua(ultimo, A_BUSCAR);
         // guarda tais dados no banco de dados.
         salvar_varredura(dados, salva_no_bd);
      } Argumentos::ProcuraTempo(t) => { 
         // varre por primos dado um determinado tempo.
         let t: u64 = t as u64;
         println!("seu tempo demandado: {}", tempo(t, false));
         // carrega de onde parou:
         let onde_parou = ultimo_numero_computado().unwrap();
         // cria "intervalo" de computação.
         let intervalo = Duration::from_secs(t);
         // começa... e já 'captura' dados da consulta...
         let dados = busca_continua_temporizada(onde_parou, intervalo);
         // informação do termino.
         envia_notificao(&dados);
         // confirmando se deseja armazenar os dados:
         if pula_confirmacao {
            // salvando de forma direta.
            salva_no_bd(dados);
            println!("salvando... \u{2705}");
         }
         else 
            { salvar_varredura(dados, salva_no_bd); }
      } Argumentos::ProcuraQtd(q) => {
         println!("sua quantia demandada: {} primos",q);
         // último número verificado.
         let unv = ultimo_numero_computado().unwrap();
         // dados conseguidos na busca.
         let dados = busca_continua(unv, q);
         // guarda no banco de dados a varredura.
         if pula_confirmacao {
            // salva automaticamente.
            salva_no_bd(dados);
         }
         else 
            { salvar_varredura(dados, salva_no_bd); }
      } Argumentos::Backup => {
         println!(
            concat!(
               "copiando dados do banco de dados de",
               "texto para o binário."
            )
         );
         // antes de começar faz a cópia do antigo.
         realiza_backup_bd(); 
      } Argumentos::Privado(tipo) => 
         { forques_demanados(tipo); }
   }
}

// entrada para confirmação, retorna verdadeiro ou falso.
fn salvar_varredura(dados:Dados, funcao:fn(Dados)) -> bool {
   // string contendo dados, futuramente...
   let confirmacao:String;
   /* salvo, ou não... visualizando um 
    * pouco do que foi encontrado. */
   informacao_da_varredura(&dados);
   // pergunta de confirmação.
   confirmacao = lanca_prompt("armazenar dados no BD[sim/não]");
   // obtendo confirmação via teclado.
   if confirmacao.trim() == "sim" { 
      // salvando no caso de "positivo".
      print!("gravando...");
      funcao(dados);
      println!("feito.");
      true 
   } else if confirmacao == "não" { 
      println!("você NÃO QUIS gravar o resultado encontrado.");
      false
   } else {
      println!(
         "\"{}\" não é uma resposta válida, apenas
         \r[sim/não], e sim, têm que ser minúsculas 
         \rambas. Tente novamente!\n", 
         confirmacao
      );
      // relançar novamente a questão.
      return salvar_varredura(dados, funcao);
   }
}

fn informacao_da_varredura(dados:&Dados) {
   let rotulo_i = "qtd. de primos achados";
   let rotulo_ii = "varredura de 100 feitas";
   let rotulo_iii = "onde parou";
   println!(
      "{}:{}\n{}: {}\n{}: {}", 
      rotulo_i, dados.0.len(),
      rotulo_ii, dados.3,
      rotulo_iii, dados.1
   );
}

pub fn info_bd_binario() {
   //let unv = ultimo_numero_computado().unwrap();
   let primos_encontrados = coleta_todos_primos().unwrap();
   let qtd_primos:u64 = primos_encontrados.len() as u64;
   println!(
      "
      \rquantidade de primos:\t{}
      \rúltima verificação:\t{}
      ", qtd_primos,
      ultimo_numero_computado().unwrap() 
   );

   // primeiros primos. 
   let inicio = Coluna::nova(
      "inicio",
      primos_encontrados[0..30]
      .to_vec()
   );
   // primos do centro da distribuição encontrada.
   let meio = (qtd_primos / 2) as usize;
   let centro = Coluna::nova(
      "meio",
      primos_encontrados[meio-15..meio+15]
      .to_vec()
   );
   // limites:
   let f = qtd_primos as usize;
   // últimos primos.
   let fim = Coluna::nova(
      "fim",
      primos_encontrados[f-30..f]
      .to_vec()
   );
   // visualização mais estruturada.
   println!("amostra do produzido até o momento:");

   let mut tabela = Tabela::nova(true);
   tabela.adiciona(inicio);
   tabela.adiciona(centro);
   tabela.adiciona(fim);
   println!("{}", tabela);
}

// notificação de termino da mineração.
fn envia_notificao(dados:&Dados) {
   let mensagem:String = format!(
      "achou {} primos, e o último número verificado foi {}",
      dados.0.len(),
      dados.1
   );
   // executando comando ...
   Command::new("notify-send")
   .arg("--icon=dialog-apply")
   .arg(mensagem.as_str())
   .spawn()
   .unwrap();
}

use crate::banco_de_dados::deleta_caminho;
use std::str::FromStr;
use std::path::Path;
use super::motor::{varre};
/* trazendo para cá, já que, um aninhamento
 * naquele nível fica muito confuso para
 * se mexer. Aqui cuida de chamadas privadas,
 * um modo de dá 'fork' no programa para 
 * funções internas do programa.
 * Os comandos que tal aceita estão abaixo,
 * e claro, tal lista será atualizada no
 * futuro: 
 *    varre <a..=b(intervalo)>
 *    ignção a..=b(intervalo)
 *    inverte-última-inserção
 */
fn forques_demanados(tipo: Funcao) {
   const QTD_THREADS: usize = 10;
   // total de forks à "chocar".
   //const QTD_SP: usize = 4;
   let qtd_sp = num_cpus::get();
   // pulando nome do programa...
   let mut argumentos = args().skip(1);

   match tipo {
      Funcao::Chamada => {
         let a = filtra_intervalo(argumentos);
         let mut geral = divide_intervalo(a, qtd_sp);
         let mut forques: Vec<Child>;
         forques = Vec::with_capacity(2 * qtd_sp);
         for (o, i) in geral.drain(..).enumerate() {
            println!("{}º. {:#?}", (o+1), i);
            let processo = gera_processo(i).unwrap(); 
            forques.push(processo); 
         }
         let mut resultados = Primos::with_capacity(30_000);
         for sp in forques.iter_mut() { 
            let r = colhe_resultado(sp);
            resultados.extend(r); 
         }
         println!(
            "primos encontrados: {}", 
            resultados.len()
         );
      } Funcao::Processo => {
         let i = filtra_intervalo(argumentos);
         let dados = simultaneadade(i, QTD_THREADS);
         /* como trecho é geralmente chamado
          * via fork, então todo conteúdo em
          * bytes é despejado via saída padrão.
          */
         despeja_bytes(dados);
      } Funcao::Inversao => {
         let t = ByteOrdem::LittleEndian;
         let _t = ByteOrdem::BigEndian;
         inverte_byte_order_de_todos_dados(t);
      } Funcao::Deleta => {
         // descarta.
         std::mem::drop(argumentos.next());
         // caminho a deletar.
         let caminho = argumentos.next().unwrap();
         let caminho = Path::new(caminho.as_str());
         // tempo até que a ação seja feita.
         let tempo = argumentos.next().unwrap();
         let tempo = u64::from_str(tempo.as_str()).unwrap();
         let tempo = Duration::from_secs(tempo);
         // deletando diretório...
         deleta_caminho(caminho.to_path_buf(), tempo);
      } Funcao::NovaChamada => {
         let a = filtra_intervalo(argumentos);
         let mut geral = divide_intervalo(a, qtd_sp);
         let mut forques: Vec<Child>;
         forques = Vec::with_capacity(qtd_sp + 3);

         for (o, i) in geral.drain(..).enumerate() {
            println!("{}º. {:#?}", (o+1), i);
            let processo = gera_processo(i).unwrap(); 
            forques.push(processo); 
         }

         let mut resultados = Primos::with_capacity(30_000);
         for sp in forques.iter_mut() { 
            let r = colhe_resultado(sp);
            resultados.extend(r); 
         }

         println!(
            "primos encontrados: {}", 
            resultados.len()
         );
      } Funcao::PoderTotal => {
         let dados = varre(filtra_intervalo(argumentos));
         /* como trecho é geralmente chamado
          * via fork, então todo conteúdo em
          * bytes é despejado via saída padrão.  */
         despeja_bytes(dados);
      }
   };
}
