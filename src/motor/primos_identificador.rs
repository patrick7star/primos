

/*! 
 Modo de identificar primos de forma 
 bem mais eficiente, será acoplado
 ao código se for demonstrado assim
 ser. Não é apenas um simples algoritmo,
 mais toda uma cadeia que serão adicionados
 ao longo do tempo. Todos devidamente
 testados.
*/


/** 
mesmo que o algoritmo usado até
aqui, porém o tempo de todas buscas
por menos do que a metade do tempo. 

retorna 'verdadeiro' ou 'falso' se o 
número é primo. 

# Exemplos:
```
let cronometro = Instant::now();
assert!(e_primo(199_382));
let t = cronometro.elapsed()

let cronometro = Instant::now();
assert!(e_primoI(199_382));
let T = cronometro.elapsed()

assert!(T < t / 2)

let nao_primo = 58;
assert_eq!(false, e_primoI(nao_primo));

```
*/
#[allow(non_snake_case)]
#[allow(clippy::needless_return)]
pub fn e_primoI(n: u64) -> bool {
   // se o valor for 1, já retorna como não-primo.
   if n == 1 || n == 0 
      { return false; }
    // o mesmo para 2, 3, 5 e 7 que são primos óbvios.
    else if n == 2 || n == 3 || n == 5 || n == 7
      { return true; }
   /* verifica até três números à frente do
   * limite real, pois ainda não sou confidente
   * na conjectura, já que não vi o teorema
   * formal. */
   let limite = (n as f64).sqrt();
   let limite: u64 = (limite as u64) + 3;
   // testa a divisíbilidade de 1 à n.
   for d in 2..=limite {
      // se d for divisível, contabiliza-lô.
      if n % d == 0 
         { return false; }
   }
   /* caso, ao pesquisar entre todos valores de 
   * 1 à n a quantia de divisores for apenas 2, 
   * então o número é primo. */
   return true;
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   use crate::motor::e_primo;
   use crate::motor::temporizador::Cronometro;

   #[test]
   fn ambasMesmosAchados() {
      let mut c1 = 0;
      for n in 1u64..=20_000 { 
         if e_primo(n)
            { c1 += 1; }
      }
      let mut c2 = 0;
      for n in 1u64..=20_000 { 
         if e_primoI(n)
            { c2+= 1; }
      }
      assert_eq!(c1, c2);
   }

   #[test]
   fn metadeDoTempo() {
      const TOTAL: u64 = 45_000;
      let mut c = Cronometro::novo();
      for n in 1u64..=TOTAL
         { e_primo(n); }
      let m = dbg!(c.marca());
      c.reseta();
      for n in 1u64..=TOTAL
         { e_primoI(n); }
      let M = dbg!(c.marca());
      // têm que ser menor obviamente.
      assert!(M < m);
      /* só que só menor é pouco, têm 
       * que ser no máximo a metade
       * do outro.
       */
      assert!(M < m / 2);
   }

   #[test]
   fn analiseEficiencia() {
      let mut C = Cronometro::novo();
      let Maximos = [
         1_000u64, 2_000, 5_000, 20_000,
         25_000, 30_000, 52_612, 104_081
      ];
      for max in Maximos {
         C.reseta();
         for n in 1u64..=max
            { e_primoI(n); }
         let t = C.marca();
         println!("{} num's / {:#?}", max, t);
      }
   }

   #[test]
   #[ignore="último trecho consome quase todo tempo!"]
   fn mesmosResultados() {
      // primos por métodos antigos.
      let mut pMA: Vec<u64> = Vec::with_capacity(11_000);
      // primos pelo método mais eficiente.
      let mut pME: Vec<u64> = Vec::with_capacity(11_000);
      // varredura normal.
      for n in 1u64..=10_000 {
         if e_primoI(n)
            { pME.push(n); }
         if e_primo(n)
            { pMA.push(n); }
      }
      assert_eq!(pME, pMA);
      // intervalo não tão certinho.
      for n in 15_921..=27_001u64 {
         if e_primoI(n)
            { pME.push(n); }
         if e_primo(n)
            { pMA.push(n); }
      }
      assert_eq!(dbg!(pME.len()), dbg!(pMA.len()));
      assert_eq!(pME, pMA);
      // um de cada vez, mesmos intervalos.
      for n in 307_991..=418_552u64 {
         if e_primoI(n)
            { pME.push(n); }
      }
      assert_ne!(dbg!(pME.len()), pMA.len());
      for n in 307_991..=418_552_u64 {
         if e_primo(n)
            { pMA.push(n); }
      }
      assert_eq!(pME, pMA);
   }
}
