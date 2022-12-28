
/*! 
 Cuida especificamente do espaço em 
 disco que os backups ocupam. Este
 código têm como principal função
 eliminar backups redundantes.
*/

use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;


/*

/* verifica backups que são redudantes, e versões 
 * mais recentes de "backups" redundantes, tem seus 
 * caminhos retornados, possívelmente para exclusão. */
fn verificacao_de_redundancia() -> Option<Vec<PathBuf>> { 
   // interando todas entradas do diretório. 
   // nada inicialmente.
   None
} 

// faz várias
fn combinacoes_pares<K:Clone + Hash + Eq>
(array:&Vec<K>) -> Vec<HashSet<K>> {
   /* pilha contendo a primeira transformação,
    * então, as próximas serão desempilhadas
    * tais para o mesmo processo. */
   let mut lista:Vec<HashSet<K>> = Vec::new();

   /* alternar locais na array, levando de
    * 1 à (n-1) espaços entre eles. */
   for k in 1..=array.len()-1 {
      for i in 0..=array.len()-1 {
         /* abandona loop se a indexação ultrapassar
          * intervalo limite da array. */
         if k + i > array.len()-1 
            { continue; }

         // conjunto para ser colocado na lista.
         let mut conjunto:HashSet<K> = HashSet::new();
         conjunto.insert(array[i].clone());
         conjunto.insert(array[k+i].clone());
         
         // adiciona conjunto na lista.
         lista.push(conjunto);   
      }
   }

   // pilha contendo todas combinações...
   return lista;
}

/* dado todas entraadas de um determinado
 * diretório, combiná-lôs de dois-em-dois
 * para que todas possíbilidades sejam criadas. */
fn forma_pares(entradas:ReadDir) -> Vec<Par> {
   /* transformando iterador em array para iteração
    * baseado no índice dela.  */
   let mut array:Vec<PathBuf> = {
      entradas
      .map(|e| e.unwrap().path())
      .collect()
   };
   /* gera todos pares possíveis com os arquivos
    * de tal diretório. */
   let pares:Vec<Par> = {
      // gerando pares ...
      combinacoes_pares(&mut array)
      .iter()
      .map(|conjunto| {
         let mut iterador = conjunto.iter();
         let s1 = iterador.next().unwrap();
         let s2 = iterador.next().unwrap();
         (s1.clone(), s2.clone())
      }).collect()
   };
   // retorna todos arquivos tomados dois-a-dois.
   return pares;
}
*/

fn fatorial(n: usize) -> usize {
   match n {
      0 => 1,
      _ => n * fatorial(n-1)
   }
}

fn permutacoes<X>(todas: &mut HashSet<Vec<X>>, 
k: usize, sequencia: &mut Vec<X>,
conjunto: &mut HashSet<X>) 
where X: Copy + Debug + Hash + Eq {
   for x in conjunto.clone().iter() {
      sequencia.push(*x);
      conjunto.remove(x);

      if k == 1 { 
         println!("{:?}", sequencia); 
         assert!(todas.insert(sequencia.clone()));
      } else { 
         permutacoes(todas, k - 1, sequencia, conjunto); 
      }

      let y = sequencia.pop().unwrap();
      conjunto.insert(y);
   }
}

fn arranjos<U>(mut sequencia: Vec<U>, de: usize) -> HashSet<Vec<U>> 
where U: Copy + Debug + Hash + Eq {
   let total = sequencia.len();
   // restrições invioláveis.
   if de > total
      { panic!("não é possível arranjar objetos numa quantia maior do que seu total!"); }
   else if de == 1
      { panic!("o minímo que se pode fazer é arranjar em pares!"); }
   else if total <= 1 
      { panic!("não aceita arrays vázias ou com apenas um elemento!"); }
   let mut conjunto: HashSet<U>;
   conjunto = HashSet::with_capacity(total);
   // adicionando elementos no conjunto ...
   for e in sequencia.drain(0..)
      { assert!(conjunto.insert(e)); }
   // computando total de anagramas.
   let necessario: usize = {
      let n = total;
      let k = de;
      fatorial(n) / fatorial(n-k)
   };
   // todos arranjos com a devida capacidade.
   let mut todos: HashSet<Vec<U>>;
   todos = HashSet::with_capacity(necessario);
   // computando permutações de k à k ... 
   permutacoes(
      &mut todos, de,
      &mut sequencia,
      &mut conjunto
   );
   // não pode exceder, está tudo contado!
   assert_eq!(todos.len(), necessario);
   // retorno:
   return todos;
}

// apelidando conjunto de sequências ...
fn combinacoes<A: Copy+Debug+Hash+Eq>
(conjunto: HashSet<A>, p: usize) -> HashSet<Vec<A>> {
   let total: usize = {
      let n = conjunto.len();
      let n_p = n-p;
      fatorial(n)/(fatorial(p)*fatorial(n_p))
   };
   // combinações:
   let mut todas: HashSet<Vec<A>>;
   todas = HashSet::with_capacity(total);

   // sequência inicial.
   let mut seq: Vec<_>;
   seq= conjunto.clone().drain().collect();

   // todos arranjos formados.
   let mut todos_arranjos: HashSet<Vec<_>>;
   todos_arranjos = arranjos(seq, p);

   for s in todos_arranjos.drain() {
      println!("{:?}", s);
      todas.insert(s.to_vec());
      if todas.len() > total
         { break; }
   }

   return todas;
}

struct Conjunto<A> where A: Debug + Eq  {
   sequencias: Vec<Vec<A>>,
   // máximo de elementos permitidos.
   capacidade: u8
}

impl Conjunto {



#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;

   #[test]
   fn gera_permutacoes() {
      let mut sequencia: Vec<char>;
      let mut conjunto: HashSet<char>;

      sequencia = Vec::with_capacity(4);
      conjunto = HashSet::with_capacity(26);

      for e in "abcd".chars() 
         { assert!(conjunto.insert(e)); }

      // usando função ...
      permutacoes(
         &mut HashSet::with_capacity(26),
         conjunto.len()-2, 
         &mut sequencia, 
         &mut conjunto
      );

      // avaliação manual.
      assert!(true);
   }

   #[test]
   fn teste_de_fatorial() {
      assert_eq!(fatorial(5), 120);
      assert_eq!(fatorial(3), 6);
      assert_eq!(fatorial(10), 3_628_800);
      assert_eq!(fatorial(1), 1);
      assert_eq!(fatorial(0), 1);
      assert_eq!(fatorial(7), 5_040);
   }

   #[test]
   fn testa_arranjos() {
      let s = vec![9, 99, 999];
      assert_eq!(6, arranjos(s.clone(), 3).len());
      assert_eq!(6, arranjos(s, 2).len());
   }

   use std::iter::FromIterator;
   #[test]
   fn testa_combinacoes() {
      let X: HashSet<char> = HashSet::from_iter("abcde".chars());
      let C = combinacoes(X, 3);
      for c in C.iter() 
         { println!("{:?}", c); }
   }
}
