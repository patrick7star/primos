/*!   Por motivos de organização, todos apelidos e constantes usados neste
 * módulo estarão aqui, e definido aqui. Aqueles que não, serão de forma
 * imediata, transferidos para aqui. Mesmos estruturas de bibliotecas que 
 * são externas, logo foram importadas, porém renomeadas ficarão aqui. 
 */
extern crate utilitarios;

use std::time::{Instant, Duration};
use std::ops::{RangeInclusive};
use std::collections::{HashSet};
use std::thread::{JoinHandle};
use std::process::{Child};
use std::io::{Error};
// Funções e estruturas externas:
use utilitarios::barra_de_progresso::ProgressoPercentual;
use utilitarios::barra_de_progresso::ProgressoTemporal;

// Abreviados para algumas estrutura de dados definidas:
pub type PP = ProgressoPercentual;
pub type PT = ProgressoTemporal;
pub type ListaDePrimos = Vec<u64>;
/* Definindo um novo tipo de dados para a tupla que representa a colêtanea 
 * produzida pela varredura. */
pub type Dados = (ListaDePrimos, u64, u64, u64);
/* Apelidos de pacotes de dados que são retornados ou recebidos nas funções
 * abaixo: */
pub type Primos = HashSet<u64>;
pub type Intervalo = RangeInclusive<u64>;
pub type Intervalos = Vec<Intervalo>;
pub type Fios = Vec<JoinHandle<Primos>>;
pub type Processo = Result<Child, Error>;
/* Uma referência da tupla que registra dados, que pode ser solicitada 
 * ou não. */
pub type IDP = InfoDeProcessamento;
pub type MonitorProcessamento<'x> = Option<&'x mut IDP>;

// Todas constantes definidas no módulo e seus submódulos:
pub const NOME_DO_PIPE: &str = "tubulacao\0";
#[allow(dead_code, non_upper_case_globals)]
pub const Okay: i32 = 0;
#[allow(dead_code, non_upper_case_globals)]
pub const Fail: i32 = -1;
pub const In: usize = 0;
pub const Out: usize = 1;

/* Estruturas para capturar metadados importantes durante o processamento
 * de Primos.*/
#[derive(Debug)]
pub struct InfoDeProcessamento {
   // Quanto da faixa dada já foi processado.
   pub percentual: f64,

   // Total de primos encontrados.
   pub quantia: usize,

   // Tempo médio para encontrar um novo primo.
   pub tempo: Duration,
   pub cronometro: Instant
}

use std::mem::{size_of, transmute, zeroed};

fn extrai_bytes<T: Sized>(data: &T) -> Vec<u8>
{
/* Pega os bytes do tipo da referência passada, e coloca numa array 
 * dinâmica. Na ordem que são lidos, ou seja, esquerda-direita. */
   let size = size_of::<T>();
   let mut array = Vec::<u8>::with_capacity(size);
   let ptr_of_t = unsafe { transmute::<&T, *const T>(data) };
   let ptr = ptr_of_t as *const u8;

   for k in 1..=size 
      { array.push(unsafe { *ptr.add(k - 1) }); }
   array
}

impl InfoDeProcessamento 
{
   pub fn nova() -> Self {
      let percentual = 0.0_f64;
      let quantia: usize = 0;
      let tempo = Duration::from_secs(0);
      let cronometro = Instant::now();

      InfoDeProcessamento { percentual, quantia, tempo, cronometro }
   }

   // A serialização em bytes, seguirá a ordem que foi codificada.
   pub fn serializa(&self) -> Vec<u8> 
      { extrai_bytes(self) }
   
   pub fn deserializa(bytes: &[u8]) -> Self {
      let sz = size_of::<Self>();
      let ptr_bytes: *const u8;
      let ptr_self: *const Self;  
      let mut objeto: Self = unsafe {zeroed()};
      let ptr_obj: *mut Self;

      unsafe { 
         ptr_obj = transmute::<&Self, *mut Self>(&mut objeto);
         ptr_bytes = bytes.as_ptr();
         ptr_self = transmute::<*const u8, *const Self>(ptr_bytes);
         ptr_self.copy_to_nonoverlapping(ptr_obj, sz);
      }

      // Tem que ser do meio tamanho que o tipo dado!
      assert_eq!(bytes.len(), sz);
      // Dereferenciando já no tipo que o ponteiro se transforma.
      objeto
   }
}

impl PartialEq for InfoDeProcessamento {
   fn eq(&self, other: &Self) -> bool {
      (self.percentual - other.percentual).abs() < 0.05 &&
      (self.quantia == other.quantia) &&
      (self.tempo == other.tempo)
   }
}


#[cfg(test)]
mod tests {
   use super::{
      Instant, Duration, size_of, InfoDeProcessamento, extrai_bytes,
      transmute
   };

   #[test]
   fn tamanho_em_bytes_de_algumas_estruturas() {
      let mut x = InfoDeProcessamento::nova();
      let bytes_a: *const Duration;
      let bytes_b: *const Instant;
      
      unsafe {
         bytes_a = transmute::<&Duration, *const Duration>(&x.tempo);
         bytes_b = transmute::<&Instant, *const Instant>(&x.cronometro);
      }

      // Fixando alguns valore para ver seus bytes na array de bytes.
      x.quantia = 152;
      x.tempo = x.cronometro.elapsed();
      x.percentual = 0.36;

      println!(
         "Duration: {} bytes\nInstant: {} bytes\nf64: {} bytes
         \rInfoDeProcessamento: {} bytes",
         size_of::<Duration>(), size_of::<Instant>(),
         size_of::<f64>(), size_of::<InfoDeProcessamento>()
      );

      println!("{:p}\n{:p}", bytes_a, bytes_b);
      println!(
         "Bytes de Duration: {:?}\nBytes de Instant: {:?}
         \rBytes do usize: {:?}\nBytes do f64: {:?}
         \rBytes InfoDeProcessamento: {:?}",
         extrai_bytes(&x.tempo), extrai_bytes(&x.cronometro),
         extrai_bytes(&x.quantia), extrai_bytes(&x.percentual),
         extrai_bytes(&x)
      );
   }

   #[test]
   fn serializacao_de_InfoDeProcessamento() {
      let x = InfoDeProcessamento::nova();
      let bytes_x = x.serializa();
      let y = InfoDeProcessamento::deserializa(&bytes_x[..]);

      assert_eq!(y, x);
   }
}
