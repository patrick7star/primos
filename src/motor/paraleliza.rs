/* Gerador de executável que cria várias forks da função que executa
 * a busca contínua por primos dado o intervalo. Você demanda a quantia
 * necessária.
 */

// Biblioteca do Rust:
use std::process::{Stdio, Command};
use std::str::FromStr;
use std::mem::swap;
use std::iter::Iterator as I;
use std::process::Child;
use std::io::Error;
use std::path::Path;

// Apelidos convenientes:
type Processo = Result<Child, Error>;
use super::turbina::Intervalo;


pub fn string_para_range(s: String) -> Intervalo {
   // Decompondo ...
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

   // Formando o intervalo em sí.
   inicio..=fim
}

/* Captura os argumentos e retorna um intervalo, onde será feito à busca 
 * pela quantia de primos. */
pub fn filtra_intervalo(i: impl I<Item=String>) -> Intervalo 
{
   string_para_range(
      // Retira apenas intervalo.
      i.filter(|s| { 
         let p1 = s.contains("..");
         let p2 = s.contains('=');
         /* contém um tipo, ou o outro, ou
          * também a combinação de ambos que 
          * é o caso. */
         p1 || p2
      }).next().unwrap()
   )
}

/* Gera vários processos que processas vários intervalos dados. Gera vários
 * forks chamando esta função quantas vezes achar que for necessário. */
pub fn gera_processo(i: Intervalo) -> Processo {
   // Binários otimizados, e o debug se não houver.
   let binario_otimizado = Path::new("target/release/primos");
   let binario_debug = Path::new("target/debug/primos");
   // Formando o comando a executar futuramente...
   let mut cmd = {
      Command::new(
         if binario_otimizado.exists() 
            { binario_otimizado.to_str().unwrap() } 
         else 
            { binario_debug.to_str().unwrap() }
      )
   };

   cmd.arg("varre");
   /* Converte itervalo para string.
    * Adiciona o argumento em sequência. */
   cmd.arg(format!("{}..={}",*i.start(), *i.end()));
   cmd.stdout(Stdio::piped());
   // execuntando ...
   cmd.spawn()
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
