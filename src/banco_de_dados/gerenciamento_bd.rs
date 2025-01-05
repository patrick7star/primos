/* 
 * Já que o banco de dados que armazena em binários
 * está pronto, vamos copiar o em 'txt' para ele, fazendo
 * as devidas conversões.
 */

// biblioteca padrão do Rust.
use std::{
   path::{PathBuf},
   time::{SystemTime, UNIX_EPOCH},
   fs::{create_dir},
   process::Command,
   env::temp_dir,
};
// próprio caixote.
use crate::{computa_caminho};

// nome do atual BD.
const CAMINHO_BD:&str = "data/banco_de_dados.dat";
const CAMINHO_UI:&str = "data/ultima_insercao.dat";
const CAMINHO_REGISTROS:&str = "data/registros.dat";
// diretório dos backup's.
const TODOS_BACKUPS:&str = "data/backups/";
const NOME_ORIGINAL_ARQ:&str = "backup_bd";

// Computa um 'nome+ID' para dá ao archive criado.
#[allow(clippy::needless_return)]
fn cria_nome_id() -> PathBuf {
   let tempo_id: u128 = {
      SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_millis()
   };

   let nome_arquivo: String = format!(
      "{}_{}.zip",
      NOME_ORIGINAL_ARQ,
      tempo_id
   );

   let mut caminho = PathBuf::new();
   caminho.push(temp_dir());
   caminho.push(nome_arquivo);

   return caminho;
}

/** 
 Realiza um novo backup do BD, ele salva tanto o aglomerado de dados 
 gerados, como o atalho para acessar o BD de forma mais específica.  
*/
pub fn realiza_backup_bd() { 
   // transformando slice-strings em 'PathBuf'.
   let caminho_bd = computa_caminho(CAMINHO_BD);
   let caminho_registros = computa_caminho(CAMINHO_REGISTROS);
   let caminho_ui = computa_caminho(CAMINHO_UI);
   let todos_backups = computa_caminho(TODOS_BACKUPS);

   // criando diretório, se necessário...
   match create_dir(todos_backups.clone()) {
      Ok(_) => (),
      Err(_) =>
         { println!("caminho já existe."); }
   };

   // comando de "compactação".
   let mut zipa_bd = Command::new("zip");
   let nome_zip = cria_nome_id();
   zipa_bd.args([
      nome_zip.to_str().unwrap(),
      caminho_bd.to_str().unwrap(),
      caminho_registros.to_str().unwrap(),
      caminho_ui.to_str().unwrap()
   ]);
   
   // comando de "movimentação". 
   let mut move_zipados = Command::new("mv");
   move_zipados.arg("-v");
   move_zipados.arg(nome_zip.to_str().unwrap());
   move_zipados.arg(todos_backups.to_str().unwrap());

   // compactando os arquivos de registros.
   match zipa_bd.spawn() {
      Ok(mut processo) => 
         { processo.wait().unwrap(); }
      Err(_) => 
         { panic!("não foi possível compactar!"); }
   };
   // então move tais archives.
   move_zipados.spawn().unwrap();
}


#[cfg(test)]
mod tests {
   extern crate utilitarios;

   use super::*;
   use std::{thread::sleep, time::Duration, fs::read_dir};

   /* função realiza cinco backups seguidos
    * de intervalos aleatórios. Como não há
    * alteração entre um e outro, então não
    * realizar todos os backups, apenas o
    * primeiro. */
   #[test]
   #[ignore="altera atual banco de backups"]
   fn tenta_realizar_cinco_backups() {
      // quantia de backups anterior.
      let total: usize = {
         let path = super::TODOS_BACKUPS;
         read_dir(path)
         .unwrap()
         .count()
      };

      for p in 1..=5 { 
         realiza_backup_bd();
         println!("{}º backup realizado com sucesso.", p);
         sleep(Duration::from_secs(3));
      }

      // nova quantia de backups
      let novo_total: usize = {
         let path = super::TODOS_BACKUPS;
         read_dir(path)
         .unwrap()
         .count()
      };

      assert_eq!(total, novo_total-5);
   }
}
