/* Gerador de executável que cria várias forks da função que executa
 * a busca contínua por primos dado o intervalo. Você demanda a quantia
 * necessária.
 */
// Bibliotecas externas:
extern crate libc;
// Biblioteca do Rust:
use std::process::{Stdio, Command};
use std::str::FromStr;
use std::iter::Iterator as I;
use std::path::Path;
use std::time::{Duration, Instant};
use std::ffi::{c_void};
use std::mem::{size_of, swap};
use std::mem::{zeroed, transmute};
use libc::{stat as Stat};
// Do próprio módulo:
use crate::motor::{
   MonitorProcessamento, InfoDeProcessamento, Primos, varre, 
   divide_intervalo, Intervalo, NOME_DO_PIPE, Processo, 
   primos_faixa_com_info, primos_faixa
};


pub fn string_para_range(s: String) -> Intervalo {
   // Decompondo ...
   let mut partes = s.split("..=");
   let inicio = partes.next().unwrap();
   let fim = partes.next().unwrap();

   // transformando em inteiros.
   let mut inicio = u64::from_str(inicio).unwrap();
   let mut fim = u64::from_str(fim).unwrap();
   /* intervalo só trabalhado na ordem crescente,
    * então caso vem o inverso, tal função
    * o inverte. */
   if inicio > fim
      { swap(&mut inicio, &mut fim); } 

   // Formando o intervalo em sí.
   inicio..=fim
}

/* Captura os argumentos e retorna um intervalo, onde será feito à busca 
 * pela quantia de primos. */
pub fn filtra_intervalo(i: impl I<Item=String>) -> Intervalo 
{
   string_para_range(
      // Retira apenas intervalo.
      i.filter(|s| { 
         let p1 = s.contains("..");
         let p2 = s.contains('=');
         /* contém um tipo, ou o outro, ou
          * também a combinação de ambos que 
          * é o caso. */
         p1 || p2
      }).next().unwrap()
   )
}

/* Gera vários processos que processas vários intervalos dados. Gera vários
 * forks chamando esta função quantas vezes achar que for necessário. */
pub fn gera_processo(i: Intervalo) -> Processo {
   // Binários otimizados, e o debug se não houver.
   let binario_otimizado = Path::new("target/release/primos");
   let binario_debug = Path::new("target/debug/primos");
   // Formando o comando a executar futuramente...
   let mut cmd = {
      Command::new(
         if binario_otimizado.exists() 
            { binario_otimizado.to_str().unwrap() } 
         else 
            { binario_debug.to_str().unwrap() }
      )
   };

   cmd.arg("varre");
   /* Converte itervalo para string.
    * Adiciona o argumento em sequência. */
   cmd.arg(format!("{}..={}",*i.start(), *i.end()));
   cmd.stdout(Stdio::piped());
   // execuntando ...
   cmd.spawn()
}

fn cria_o_canal() {
/* Cria o arquivo(um named pipe), aonde os bytes dos primos processados 
 * serão enviados. */
   let pathname = NOME_DO_PIPE.as_ptr() as *const i8;

   unsafe { 
      let out = libc::mkfifo(pathname, 666);  

      if out == -1 
         { println!("Não foi possível criar a tubulação!"); }
      else
         { println!("Tubulação criada com sucesso."); }
   }
}

fn pipe_name() -> *const i8
/* Retorna o ponteiro(estilo do C) do do nome do 'named pipe'. */
   { NOME_DO_PIPE.as_ptr() as *const i8 }

fn envia_dados_por_namedpipes(lista: &Vec<u64>)
{
/* Envia uma array finita de bytes de cada número inteiro sem sinal de 
 * 64-bits que contém nela. */
   // Computa o total de bytes a enviar:
   let total = lista.len() * size_of::<u64>();
   let pathname = pipe_name();
   let modo = libc::O_WRONLY;
   // Ponteiro para 'raw array' da estrutura de lista.
   let data = lista.as_ptr() as *const c_void;

   unsafe {
      let fd = libc::open(pathname, modo);
      let escrito = libc::write(fd, data, total);

      assert_eq!(escrito, total as isize);
      libc::close(fd);
   }
}

fn encontra_primos_e_envia_via_named_pipes(i: &Intervalo) -> Primos
{
   if cfg!(debug_assertions) 
      { println!("\tPrimos buscados em {:?}", *i); }

   // Realiza varredura para encontrar primos.
   let mut output = varre(i.clone());
   assert!(output.len() > 0);
   let lista = output.drain().collect::<Vec<u64>>();

   envia_dados_por_namedpipes(&lista);
   // Lista que serve apenas como buffer pode ser liberada agora.
   drop(lista);

   // Informando saída ...
   if cfg!(debug_assertions) 
      { println!("\t\tVarredura de {:?} finalizada", *i); }

   // Pensando no processo pai, retorna o conteúdo encontrado aqui.
   output
}

fn recebe_primos_um_por_um_via_named_pipes(out: &mut Primos) {
/* Captura bytes de cada inteiro sem sinal de 64-bits que está sendo enviado
 * via 'named pipes'. Colocas num 'conjunto' de primos. */
   const N: usize = size_of::<u64>();
   let mut buffer: [u8; N] = [u8::MAX; N];
   let data = buffer.as_mut_ptr() as *mut c_void;
   let modo = libc::O_RDONLY;
   let mut clock = Instant::now();
   let limite = Duration::from_micros(500); 

   unsafe {
      let via = libc::open(pipe_name(), modo);

      // Cada primo lido vai sendo adicionado ao conjunto de primos ...
      'leitura: loop { 
         let lido = libc::read(via, data, N);

         if clock.elapsed() > limite {
            let mut mt: libc::stat = zeroed(); 
            let ptr = transmute::<&mut Stat, *mut Stat>(&mut mt);

            libc::fstat(via, ptr);
            println!("bytes: {buffer:?}, lido: {lido:}");
            println!("Ainda no buffer: {} bytes", mt.st_blocks);
            clock = Instant::now();
         }

         if lido == N as isize
            { out.insert(u64::from_le_bytes(buffer)); }

         if lido == 0
            { break 'leitura; }
      }
      libc::close(via);
   }
}

fn varre_em_paralelo(i: Intervalo) -> Primos 
{
/* Faz o processamento via multiplos processos; isso tudo via uma chamada
 * de sistema, o 'fork'. Este método de apelar multiplos processo faz com
 * que todo recurso do CPU seja utilizado. O processamento de primos em seu
 * máximo. */
   let ranges = divide_intervalo(i, 4);
   let rotine: fn(&Intervalo) -> Primos;
   let sucesso = libc::EXIT_SUCCESS;
   let mut output: Primos;

   // Atribuido aqui pra não transbordar a tela.
   rotine = encontra_primos_e_envia_via_named_pipes;

   /* Começando a forquear subprocessos para processar seus respectivos
    * intervalos dados. Neste caso aqui, estamos falando de 4 forques,
    * pois são quatro CPUs ao todo na máquina. */
   unsafe {
      if libc::fork() == 0 {
         rotine(&ranges[0]); 
         libc::exit(sucesso); 
      } else { 
         if libc::fork() == 0 {
            rotine(&ranges[1]);
            libc::exit(sucesso); 
         } else {
            if libc::fork() == 0 {
               rotine(&ranges[2]);
               libc::exit(sucesso); 
            }   
            output = varre(ranges[3].clone()); 
         }
      }
   }
   
   /* Recebendo os primos enviados, então juntando com os demais. No fim,
    * apenas retorna o resultado encontrado. */
   recebe_primos_um_por_um_via_named_pipes(&mut output);
   output
}

pub fn varre_em_paralelo_com_info
  (faixa: Intervalo, info: MonitorProcessamento) -> Primos 
{
   let ranges = divide_intervalo(faixa, 4);
   let rotine: fn(&Intervalo) -> Primos;
   let sucesso = libc::EXIT_SUCCESS;
   let mut output: Primos;

   // Atribuido aqui pra não transbordar a tela.
   rotine = encontra_primos_e_envia_via_named_pipes;

   /* Começando a forquear subprocessos para processar seus respectivos
    * intervalos dados. Neste caso aqui, estamos falando de 4 forques,
    * pois são quatro CPUs ao todo na máquina. */
   unsafe {
      if libc::fork() == 0 {
         rotine(&ranges[0]); 
         libc::exit(sucesso); 
      } else { 
         if libc::fork() == 0 {
            rotine(&ranges[1]);
            libc::exit(sucesso); 
         } else {
            if libc::fork() == 0 {
               rotine(&ranges[2]);
               libc::exit(sucesso); 
            }   
            output = varre(ranges[3].clone()); 
         }
      }
   }
   
   /* Recebendo os primos enviados, então juntando com os demais. No fim,
    * apenas retorna o resultado encontrado. */
   recebe_primos_um_por_um_via_named_pipes(&mut output);
   output
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

   use super::*;
   use std::env::args;

   #[test]
   fn testeBasicoFI() {
      println!("digite um intervalo: (ínicio)..=(fim)");
      let argumentos = filtra_intervalo(args());
      println!("{:#?}", argumentos);
      // avaliação manual.
      assert!(true);
   }

   #[test]
   fn numCPUsExternPackage() {
      use num_cpus;
      println!("total de CPU's é {}.", num_cpus::get());
      assert_eq!(num_cpus::get(), 4);
   }

   fn envia_via_named_pipes() {
      let data = [5_i32, 4, 3, 2, 9, 8, 7, 6];

      unsafe {
         let n = size_of::<i32>() * data.len();
         let pathname = NOME_DO_PIPE.as_ptr() as *mut i8;
         let modo = libc::O_WRONLY as i32;
         let fd = libc::open(pathname, modo);
         let dump = libc::write(fd, data.as_ptr() as *mut c_void, n);

        if dump == -1
           { println!("Não foi possível enviar no pipe!"); }
         libc::close(fd);
      }
   }

   #[test]
   fn simplesTesteDeNamedPipesEmRust() {
      let mut buffer: [i32; 8] = [-1; 8];

      println!("buffer: {:?}", buffer);
      cria_o_canal();


      if unsafe { libc::fork() == 0} { 
         let code = libc::EXIT_SUCCESS;
         envia_via_named_pipes(); 
         unsafe { libc::exit(code)};
      }

      unsafe {
         let n = size_of::<u32>() * buffer.len();
         let pathname = NOME_DO_PIPE.as_ptr() as *mut i8;
         let modo = libc::O_RDONLY as i32;
         let fd = libc::open(pathname, modo);
         libc::read(fd, buffer.as_mut_ptr() as *mut c_void, n);
         libc::close(fd);
      }

      println!("buffer: {:?}", buffer);
   }

   fn lista_todos_primos(conjunto: &Primos)
   {
      for prime in conjunto
         { print!("{}, ", prime); }
      println!();
   }

   #[test]
   fn construcaodeVarreduraViaForque() {
      let output = varre_em_paralelo(1..=5_000);

      println!("Primos encontrados em até 1000: {}", output.len());
      lista_todos_primos(&output); 
   }

   #[test]
   fn processamentoRealmenteBastantesPrimos() {
      let faixa = 1..=10_000_000; 
      let mut info = InfoDeProcessamento::nova();
      let out = varre_em_paralelo_com_info(faixa, Some(&mut info));
   }
}
