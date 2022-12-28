
use std::process::Command;
use std::time::Duration;
use std::path::{PathBuf, Path};
use std::ops::{Drop, AddAssign};
use std::thread::sleep;

// caminho do executável.
const DIR: &'static str = "target/release/deps";
const NOME_EXE: &'static str = "fork_remocao";
// tempo de delay da exclusão dos diretórios/arquivos.
const DELAY: f32 = 3.5;
const CODIGO_FONTE: &'static str = concat!(
   "src/banco_de_dados/",
   "fork_remocao.rs"
);


pub struct DeletorPaciente {
   lista: Vec<PathBuf>,
   tempo_de_espera: Duration,
   //cronometro: Option<Instant>
}

impl DeletorPaciente {
   pub fn novo(tempo: Duration) -> Self {
      Self { 
         //cronometro: None,
         tempo_de_espera: tempo,
         lista: Vec::new()
      }
   }
}

impl Drop for DeletorPaciente {
   fn drop(&mut self) {
      let total = self.lista.len();

      for caminho in self.lista.drain(..) {
         // compila se necessário ...
         compila_fork();

         // cria fork convocando o comando.
         let tde = self.tempo_de_espera;
         // extraí caminho como string.
         let path = {
            caminho.as_path()
            .to_str().unwrap()
         };
         /* forma comando como string.
          * remoção em sí: */
         let cmd_str = format!("{}/{}", DIR, NOME_EXE);
         let mut comando = Command::new(cmd_str);
         comando.arg(path);
         comando.arg(tde.as_secs().to_string());
         comando.spawn().unwrap();

         // espaça as deletações em um segundo e meio.
         sleep(Duration::from_secs_f32(DELAY));
      }
      println!(
         "todos os {} diretórios foram colocadas
         \rem processo de exclusão.", total
      );
   }
}

impl AddAssign<PathBuf> for DeletorPaciente {
   fn add_assign(&mut self, caminho: PathBuf) 
      { self.lista.push(caminho); }
}

         
/* realiza uma compilação do executável
 * que quando forqueado excluí tais
 * diretórios.
 */
fn compila_fork() {
   // só faz se não existir um compilado.
   let executavel = Path::new(DIR).join(NOME_EXE);
   if executavel.exists() {
      println!("compilado já existe!");
      return ();
   } else {
      let mut comando = Command::new("rustc");
      // argumentos: onde compilar e que código.
      comando.arg("--out-dir");
      comando.arg(DIR);
      comando.arg(CODIGO_FONTE);
      // execuntando ...
      comando.status().unwrap();
      println!(
         "'{}' compilado em '{}' com sucesso.", 
         NOME_EXE, DIR
      );
   }
}

