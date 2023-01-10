
/*! Serializa o conjunto de dados
 dado pelo pela saída de dados
 principal.
*/


use crate::motor::Primos;
use std::io::{self, StdoutLock as SL, Write};
use std::mem;

/* auxiliando no desenvolvimento da 
 * aplicação. */
#[allow(dead_code)]
fn impressao_debug(primo: u64, ordem: usize, total: usize) {
   if ordem % 15 != 0 { 
      if ordem < total-1
         { print!("{}, ", primo); }
      else
         // quebra-de-linha final.
         { println!("{}", primo); }
   } else
      { println!(""); }
}

/* pega o número de 64-bits e cospes seus 
 * 8 bytes, sem espaço, sem nada, para 
 * o próximo. São despejados no output via
 * pelo modo BigEnding. */
fn despeja_inteiro_64bits(numero: u64, saida: &mut SL) {
   let bytes = numero.to_be_bytes();
   saida.write(&bytes[..]).unwrap(); 
}

/** 
 pega o conjunto de primos, e despeja todos
 os bytes que forma cada, pela saída padrão
 do dispotivo.
*/
pub fn despeja_bytes(conjunto: Primos) {
   let mut saida = io::stdout().lock();
   for p in conjunto.iter() 
      //{ impressao_debug(*p, q, qtd); }
      { despeja_inteiro_64bits(*p, &mut saida); }
   mem::drop(saida);
}

type Bytes = Vec<u8>;
fn parse_bytes(mut array: Bytes) -> Vec<u64> {
   let mut a: [u8; 8] = [0; 8];
   let mut lista: Vec<u64> = Vec::new();

   let mut contador: usize = 0;
   for byte in array.drain(..) {
      if contador == 7 {
         lista.push(u64::from_be_bytes(a));
         contador = 0;
      } else 
         { a[contador] = byte; }
      contador += 1;
   }

   return lista;
}

use std::process::Child;
use std::io::Read;
/* transforma array de bytes lida em
 * inteiros positivos de 64-bits, que 
 * são provavelmente os números primos.
 */
pub fn colhe_resultado(sp: &mut Child) -> Vec<u64> {
   let mut bytes: Vec<u8>; 
   bytes = Vec::with_capacity(10_000);  
   let mut leitor = sp.stdout.take().unwrap();
   leitor.read_to_end(&mut bytes).unwrap();
   return parse_bytes(bytes);
}
