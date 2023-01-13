

/** Módulo cuida específicamente das 
 inserções feitas, no caso todo trabalho
 feito no arquivo como nomo/caminho REGISTROS_BD
 (como pode ser visto, a constante abaixo). 
 Atualiza/ou grava a informação que já existe, 
 e à lê também.
 */

// biblioteca padrão:
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::io::{Write, Read};

// puxando do módulo.
use super::{carrega_bd, DIR};

// local do arquivo que será lido e atualizado.
const REGISTROS_BD:&'static str = "ultima_insercao.dat";

/* obtem o índice de inserção, que foi feito 
 * por último no BD. */
pub fn ultima_insercao_feita() -> Option<u32> {
   /* tenta abrir o banco de dados, se não houver 
    * algum, criar e colocar o último índice de
    * inserção nele. */
   let mut caminho = PathBuf::new();
   caminho.push(DIR);
   caminho.push(REGISTROS_BD);
   let arquivo = {
      OpenOptions::new()
      .read(true)
      .open(caminho.as_path())
   };

   return match arquivo {
      Ok(mut file) => {
         // lendo os únicos 4 bytes dentro do arquivo.
         let mut bytes = [u8::MAX; 4];
         file.read_exact(&mut bytes).unwrap();
         let valor = u32::from_le_bytes(bytes);
         // fechando BD...
         drop(file);
         Some(valor)
      } Err(_) =>  { 
         /* se não tem arquivo do BD, então não apenas
          * criar um como, carregar todo BD, para que
          * se tenha o índice de inserção. */
         let todos_dados = carrega_bd().unwrap();
         let tamanho: u32 = todos_dados.len() as u32;
         let bytes:[u8; 4] = tamanho.to_le_bytes();
         // abrindo arquivo para escrita.
         let arquivo_uif = {
            OpenOptions::new()
            .create(true)
            .write(true)
            .open(caminho.as_path())
         };
         match arquivo_uif {
            Ok(mut arquivo) => 
               { arquivo.write(&bytes[..]).unwrap() },
            Err(_) => { 
               let mensagem: &'static str = concat!(
                  "erro ao escrever dados no arquivo ",
                  "\"última_inserção.dat\""
               );
               panic!("{}", mensagem);
            }
         };
         /* como acabou de carregar no BD, ele
          * retorna "null". */
         None
      }
   };
}

/* apenas faz isso se houver trinta ou mais inserções
 * a mais ou, o tamanho atual do BD for 30% superior
 */
pub fn atualiza_indice_de_insercao(indice:u32) {   
   /* abre arquivo para escrita de dados, se 
    * não houver um, então o cria. */
   let arquivo = {
      OpenOptions::new()
      .write(true)
      .create(true)
      .open(Path::new(DIR).join(REGISTROS_BD))
   };
   match arquivo {
      Ok(mut file) => {
         // lendo os únicos 4 bytes dentro do arquivo.
         let bytes = indice.to_le_bytes();
         file.write(&bytes[..]).unwrap()
      },
      Err(_) => 
         { panic!("não conseguiu atualizar o índice!"); }
   };
}
