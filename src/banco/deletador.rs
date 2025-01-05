use std::process::Command;
use std::time::Duration;
use std::path::{PathBuf};
use std::ops::{Drop, AddAssign};
use std::thread::sleep;
use std::fs::remove_dir_all;

// Caminho do executável.
const NOME_EXE: &str = "target/debug/primos";
// Tempo de delay da exclusão dos diretórios/arquivos.
const DELAY: f32 = 0.839;


pub struct DeletorPaciente {
   lista: Vec<PathBuf>,
   tempo_de_espera: Duration,
}

#[allow(dead_code)]
impl DeletorPaciente {
   pub fn novo(tempo: Duration) -> Self {
      Self { 
         tempo_de_espera: tempo,
         lista: Vec::new()
      }
   }
}

impl Drop for DeletorPaciente {
   /* só choca o processo de exclusão quando
    * o objeto for liberado.
    */
   fn drop(&mut self) {
      let total = self.lista.len();

      print!("começando processos para exclusão...");
      for caminho in self.lista.drain(..) {
         let caminho_str = {
            caminho.as_path()
            .to_str().unwrap()
         };
         let tempo_str = {
            self.tempo_de_espera
            .as_secs().to_string()
         };
         let mut comando = Command::new(NOME_EXE);
         comando.arg("função-deleta-caminho");
         comando.arg(caminho_str);
         comando.arg(tempo_str.as_str());
         comando.spawn().unwrap();
         // espaça as deletações em um segundo e meio.
         sleep(Duration::from_secs_f32(DELAY));
      }

      println!("todos os {} diretórios na fila de exclusão", total);
   }
}

impl AddAssign<PathBuf> for DeletorPaciente {
   fn add_assign(&mut self, caminho: PathBuf) 
      { self.lista.push(caminho); }
}

/* Remove o arquivo demanda. */
pub fn deleta_caminho(caminho: PathBuf, tempo: Duration) {
   // pausa antes de começar.
   sleep(tempo);

   // deleta diretório ou arquivo.
   match remove_dir_all(caminho.clone()) {
      Ok(_) => 
         { println!("removendo '{:#?}' ... feito!", caminho); }
      Err(_) =>
         { println!("erro ao remover '{:#?}'", caminho); }
   };
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   // bibliotecas para teste.
   use super::*;
   use std::path::Path;
   use std::time::{Duration, Instant};


   #[test]
   #[ignore="precisa de uma 'pasta-copia' no desktop"]
   fn DeletaCaminho() {
      let caminho_str = concat!(
         env!("HOME"),
         "/Desktop",
         "/pasta-copia"
      );
      let caminho = Path::new(caminho_str);
      let cronometro = Instant::now();
      assert!(caminho.exists());
      deleta_caminho(
         caminho.to_path_buf(), 
         Duration::from_secs(10)
      );
      assert!(!caminho.exists());
      assert!(cronometro.elapsed() < Duration::from_secs(13));
      println!("{:#?}", cronometro.elapsed());
   }
}
