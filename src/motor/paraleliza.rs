
/* Gerador de executável que cria
 * várias forks da função que executa
 * a busca contínua por primos dado
 * o intervalo. Você demanda a quantia
 * necessária.
 */

// biblioteca do Rust:
use std::process::{Stdio, Command};
//use std::env::Args;
use std::str::FromStr;
use std::mem::swap;
// resto do módulo:
use super::turbina::Intervalo;


pub fn string_para_range(s: String) -> Intervalo {
   // decompondo ...
   let mut partes = s.split("..=");
   let inicio = partes.next().unwrap();
   let fim = partes.next().unwrap();

   // transformando em inteiros.
   let mut inicio = u64::from_str(inicio).unwrap();
   let mut fim = u64::from_str(fim).unwrap();
   /* intervalo só trabalhado na ordem crescente,
    * então caso vem o inverso, tal função
    * o inverte. */
   if inicio > fim
      { swap(&mut inicio, &mut fim); } 

   // formando o intervalo em sí.
   return inicio..=fim;
}

/* captura os argumentos e retorna 
 * um intervalo, onde será feito à 
 * busca pela quantia de primos.
 */
use std::iter::Iterator as I;
pub fn filtra_intervalo(i: impl I<Item=String>) -> Intervalo {
   // retira apenas intervalo.
   let intervalo = {
      i.filter(|s| { 
         let p1 = s.contains("..");
         let p2 = s.contains("=");
         /* contém um tipo, ou o outro, ou
          * também a combinação de ambos que 
          * é o caso. */
         p1 || p2
      }).next().unwrap()
   };
   return string_para_range(intervalo);
}

use std::process::Child;
use std::io::Error;
use std::path::Path;
type Processo = Result<Child, Error>;
/* gera vários processos que processas
 * vários intervalos dados. Gera vários
 * forks chamando esta função quantas 
 * vezes achar que for necessário. */
pub fn gera_processo(i: Intervalo) -> Processo {
   // binários otimizados, e o debug se não houver.
   let binario_otimizado = Path::new("target/release/primos");
   let binario_debug = Path::new("target/debug/primos");
   // formando ...
   let mut cmd = {
      let cmd_str: &str;
      if binario_otimizado.exists() { 
         cmd_str = {
            binario_otimizado
            .to_str().unwrap()
         };
      } else {
         cmd_str = {
            binario_debug
            .to_str().unwrap()
         };
      }
      Command::new(cmd_str)
   };
   cmd.arg("varre");
   /* Converte itervalo para string.
    * Adiciona o argumento em sequência. */
   cmd.arg(format!("{}..={}",*i.start(), *i.end()));
   cmd.stdout(Stdio::piped());
   // execuntando ...
   Ok(cmd.spawn()?)
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   use std::env::args;

   #[test]
   fn testeBasicoFI() {
      println!("digite um intervalo: (ínicio)..=(fim)");
      let argumentos = filtra_intervalo(args());
      println!("{:#?}", argumentos);
      // avaliação manual.
      assert!(true);
   }

   #[test]
   fn numCPUsExternPackage() {
      use num_cpus;
      println!("total de CPU's é {}.", num_cpus::get());
      assert_eq!(num_cpus::get(), 4);
   }
}
