
/** tem como objetivo ser "forqueado",
 para remoção posterior dos caminhos
 passados como argumento, dado um tempo,
 também passado como argumento.
 */

// biblioteca padrão do Rust:
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::env::args;
use std::mem::drop;
use std::str::FromStr;


fn main() {
   // caminho passado como argumento.
   let mut argumentos = args();
   drop(argumentos.next());
   let caminho = argumentos.next().unwrap(); 
   // tempo em segundos.
   let tempo = argumentos.next().unwrap();

   // pausa antes de começar.
   let valor = u64::from_str(tempo.as_str());
   sleep(Duration::from_secs(valor.unwrap()));

   let mut comando = Command::new("rm");
   comando.arg("-rv");
   comando.arg(caminho.as_str());
   // execuntando ...
   //comando.output().unwrap();
   match comando.output() {
      Ok(_) => 
         { println!("removendo '{}' ... feito!", caminho); }
      Err(_) =>
         { println!("erro ao remover '{}'", caminho); }
   };
}
