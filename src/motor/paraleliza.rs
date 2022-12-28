
/* Gerador de executável que cria
 * várias forks da função que executa
 * a busca contínua por primos dado
 * o intervalo. Você demanda a quantia
 * necessária.
 */

// biblioteca do Rust:
use std::process::Command;
use std::env::Args;
use std::str::FromStr;
use std::mem::swap;
// resto do módulo:
use super::turbina::{Intervalo, simultaneadade};


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
pub fn filtra_intervalo(i: Args) -> Intervalo {
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

/* gera vários processos que processas
 * vários intervalos dados. Gera vários
 * forks chamando esta função quantas 
 * vezes achar que for necessário. */
pub fn gera_processo(i: Intervalo) {
   // converte itervalo para string.
   let intervalo_str= format!("{}..={}",*i.start(), *i.end());
   // formando ...
   let mut cmd = Command::new("cargo run");
   cmd.arg("varre");
   cmd.arg(intervalo_str);
   // execuntando ...
   cmd.spawn().unwrap();
}

/* faz uma varredura dado o intervalo. Quando
 * chamado um novo processo, em modo exclusivo
 * para funções internas, chama esta função.*/
pub fn varredura(i: Intervalo) {
   let _dados = simultaneadade(i, 10);
   //despeja(dados).unwrap();
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
}
