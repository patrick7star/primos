/* 
 * Já que o banco de dados que armazena em binários
 * está pronto, vamos copiar o em 'txt' para ele, fazendo
 * as devidas conversões.
 */

// biblioteca padrão do Rust.
use std::path::{PathBuf, Path};
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use std::fs::{File, create_dir};
use std::io::Read;
use std::process::Command;
use std::env::temp_dir;
// próprio caixote.
use crate::{Dados, computa_caminho};
// próprio módulo.
use super::deserializa_uma_busca as deserializa;

// nome do atual BD.
const CAMINHO_BD:&str = "data/banco_de_dados.dat";
const CAMINHO_UI:&str = "data/ultima_insercao.dat";
const CAMINHO_REGISTROS:&str = "data/registros.dat";
// diretório dos backup's.
const TODOS_BACKUPS:&str = "data/backups/";
const NOME_ORIGINAL_ARQ:&str = "backup_bd";

/* abre o arquivo zip, lê seus dados e retorna
 * se for um arquivo incompatível dispara erro. */
type BD = HashMap<u32, Dados>;
// renomeando tipo de dado para legibilidade.
type Str = &'static str;
// backup completo.
type Backup = Result<(u32, BD), Str>;


// computa um 'nome+ID' para dá ao archive criado.
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
 realiza um novo backup do BD, ele salva 
 tanto o aglomerado de dados gerados, como 
 o atalho para acessar o BD de forma
 mais específica.  
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

// descompacta um backup.
fn descompacta(caminho:&Path) -> Option<PathBuf> {
   /* um nome inválido, portanto, o caminho também
    * inválido produz nenhum dado. */
   if !caminho.exists()
      { return None; }

   // nome do arquivo em sí.
   let nome: &str = {
      caminho
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
   };
   // cria diretório para armazenar "artefatos".
   let destino = {
      let nome_dir = {
         nome
         .strip_suffix(".zip")
         .unwrap()
      };
      let mut caminho = temp_dir();
      caminho.push(nome_dir);
      caminho
   };
   match create_dir(destino.as_path()) {
      Ok(_) => (),
      Err(_) => { 
         println!("\"{}\" já existe.", destino.display()); 
         print!("\texcluindo-o ...");
         create_dir(destino.as_path()).unwrap();
         println!("feito.");
      }
   };

   // comando de descompactação:
   let mut comando = Command::new("unzip");
   comando.arg("-o");
   comando.arg(caminho);
   comando.arg("-d");
   comando.arg(destino.as_path());

   // executa o comando.
   comando.output().unwrap();
   // retora diretório contedo arquivos descompactados.
   return Some(destino);
}

// carrega arquivo do disco.
fn carrega_bd_do_arquivo(mut arquivo:File) -> Result<BD, Str> {
   let mut tabela: BD = BD::new();
   let mut indice:u32 = 1;

   loop { 
      match deserializa(&mut arquivo) {
         // tupla representando busca.
         Some(tupla) => {
            tabela.insert(indice, tupla);
            indice += 1;
         } None => break Ok(tabela)
      };
   }
}

// carrega arquivo contendo número de inserções.
fn ultima_insercao_feita(mut arquivo:File) -> Option<u32> {
   // buffer:
   let mut bytes = [u8::MAX; 4];

   // lendo os únicos 4 bytes dentro do arquivo.
   match arquivo.read_exact(&mut bytes) {
      Ok(_) => {
         let valor = u32::from_le_bytes(bytes);
         Some(valor)
      } Err(_) => None
   }
}

/**
 Extrai um backup passado e carrega-o
 na memória assim como faz com o atual 
 consultado. 
*/
fn carrega_backup(caminho: &Path) -> Backup {
   let base: PathBuf;
   match descompacta(caminho) {
      Some(path_do_dir) => 
         { base = path_do_dir; },
      None => { 
         println!(
            "Tem certeza que este '{}' caminho é o certo?", 
            caminho.display()
         );
         return Err("caminho inválido!"); 
      }
   };

   // caminhos para os arquivos:
   let ui_file = base.as_path().join(CAMINHO_UI);
   let bd_file = base.as_path().join(CAMINHO_BD);

   // conteúdos em sí:
   let file = File::open(ui_file).unwrap();
   let ui = ultima_insercao_feita(file).unwrap(); 
   let file = File::open(bd_file).unwrap();
   let bd = carrega_bd_do_arquivo(file)?;

   return Ok((ui, bd));
}

// uso para testes:
#[allow(unused_imports)]
use super::deletador::DeletorPaciente;

#[cfg(test)]
mod tests {
   // bibliotecas para teste.
   use std::thread::sleep;
   use std::time::Duration;
   use super::*;
   use std::fs::read_dir;
   extern crate utilitarios;
   use utilitarios::{
      arvore::arvore,
      por_extenso::escreve_por_extenso
   };

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

   #[test]
   fn testa_descompacta() {
      let entradas = read_dir(TODOS_BACKUPS).unwrap(); 

      let t = Duration::from_secs(20);
      let mut xis = DeletorPaciente::novo(t);

      for entrada in entradas {
         let caminho = entrada.unwrap().path();
         let caminho_zip = {
            descompacta(caminho.as_path())
            .unwrap()
         };

         // caminho do diretório descompactado:
         xis += caminho_zip.clone();

         println!("descompactado em {:#?}.", caminho_zip);
         let caminho_str:&str = {
            caminho_zip
            .as_path()
            .to_str()
            .unwrap()
         };
         println!("{}\n", arvore(caminho_str, true));
      }
   }

   // info do backup carregado.
   fn info_backup(dados: (u32, BD)) {
      let (mut ui, bd) = dados;
      /* erro de incremento ao gravar. Conflito
       * de dados antigos com novos. */
      let relevante = match bd.get(&dbg!(ui)) {
         Some(conteudo) => conteudo,
         None => 
            { ui -= 1; bd.get(&(ui-1)).unwrap() }
      };
      // primeiro e último primo encontrado.
      let t = relevante.0.len();
      let first_prime = relevante.0[0];
      let last_prime = relevante.0[t-1];
      println!(
         "\túltima inserção(total também): {}
         \r\tprimeiro/e útlimo primos: {} e {}
         \r\ttotal de primos: {}", 
         ui, first_prime, last_prime,t
      )
   }

   #[test]
   fn teste_de_carrega_backup() {
      let entradas = read_dir(TODOS_BACKUPS).unwrap();
      let mut contador = 0;

      for entrada in entradas {
         let path = entrada.unwrap().path();
         println!("\n{:#?}", path);
         match carrega_backup(path.as_path()) {
            Ok(tupla) => 
               { info_backup(tupla); }
            Err(_) => {}
         };
         assert!(true);
         contador += 1;
      }

      let textual = escreve_por_extenso(contador as u64);
      println!("\n{} ao total carregados.", textual.unwrap());
      // avaliação manual.
      assert!(true);
   }
}
