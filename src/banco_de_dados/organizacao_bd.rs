
/* faz a organização do BD, que geralmente é 
 * gravado com o préfixo "backup", mais o tempo
 * de sistema que está sendo criado na frente.
 * Vamos continuar com o préfixo, porém mudar 
 * o selo do sistema, e colocar um número 
 * romano no local. */

use std::fs::{read_dir, DirEntry, ReadDir};
use std::time::Duration;
use utilitarios::romanos::decimal_para_romano;


fn decorrido(entrada: &DirEntry) -> Duration {
   let mt = entrada.metadata().unwrap();
   return {
      mt.created().unwrap()
      .elapsed()
      .unwrap()
   };
}

/* ordena lista de acordo com a data 
 * de criação. Em ordem crescente, ou seja,
 * os mais recentes ficam na esquerda da 
 * array. */
fn ordena(lista: ReadDir) -> Vec<DirEntry> {
   let mut array: Vec<DirEntry> = Vec::new();
   
   // inserindo já ordenando(insert-sort).
   for entrada in lista {
      let e = entrada.unwrap();
      /* se não há elementos, apenas adiciona
       * e reseta o 'loop'. */
      if array.len() == 0
         { array.push(e); continue; }
      let t1 = decorrido(&e);
      let mut indice = 0;
      for (i, x) in array.iter().enumerate() {
         let t2 = decorrido(&x);
         // atualiza o índice atual.
         indice = i;
         if t1 < t2
            { break; }
      }
      // adiciona no índice parado.
      array.insert(indice, e);
   }
   return array;
}

fn nome(entrada: &DirEntry) -> String { 
   let caminho = entrada.path();
   caminho.as_path()
   .file_name().unwrap()
   .to_str().unwrap() 
   .to_string()
}

pub fn renomea(mut lista_ordenada: Vec<DirEntry>) {
   let mut tamanho = lista_ordenada.len();
   for entrada in lista_ordenada.drain(0..) {
      let numero_romano = decimal_para_romano(tamanho as u16);
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

   /* tamanho em bytes do arquivo. */
   fn size(entrada: &DirEntry) -> u64 
      { entrada.metadata().unwrap().len() }

   #[test]
   fn visualizaListagem() {
      let caminho = "data/backups";
      let entradas = read_dir(caminho).unwrap();
      let lista = ordena(entradas);

      for (i, e) in lista.iter().enumerate() {
         println!(
            "{0:>3.0} ==> '{2}' ({1}/{3})",
            i, tamanho(size(e) as u64, true), 
            nome(e), Tempo(tempo(e), true)
         );
      }
      assert!(true);
   }

   #[test]
   fn prototipoDeRenomeacao() {
      let caminho = "data/backups";
      let entradas = read_dir(caminho).unwrap();
      let lista = ordena(entradas);
      renomea(lista); 
   }
}
