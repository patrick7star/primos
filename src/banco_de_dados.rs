
/**! 
 alternativa ao banco de dados existente
 que guarda a informação de forma seriada, porém
 em binário.
*/


// biblioteca padrão do Rust:
use std::io::{Write, Read, Error};
use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::path::{Path};

// resto do módulo:
mod gerenciamento_bd;
mod organizacao_bd;
mod deletador;
mod grande_inversao;
mod ultima_insercao;
pub use gerenciamento_bd::*;
pub use ultima_insercao::{ultima_insercao_feita};
// usada apenas aqui, à acima exporta.
use ultima_insercao::{atualiza_indice_de_insercao};
pub use grande_inversao::*;
pub use deletador::*;

/// caminho e nome do BD:
const DIR:&'static str = concat!(
   env!("RUST_CODES"),
   "/primos",
   "/data"
);
const NOME_BD:&'static str = "banco_de_dados.dat";
//const REGISTROS_BD:&'static str = "ultima_insercao.dat";

// apelidos para codificação:
type BD = HashMap<u32, Busca>;
type Primos = Vec<u64>;
type Busca = (Primos, u64, u64, u64);
/* não se repete com frequência, porém
 * deixa o código bem limpo(legível).  */
type BancoDeDados = Result<BD, Error>;


/// registra os `Dados` no disco.
pub fn salva_no_bd(dados:Busca) {
   // configurando o arquivo ao abrir.
   let caminho = Path::new(DIR).join(NOME_BD);
   let arquivo: Result<File, Error>  =  {
      OpenOptions::new()
      .append(true)
      .open(caminho)
   };
   
   match arquivo {
      // se o arquivo foi aberto com sucesso.
      Ok(mut arq) => {
         // gravando primeiro o tanto de elementos da array.
         let tamanho_array:u64 = dados.0.len() as u64;
         let seus_bytes:&[u8] = &tamanho_array.to_le_bytes()[..];
         arq.write(seus_bytes).unwrap();

         // agora gravando array, valor por valor, byte por byte.
         for valor in dados.0.iter() {
            let seus_bytes:&[u8] = &valor.to_le_bytes()[..];
            arq.write(seus_bytes).unwrap();
         }

         // gravando último-número-verificado...
         let seus_bytes:&[u8] = &dados.1.to_le_bytes()[..];
         arq.write(seus_bytes).unwrap();

         // gravando tempo-de-computação(em miliseg)...
         let seus_bytes:&[u8] = &dados.2.to_le_bytes()[..];
         arq.write(seus_bytes).unwrap();

         // gravando mini-varreduras-por-100-números...
         let seus_bytes:&[u8] = &dados.3.to_le_bytes()[..];
         arq.write(seus_bytes).unwrap();
      },
      // parar programa em caso de erro.
      Err(_) => 
         { panic!("erro ao abrir e gravar dados no arquivo."); }
   };
   /* toda vez que for inserido com sucesso, ou 
    * não um novo dado no BD, vamos atualizar o
    * índice de inserção de tal. Para comportar
    * a consulta ao BD mais veloz que têm-se. */
   match ultima_insercao_feita() {
      Some(indice) => {
         let mensagem = format!(
            "atualizando o índice para [inserção nº.: {}]", 
            indice + 1
         );
         println!("{}", mensagem.as_str());
         atualiza_indice_de_insercao(indice+1);
      },
      None => (),
   };
}

/// lê todo o conteúdo no banco de dados.
pub fn carrega_bd() -> BancoDeDados {
   let mut tabela_dados = BD::new();
   let mut indice: u32 = 1;

   // abrindo o arquivo do banco-de-dados...
   let caminho = Path::new(DIR).join(NOME_BD);
   let mut arquivo: File = {
      OpenOptions::new()
      .read(true)
      .open(caminho)?
   };

   /* enquanto não lê todos bytes do arquivo, ficar
    * num laço infinito. Só para quando o alerta de
    * fim de arquivo da função 'read_exact' apitar,
    * assim "emitirá" uma quebra de laço. */
   'leitura: loop { 
      // array para armazenar primos escaneados.
      match deserializa_uma_busca(&mut arquivo) {
         Some(tupla) => {
            //println!("{:?}", tupla.0);
            //assert!(false);
            tabela_dados.insert(indice, tupla);
            // contabilizando índice para próxima inserção.
            indice += 1
         } None => break 'leitura,
      };
   }
   Ok(tabela_dados)
}

/// coleta todos primos do banco de dados. 
pub fn coleta_todos_primos() -> Option<Primos> {
   // carregando todo o banco de dados...
   match carrega_bd() {
      // se for sucedido e abrir-lô.
      Ok(tabela_hash_dados) => {
         // vázio é inútil.
         if tabela_hash_dados.is_empty() 
            { return None; }
         let total: u32 = tabela_hash_dados.len() as u32;
         // array para acomodar primos.
         let mut primos = Primos::new();
         for indice in 1..=total {
            // 'tupla' com dados.
            let tupla = {
               tabela_hash_dados
               .get(&indice)
               .unwrap()
            };
            // colocando 'fatia' na array.
            primos.extend(&tupla.0[..]);
         }
         // retornando 'todos primos'.
         Some(primos)
      } Err(_) =>  None
   }
}

/// obtem o 'último número computado'.
pub fn ultimo_numero_computado() -> Option<u64> {
   match carrega_bd() {
      Ok(bd) => {
         // banco de dados vázio, sem valor desejável.
         if bd.is_empty() 
            { return None; } 
         // última chave.
         let ultima_chave = bd.len() as u32;
         // obtendo 'tupla' com dados desejados.
         let dados = bd.get(&ultima_chave).unwrap();
         // o dado desejado é o segundo da 'tupla'
         Some(dados.1)
      } Err(erro) => 
         { println!("{}", erro);  None }
   }
}

/* Deserializa uma 'Busca' adicionado no BD
 * como bytes. Faz apenas uma vez à cada 
 * chamada. */
pub fn deserializa_uma_busca(file:&mut File) -> Option<Busca> {
   // colocar bytes, temporariamente, de um valor lido(buffer).
   let mut aux:[u8; 8] = [u8::MAX; 8]; 
   /* lendo total de valores de 64bits a gerá e
    * colocar numa array. */
   let tamanho:u64;
   match file.read_exact(&mut aux) {
      Ok(_) => tamanho = u64::from_le_bytes(aux),
      Err(_) => return None
   };
   // array para armazenar primos escaneados.
   let mut primos:Vec<u64> = Vec::new();
   // pegando primos 'tamanho' vezes.
   for _ in 1..=tamanho {
      // obtendo valor de 8-bytes.
      match file.read_exact(&mut aux) {
         Ok(_) => (),
         Err(_) => return None
      };
      // colocando na array primo recuperado do disco.
      primos.push(u64::from_le_bytes(aux));
   }
   // obtendo valor de 8-bytes.
   match file.read_exact(&mut aux) {
      Ok(_) => (),
      Err(_) => return None,
   };
   // transformando 8-bytes array num valor de 64bits
   let unv = u64::from_le_bytes(aux);
   // obtendo valor de 8-bytes.
   match file.read_exact(&mut aux) {
      Ok(_) => (),
      Err(_) => return None,
   };
   // transformando 8-bytes array num valor de 64bits
   let ttc = u64::from_le_bytes(aux);
   // obtendo valor de 8-bytes.
   match file.read_exact(&mut aux) {
      Ok(_) => (),
      Err(_) => return None,
   };
   // transformando 8-bytes array num valor de 64bits
   let mvr = u64::from_le_bytes(aux);
   Some((primos, unv, ttc, mvr))
}


#[cfg(test)]
mod tests {
   // importando escope acima.
   use super::*;
   use std::fs::{remove_file};
   use std::path::Path;
   use std::process::Command;
   
   fn zerando_bd() {
      match remove_file(Path::new(NOME_BD)) {
         Ok(_) => { println!("removido com sucesso."); },
         Err(_) => { println!("o arquivo não existe."); }
      };
      println!("criando um novo.. feito");
      Command::new("touch banco_de_dados.dat")
      .spawn()
      .unwrap();
      println!("todo BD foi deletado com sucesso..");
   }

   #[test]
   #[ignore]
   fn registrando_no_banco() {
      // deleta todo banco de dados primeiramente...
      zerando_bd();
      let array = vec![389, 519, 100, 6261];
      let dados = (array, 1, 2, 3);
      salva_no_bd(dados);
      let array = vec![9, 99, 999, 9999];
      let dados = (array, 99_998, 15_000, 5_000);
      salva_no_bd(dados);
      let array = vec![8, 5, 10, 62];
      let dados = (array, 4, 5, 6);
      salva_no_bd(dados);
      let array = vec![19, 17, 11, 12];
      let dados = (array, 7, 8, 9);
      salva_no_bd(dados);
      assert!(true);
   }

   #[test]
   #[ignore]
   fn lendo_todas_insercoes_no_bd() {
      println!("a númeração das inserções não segue a ordem gravada:");
      match carrega_bd() {
         Ok(dados) => {
            for (i, insercao) in dados.values().enumerate() {
               println!("--- --- inserção({}) --- ---", i+1);
               println!("array:{:?}\nunv={}\nttc={}\nqb={}\n", 
                  insercao.0, insercao.1, 
                  insercao.2, insercao.3 
               );
            }
            let backup = dados.get(&1).unwrap();
            assert_eq!(
               *backup, 
               (vec![389,519,100,6261], 1, 2, 3)
            );
         },
         Err(_) => { assert!(false); }
      }
   }


   #[test]
   #[ignore]
   fn coletando_todos_primos_inseridos() {
      match coleta_todos_primos() {
         Some(primos) => {
            println!("primos: {:?}", primos);
            assert_eq!(primos.len(), 4*4)
         },
         None => assert!(false)
      };
   }

   #[test]
   #[ignore]
   fn verificando_ultimo_numero_computado_inserido() {
      match ultimo_numero_computado() {
         Some(numero) => assert_eq!(7, numero),
         None => assert!(false)
      };
   }

   #[test]
   #[ignore]
   fn testando_multiplas_insercoes() {
      // limpando dados já colocados.
      zerando_bd();
      // inserindo 1000 dados redundantes.
      for _ in 1..=5_000 {
         let dados = (
            vec![9, 99, 999, 9_999, 99_999, 999_999],
            10, 20, 30
         );
         salva_no_bd(dados);
      }
      // carregando dados_novamentes.
      let dados = carrega_bd().unwrap();
      assert!(dados.len() == 5_000);
   }

   extern crate utilitarios;
   use utilitarios::barra_de_progresso::progresso;
   #[test]
   #[ignore]
   fn testando_multiplas_insercoes_com_dados_grandes() {
      // limpando dados já colocados.
      zerando_bd();
      // inserindo 1000 dados redundantes.
      for p in 1u64..=5_000u64 {
         let dados = ([23; 1_000].to_vec(), 10, 200, 3000);
         salva_no_bd(dados);
         let bp = progresso(p, 5000);
         print!("\r{}", bp); 
         drop(bp);
      }
      // carregando dados_novamentes.
      let dados = carrega_bd().unwrap();
      assert!(dados.len() == 5_000);
   }
   
   #[test]
   #[ignore]
   fn lendo_insercoes_em_serie() {
      match carrega_bd() {
         Ok(tabela) => {
            for chave in tabela.keys() {
               let dado = tabela
               .get(chave)
               .unwrap();
               println!(
                  "\n\r{}ª inserção:
                  \rtp := {}
                  \runv := {}
                  \rttc := {}
                  \rqb100 := {}", 
                  *chave, 
                  dado.0.len(),
                  dado.1,
                  dado.2,
                  dado.3
               ); 
            }
         },
         Err(_) => (),
      };
      assert!(true);
   }
}
