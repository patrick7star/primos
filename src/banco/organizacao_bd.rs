
/* faz a organização do BD, que geralmente é 
 * gravado com o préfixo "backup", mais o tempo
 * de sistema que está sendo criado na frente.
 * Vamos continuar com o préfixo, porém mudar 
 * o selo do sistema, e colocar um número 
 * romano no local. */

use std::fs::{DirEntry, ReadDir};
use std::time::Duration;
use utilitarios::romanos::decimal_para_romano;

// facilita codificação:
type Entradas = Vec<DirEntry>;

/* extrai a duração da "entrada do diretório"
 * dada, referente a sua data de criação. */
fn decorrido(entrada: &DirEntry) -> Duration {
   let mt = entrada.metadata().unwrap();
   return {
      mt.created().unwrap()
      .elapsed()
      .unwrap()
   };
}

/* tamanho da entrada, se for um arquivo. */
fn size(e: &DirEntry) -> u64 
   { e.metadata().unwrap().len() }


enum Ordenacao { Tamanho, Tempo }
/* ordena lista de acordo com a data 
 * de criação. Em ordem crescente, ou seja,
 * os mais recentes ficam na esquerda da 
 * array. */
fn ordena(lista: ReadDir, tipo: Ordenacao) -> Entradas {
   let mut array = Entradas::new();

   // inserindo já ordenando(insert-sort).
   for entrada in lista {
      let e = entrada.unwrap();
      /* se não há elementos, apenas adiciona
       * e reseta o 'loop'. */
      if array.len() == 0
         { array.push(e); continue; }
      let t1 = decorrido(&e);
      let s1 = size(&e);
      let mut indice = 0;
      for (i, x) in array.iter().enumerate() {
         let t2 = decorrido(&x);
         let s2 = size(&x);
         // atualiza o índice atual.
         indice = i;
         match tipo {
            Ordenacao::Tempo => {
               if t1 < t2
                  { break; }
            } Ordenacao::Tamanho => {
               if s1 < s2
                  { break; }
            }
         };
      }
      // adiciona no índice parado.
      array.insert(indice, e);
   }
   return array;
}

// extrai nome da entrada que foi referênciada.
fn nome(entrada: &DirEntry) -> String { 
   entrada.path()
   .as_path()
   .file_name().unwrap()
   .to_str().unwrap()
   .to_string() 
}

/* renomea 'Entradas' dadas para um novo
 * formato de identificação, que leva em
 * conta a contagem romana. Aceita basicamente
 * a ordem na 'array', então, não segue 
 * critério de "tempo de criação" ou "tamanho"
 * tal ordenação, apenas segue a ordem da array. */
pub fn renomea(mut lista_ordenada: Entradas) {
   let mut tamanho = lista_ordenada.len();
   for entrada in lista_ordenada.drain(0..) {
      let t = tamanho as u16;
      let numero_romano = decimal_para_romano(t);
      let antigo = nome(&entrada);
      let novo_nome = format!("backup.{}.zip", numero_romano);
      println!(
         "{} >>> {:#?}", 
         antigo, 
         novo_nome.to_lowercase()
      );
      tamanho -= 1;
   }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   use std::fs::read_dir;
   use utilitarios::legivel::{tamanho, tempo as Tempo};

   /* tempo total desde sua criação. */
   fn tempo(entrada: &DirEntry) -> u64{
      let mt = entrada.metadata().unwrap();
      return {
         mt.created().unwrap()
         .elapsed()
         .unwrap().as_secs()
      };
   }


   #[test]
   fn visualizaListagem() {
      let caminho = "data/backups";
      let entradas = read_dir(caminho).unwrap();
      let lista = ordena(entradas, Ordenacao::Tamanho);

      for (i, e) in lista.iter().enumerate() {
         println!(
            "{0:>3.0}º ==> '{2}' ({1}/{3})",
            i+1, tamanho(size(e) as u64, true), 
            nome(e), Tempo(tempo(e), true)
         );
      }
      assert!(true);
   }

   #[test]
   fn prototipoDeRenomeacao() {
      let caminho = "data/backups";
      let entradas = read_dir(caminho).unwrap();
      let lista = ordena(entradas, Ordenacao::Tempo);
      renomea(lista); 
   }
}
