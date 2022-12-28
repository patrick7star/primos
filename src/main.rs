// biblioteca padrão do Rust:
use std::env::args;

// minha biblioteca:
mod motor; 
mod banco_de_dados;
mod menu;
//mod gerenciamento_bd;
//mod deletador;
//mod organizacao_bd;

// define-se com quase mil número primos a buscar.
const A_BUSCAR:u64 = 932;
/* definindo um novo tipo de dados para a tupla
 * que representa a colêtanea produzida pela 
 * varredura. */
type Dados = (Vec<u64>, u64, u64, u64);

// se há um argumento para burlar prompt de confirmação.
fn burla_prompt(args:&mut Vec<String>) -> bool {
   // verificando confirmas em todos argumentos passados.
   for (indice, s) in args.iter().enumerate() {
      let arg = s.to_lowercase();
      let arg = arg.trim();
      if arg == "--sim" || arg == "--yes" { 
         // primeiro o remove.
         args.remove(indice);
         return true; 
      }
   }
   return false;
}

fn main() {
   // trabalhando na entrada de terminal...
   let mut entrada:Vec<String> = args().collect();
   // salva automaticamente pós-termino.
   let salvo_automatico = burla_prompt(&mut entrada);
   /* obtem a opção e o possível argumento e 
    * gera o melhor enum que trabalha em cima
    * dele. */
   let argumentos = menu::transforma(&entrada);

   /* executa o menu, dado o tipo de argumento
    * gerado anteriormente. */
   menu::menu(argumentos, salvo_automatico);
}

use std::path::PathBuf;
use std::env::current_exe;
/* computa o caminho ao diretório
 * baseado no caminho do executável. */
fn computa_caminho(extra: &str) -> PathBuf {
   match current_exe() {
      Ok(mut path) => {
         // indo para a parte principal do crate.
         path.pop(); path.pop(); path.pop();
         path.push(extra);
         path
      } Err(erro) =>
         { panic!("erro: '{}'", erro); }
   }
}

