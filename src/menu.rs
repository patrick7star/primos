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
   tabela_visualizacao::{ Coluna, Tabela},
   lanca_prompt,
   legivel::tempo
};

// biblioteca do Rust:
use std::fs::read_to_string;
use std::time::Duration;
//use std::fmt::Error;
use std::process::Command;
use std::env::args;

// meus módulos:
use super::banco_de_dados::*;
use super::gerenciamento_bd;
#[doc(inline)]
use super::motor::{
   busca_continua, 
   busca_continua_temporizada,
   // novas inserções:
   filtra_intervalo,
   divide_intervalo,
   gera_processo,
   varredura, 
   simultaneadade
};
use super::{Dados, A_BUSCAR};
use crate::computa_caminho;
// extensão do módulo.
mod tipo;
pub use tipo::{Argumentos, Funcao, transforma};

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
      }

      // mostra informações gerais do programa.
      Argumentos::Infomarcao => 
         { info_bd_binario(); },

      // varre por uma quantia de primos.
      Argumentos::Procura => {
         // obtem-se o número onde parou a última varredura.
         let ultimo = ultimo_numero_computado().unwrap();
         // busca a quantidade e obtem-se eles e seus metadados produzidos.
         let dados = busca_continua(ultimo, A_BUSCAR);
         // guarda tais dados no banco de dados.
         salvar_varredura(dados, salva_no_bd);
      },
      
      // varre por primos dado um determinado tempo.
      Argumentos::ProcuraTempo(t) => { 
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
      },
      
      Argumentos::ProcuraQtd(q) => {
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
      },

      Argumentos::Backup => {
         println!("copiando dados do banco de dados de texto para o binário.");
         // antes de começar faz a cópia do antigo.
         gerenciamento_bd::realiza_backup_bd(); 
      },

      Argumentos::Privado(tipo) => {
         match tipo {
            Funcao::Chamada => {
               const QTD: usize = 4;
               let a = filtra_intervalo(args());
               let mut geral = divide_intervalo(a, QTD);
               for i in geral.drain(..) 
                  { gera_processo(i); }
            } Funcao::Processo => {
               let i = filtra_intervalo(args());
               let dados = simultaneadade(i, 10);
            }
         };
      }
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
   let unv = ultimo_numero_computado().unwrap();
   let primos_encontrados = coleta_todos_primos().unwrap();
   let qtd_primos:u64 = primos_encontrados.len() as u64;
   println!("\nquantidade de primos:\t{}", qtd_primos);
   println!("última verificação:\t{}", unv);

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
