
use std::ops::RangeInclusive;
use std::collections::HashSet;
use std::thread::{spawn, JoinHandle};
use crate::motor::primos_faixa;

pub type Primos = HashSet<u64>;
pub type Intervalo = RangeInclusive<u64>;
type Intervalos = Vec<Intervalo>;
type Fios = Vec<JoinHandle<Primos>>;


/* embrulho da função 'primos_faixa', que
 * retorna a array de u64 como um conjunto
 * do mesmo. */
fn varre(intervalo: Intervalo) -> Primos {
   let a = *intervalo.start();
   let b = * intervalo.end();
   let mut array = primos_faixa(a, b);
   let mut conjunto = Primos::with_capacity(array.len());
   for e in array.drain(..)
      { conjunto.insert(e); }
   return conjunto;
}

/* gera vários intervalos disjuntos, dado os
 * limites. */
fn intervalos_de(mut i: u64, f: u64, qtd: u64) -> Intervalos {
   let mut lista = Intervalos::with_capacity(qtd as usize + 1);
   let q = (f-i) / qtd;
   let mut primeiro = false;

   // retira o último para ajuste variádo.
   for _ in 1..=qtd-1 {
      if primeiro
         { lista.push((i+1)..=(i+q)); }
      else 
         { lista.push(i..=(i+q)); primeiro = true; }
      i += q;
   }
   lista.push((i+1)..=f);
   return lista;
}

/* pega o intervalo dado, faz repartições, e
 * busca cada fatia usando da técnica de
 * concorrência, com isso fica em média duas
 * vezes mais veloz do que a técnica antiga. 
 * Os parâmetros são o intervalo repartido 'i',
 * e 'nt' é a quantia de threads utilizadas,
 * mais de quartoze, o efeito é basicamente o 
 * mesmo ou menor, então modere.
 */
pub fn simultaneadade(i: Intervalo, nt: usize) -> Primos {
   let a = *i.start();
   let b = *i.end();
   let mut conjunto = Primos::with_capacity(1000);
   let mut fios = Fios::with_capacity(nt);

   // criando threads ...
   let d = nt as u64;
   for intervalo in intervalos_de(a, b, d).drain(..) {
      let fio = spawn(|| { varre(intervalo) });
      fios.push(fio);
   }

   // aguardando todas terminar ...
   for f in fios.drain(..) {
      match f.join() {
         Ok(set) => 
            { conjunto.extend(set); }
         Err(_) =>
            { panic!("não funcionou para tal 'fio'."); }
      };
   }
   return conjunto;
}

/* O mesmo que o intervalo_de, no entanto, ele
 * faz com um intervalo-inclusívo de 64-bits
 * dado, e o outro parâmetro é quantas vezes
 * reparti-lô. */
pub fn divide_intervalo(i: Intervalo, qtd: usize) -> Intervalos {
   /* retira valores extremos do intervalo 
    * inclusívo, para chamar função que já
    * faz isso. */
   let a = *i.start();
   let b = *i.end();
   // só reutiliza a função acima.
   intervalos_de(a, b, qtd as u64)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   extern crate utilitarios;
   use utilitarios::tabela_visualizacao::{Tabela, Coluna};
   use crate::motor::temporizador::Cronometro;

   #[test]
   fn IntervalosDe() {
      for i in intervalos_de(2, 1000, 4) {
         println!("{:#?}", i);
      }
      print!("\n");
      for i in intervalos_de(100, 1000, 6) {
         println!("{:#?}", i);
      }
      // avaliação manual.
      assert!(true);
   }

   #[test]
   fn ComparacaoVelocidades() {
      let mut cronometro = Cronometro::novo();
      let total: u64 = 27_000;

      let l1 = varre(2..=total);
      let t1 = dbg!(cronometro.marca());
      // reutilizando mesmo objeto.
      cronometro.reseta();
      let l2 = simultaneadade(2..=total, 7);
      let t2 = dbg!(cronometro.marca());

      // mesmos resultados:
      assert_eq!(l1, l2);
      assert_eq!(l1.len(), l2.len());
      // um terço do tempo ao menos.
      let razao = t1.as_secs_f32() / t2.as_secs_f32();
      assert!(dbg!(razao) >= 1.0)
   }

   #[test]
   fn aumentoDeThreads() {
      // arrays para as colunas:
      let mut razoes: Vec<f32> = Vec::with_capacity(20);
      let mut n_threads: Vec<usize> = Vec::with_capacity(20);
      let mut cronometro = Cronometro::novo();

      let total: u64 = 15_000;
      let _l = varre(2..=total);
      let t = cronometro.marca();
      razoes.push(1.0);
      n_threads.push(1);

      for n in 2..=30 {
         cronometro.reseta();
         let _l = simultaneadade(2..=total, n);
         let T = cronometro.marca();
         razoes.push(t.as_secs_f32() / T.as_secs_f32());
         n_threads.push(n);
      }

      // visualizando informação.
      let fios = Coluna::nova("qtd. de threads", n_threads);
      let eficiencia = Coluna::nova("eficiência", razoes);
      let mut tabela = Tabela::nova(true);

      tabela.adiciona(fios);
      tabela.adiciona(eficiencia);
      println!("{}", tabela);

      // avaliação manual:
      assert!(true);
   }
}
