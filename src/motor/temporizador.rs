

/* Temporizador básico para ajudar
 * em execução determinadas de 
 * várias partes no código.
 */

use std::time::{Instant, Duration};
use std::collections::VecDeque;
use std::cmp::{Ordering, PartialOrd, PartialEq};
use std::fmt::{Debug, Formatter, Result as ResultFMT};

/* contador regressivo. */
pub struct Temporizador {
   // cronômetro que conta até "limite imposto".
   cronometro: Instant,
   /* ínicio de onde parte a contagem
    * regressiva estabelicida na 
    * geração da instância. */
   inicio: Duration,
}

impl Temporizador {
   // método construtor.
   pub fn novo(de: Duration) -> Self {
      return Self { 
         cronometro: Instant::now(),
         inicio: de,
      };
   }
   /* percentual da contagem, partido do 
    * principio dado no começo.
    * 100% não passou nada, já 0% é considerado
    * totalmente esgotado. */
   #[allow(dead_code)]
   pub fn percentual(&self) -> f32 {
      if self.esgotado()
         { return 0.0f32; }
      let t = self.cronometro.elapsed().as_nanos() as f32;
      let total = self.inicio.as_nanos() as f32;
      return 1.0 - (t / total);
   }
   /* verifica se o 'Temporizador' já 
    * chegou ao fim, ou seja, se já está
    * zerado. */
   pub fn esgotado(&self) -> bool { 
      let decorrido = self.cronometro.elapsed();
      return decorrido > self.inicio;
   }
   /* lembrando que 'timers' tem contagem
    * reversas, ou sejá não partem do zero,
    * mas sim do final, e vão diminuindo.
    */
   pub fn contagem(&self) -> Duration {
      if self.esgotado() 
         { return Duration::new(0, 0); }
      let decorrido = self.cronometro.elapsed();
      match self.inicio.checked_sub(decorrido) {
         Some(tempo) => tempo,
         None => 
            { panic!("erro complicado na cronometragem"); }
      }
   }
}

type Marcos = VecDeque<Duration>;
/* contador continuo. */
pub struct Cronometro(Instant, Marcos);

/* máximo de 'marcos' que ele grava 
 * por instâncias. */
const LIMITE: usize = 10;

impl Cronometro {
   // método construtor.
   pub fn novo() -> Cronometro 
      { Self(Instant::now(), Marcos::with_capacity(LIMITE)) }
   /* reinicia o cronômetro, assim apagando
    * todos os marcos. */
   pub fn reseta(&mut self) { 
      self.0 = Instant::now(); 
      self.1.clear();
   }
   /* marca o estado atual do contador. */
   #[allow(dead_code)]
   pub fn marca(&mut self) -> Duration {
      let decorrido = self.0.elapsed();
      if self.1.capacity() == self.1.len()
         { self.1.pop_front().unwrap(); }
      self.1.push_back(decorrido.clone());
      return decorrido;
   }
   /* decorrido desde o último marco
    * registrado. Se não hovuer algum,
    * simplesemente retorna marco.*/
   #[allow(dead_code)]
   pub fn delta(&self) -> Duration {
      if self.1.is_empty()
         { return self.0.elapsed(); }
      let ultimo = self.1.len()-1;
      let registro = self.1[ultimo];
      let atual = self.0.elapsed();
      match atual.checked_sub(registro) {
         Some(duracao) => duracao,
         None =>
            { panic!("Erro inconciliável"); }
      }
   }
}

impl PartialOrd<Duration> for Cronometro {
   fn partial_cmp(&self, _: &Duration) -> Option<Ordering>
      { todo!(); }
   fn lt(&self, other: &Duration) -> bool 
      { self.0.elapsed() < *other }
   fn gt(&self, other: &Duration) -> bool 
      { self.0.elapsed() > *other }
   /* usa os métodos acimas, a precisão aqui
    * é insignificante na lógica. */
   fn le(&self, direita: &Duration) -> bool
      { self < direita }
   fn ge(&self, direita: &Duration) -> bool
      { self > direita }
}

impl PartialEq<Duration> for Cronometro {
   /* analisa ambos, numa margem de nanos
    * segundos de erro. A igualdade só funciona
    * bem na escala de dezenas de milisegundos. */
   fn eq(&self, direita: &Duration) -> bool {
      let a = self.0.elapsed().as_nanos() as f32;
      let b = direita.as_nanos() as f32;
      /* com uma margem percentual, de menos
       * de 5%, será dado como igual. */
      let p: f32 = {
         if a > b
            { b / a }
         else if a == b
            { 1.0 }
         else
            { a / b}
      };
      //println!("margem: {}%", (1.0-p) * 100.0);
      (1.0-p) < 0.05 
   }
   /* a negação da setença acima. */
   fn ne(&self, direita: &Duration) -> bool 
      { !self.eq(direita) }
}

impl Debug for Cronometro {
  fn fmt(&self, f: &mut Formatter<'_>) -> ResultFMT 
   { f.write_fmt(format_args!("~ {:3.0?}", self.0.elapsed())) }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   use std::{thread};

   #[test]
   /* testando a precisa de igualdade que 
    * se têm em cada escala de tempo. Presuposto
    * é que ela se perde apenas na escala
    * dos nanosegundos. O que mostra-se na 
    * realidade é quebra no "fimzinho" da 
    * escala dos 'mills', portanto, na
    * escala dos microsegundos. */
   fn igualdadeCronometro() {
      let c = Cronometro::novo();
      // em segundos.
      thread::sleep(Duration::from_secs(3));
      assert_eq!(c, Duration::from_secs(3));
      // em milisegundos.
      for V in [801, 301, 87, 20, 7] {
         let decorrido = Duration::from_millis(V);
         let c = Cronometro::novo();
         thread::sleep(decorrido.clone());
         assert_eq!(c, decorrido); 
      }
      // em microsegundos.
      let decorrido = Duration::from_micros(23);
      let decorrido_copia = decorrido.clone();
      let c = Cronometro::novo();
      thread::sleep(decorrido_copia);
      assert_ne!(c, decorrido);
      // em nanosegundos.
      let decorrido = Duration::from_nanos(17);
      let decorrido_copia = decorrido.clone();
      let c = Cronometro::novo();
      thread::sleep(decorrido_copia);
      assert_ne!(c, decorrido);
   }

   #[test]
   fn todasComparacoes() {
      let c = Cronometro::novo();
      thread::sleep(Duration::from_millis(20));
      assert!(
         c > Duration::from_millis(19) &&
         c < Duration::from_millis(21)
      );
      let c = Cronometro::novo();
      thread::sleep(Duration::from_secs_f32(1.3));
      assert!(
         c > Duration::from_secs_f32(1.2) &&
         c < Duration::from_secs_f32(1.4)
      );
   }
}
