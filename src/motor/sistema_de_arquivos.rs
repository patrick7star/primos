

/* Vamos construir um modo de
 * processamento paralelo, e jeito
 * de transmitir tais informações via
 * entre os processos.
 * Cada dos 'n' processos chamados 
 * serão "chocados", eles também, usando
 * várias threads simultâneas, computam
 * baseado nos argumentos passados. Os
 * parâmetros não envolverão nada mais
 * que os da função de "busca contínua".
 */

// importando do módulo:
use super::turbina::{Primos, simultaneadade};
// do Rust:
use std::fs::{
   OpenOptions, File, ReadDir,
   read_dir, create_dir_all,
   remove_dir_all
};
use std::path::Path;
use std::io::{Error, ErrorKind, Read, Write};

type Str = &'static str;
// onde ficarão, tipo os... "fifo files".
const RAIZ: Str = "data/transmissao";


pub fn cria_arquivos(n: usize) {
   match create_dir_all(RAIZ) {
      Ok(_) => 
         { println!("'{}' criado com sucesso.", RAIZ); }
      Err(_) =>
         { println!("'{}' tal diretório já existe.", RAIZ); }
   };

   for i in 1..=n {
      let nome = format!("pipe::{:0>2.2}", i);
      let caminho = {
         Path::new(RAIZ)
         .join(nome.as_str())
      };
      let mut arquivo = {
         OpenOptions::new()
         .create(true)
         .write(true)
         .open(caminho)
         .unwrap()
      };
      // escreve "nada" nele.
      arquivo.write(b"").unwrap();
   }
}

/* extrai os primos dado o arquivo. É 
 * esperado tenham sido colocados lá por 
 * meio de um conjunto, ou seja, só há 
 * inteiros de 64-bits, qualquer coisa 
 * a mais, será rejeitada. */
fn extrai_primos(mut arquivo: File) -> Option<Primos> {
   let mut buffer: [u8; 8] = [u8::MAX; 8]; 
   /* calculando quantia total de 
    * primos que pode haver ... */
   let qtd = (arquivo.metadata().unwrap().len() / 8) as usize;
   // mais dois, caso a conta esteja errada.
   let mut saco = Primos::with_capacity(qtd + 2);

   /* presupondo que nenhum arquivo está 
    * comrrompido, e possui a quantia(um 
    * múltiplo de oito) certa de bytes. */
   'varredura: loop {
      match arquivo.read_exact(&mut buffer) {
         Ok(_) => 
            { saco.insert(u64::from_be_bytes(buffer)); } 
         Err(_) =>
            { break 'varredura; }
      };
   }

   /* se chegar sem nada, então não 
    * retornar nada. */
   if saco.is_empty()
      { return None; }

   return Some(saco);
}

fn diretorio_vazio() -> bool {
   match read_dir(RAIZ) {
      Ok(entradas) => entradas.count() == 0,
      Err(_) => true
   }
}

/* coleta todos primos contido nos arquivos
 * do diretório, e coloca-os dentro de um
 * conjunto. */
pub fn coletagem() -> Result<Primos, Str> {
   // tenta pegar entradas, ou retorna 'erro'.
   if diretorio_vazio() {
      let msg = "sem qualquer 'pipe' no diretório";
      return Err(msg);
   }
   let entradas = read_dir(RAIZ).unwrap();
   let mut saco = Primos::with_capacity(25_000);

   for e in entradas {
      let caminho = e.unwrap().path();
      println!("{}", caminho.display());
      let arquivo = File::open(caminho).unwrap();
      match extrai_primos(arquivo) {
         Some(mut conjunto) => {
            for x in conjunto.drain()
               { saco.insert(x); }
         } None => 
            { println!("contabilizando chamadas ..."); }
      };
   }

   assert!(saco.len() > 0);
   if saco.is_empty()
      { return Err("arquivos estão vázios"); }

   // reduzindo espaço desnecessário ...
   saco.shrink_to_fit();
   /* remove diretório e seus arquivos de 
    * suporte para transferência. */
   remove_dir_all(RAIZ).unwrap();

   return Ok(saco);
}

/* Põe todos primos dados num dado
 * arquivo no diretório raíz. */
fn registra(mut dados: Primos, nome_arq: &str) 
-> Result<(), Error> {
   let caminho = Path::new(RAIZ).join(nome_arq);

   // condições necessárias:
   assert!(dados.len() > 0);
   assert!(caminho.exists());

   let mut arquivo = {
      OpenOptions::new()
      .write(true)
      .open(caminho)?
   };

   for p in dados.drain() { 
      let bytes = p.to_be_bytes();
      arquivo.write_all(&bytes[..])?; 
   }

   // escrita foi ok!
   return Ok(());
}

/* o mesmo que a função acima, porém computa o 
 * nome automaticamente, baseado nos arquivos
 * vázios do diretório. Na verdade é isto mesmo,
 * por este motivo que está sendo criado, para ser 
 * uma camada lógica da função acima. */
pub fn despeja(dados: Primos) -> Result<(), Error> {
   let mut nome = String::from(""); 

   for entrada in read_dir(RAIZ).unwrap() {
      let caminho = entrada.unwrap().path();
      let tamanho = {
         caminho.clone()
         .metadata().unwrap()
         .len()
      };
      if tamanho == 0 || tamanho < 8 { 
         nome = {
            caminho.as_path()
            .file_name().unwrap()
            .to_str().unwrap()
            .to_string()
         };
         break;
      }
   }

   // para por que já localizou um 'cano'.
   if !nome.starts_with("pipe::") {
      let msg = "sem nenhum arquivo transmissor";
      let tipo = ErrorKind::BrokenPipe;
      return Err(Error::new(tipo, msg)); 
   }
      

   Ok(registra(dados, nome.as_str())?)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   use std::fs::{read_dir, remove_file};
   extern crate utilitarios;
   use utilitarios::aleatorio::sortear;

   #[test]
   fn criaArquivos() {
      let _entradas = match read_dir(RAIZ) {
         Ok(_) => assert!(false),
         Err(_) => assert!(true)
      };
      cria_arquivos(17);
      let entradas = read_dir(RAIZ).unwrap();
      assert_eq!(entradas.count(), 17);
      remove_dir_all(RAIZ).unwrap();

      cria_arquivos(6);
      let entradas = read_dir(RAIZ).unwrap();
      assert_eq!(entradas.count(), 6);
      remove_dir_all(RAIZ).unwrap();

      cria_arquivos(42);
      let entradas = read_dir(RAIZ).unwrap();
      assert_eq!(entradas.count(), 42);
      remove_dir_all(RAIZ).unwrap();
   }

   #[test]
   fn TodosNumerosForamRegistrados() {
      cria_arquivos(5);
      let mut conjuntos = vec![
         Primos::with_capacity(205),
         Primos::with_capacity(205),
         Primos::with_capacity(205),
         Primos::with_capacity(205),
         Primos::with_capacity(205)
      ];
      for i in 1..=1_000 
         { conjuntos[i % 5].insert(i as u64); }
      for (j, c) in conjuntos.drain(..).enumerate() {
         let nome = format!("pipe::{:0>2.2}", j + 1);
         registra(c, nome.as_str()); 
      }
      let total_de_bytes: u64 = {
         read_dir(RAIZ).unwrap()
         .map(|e| e.unwrap().path())
         .map(|t| t.metadata().unwrap().len())
         .sum()
      };
      assert_eq!(total_de_bytes / 8, 1_000);
      remove_dir_all(RAIZ);
   }

   #[test]
   fn ExtracaoPerfeita() {
      let (pth1, pth2) = (
         Path::new(RAIZ).join("pipe::01"),
         Path::new(RAIZ).join("pipe::02")
      );
      cria_arquivos(2);
      let (f1, f2) = (
         File::open(pth1.clone()).unwrap(),
         File::open(pth2.clone()).unwrap()
      );
      let (mut s1, mut s2) = (
         Primos::with_capacity(5_000),
         Primos::with_capacity(5_000)
      );
      for k in 1..=20_000 {
         if k % 2 == 0 {
            if k <= 10_000
               { s1.insert(k); }
            else
               { s2.insert(k); }
         }
      }
      registra(s1, "pipe::01").unwrap();
      registra(s2, "pipe::02").unwrap();
      let c1 = extrai_primos(f1.try_clone().unwrap()).unwrap();
      let c2 = extrai_primos(f2.try_clone().unwrap()).unwrap();
      // total demandado.
      assert_eq!(c1.len() + c2.len(), 10_000);
      // tamanho de bytes do arquivo o mesmo que o conjunto.
      assert_eq!(
         (c1.len() * 8) as u64, 
         f1.metadata().unwrap().len()
      );
      assert_eq!(
         (c2.len() * 8) as u64, 
         f2.metadata().unwrap().len()
      );
      //apenas pares.
      assert!(pth1.exists() && pth2.exists());
      remove_file(pth1.clone()); 
      remove_file(pth2.clone());
      assert!(!(pth1.exists() && pth2.exists()));
   }

   #[test]
   #[should_panic]
   fn NaoFuncionaSemArquivos() 
      { coletagem().unwrap(); }

   #[test]
   fn ColetagemDeVariosPipes() {
      cria_arquivos(60);
      let total: usize = 600;
      for indice in 1..=60 {
         let nome = format!("pipe::{:0>2.2}", indice);
         // saco com valores inteiros.
         let mut saco = Primos::with_capacity(total);
         for _ in 1..=total
            { saco.insert(sortear::u64(1..=u64::MAX)); }
         registra(saco, nome.as_str()).unwrap();
      }
      // contabilizando o total de bytes.
      let contabilizacao: u64 = {
         read_dir(RAIZ).unwrap()
         .map(|e| e.unwrap())
         .map(|e| e.metadata().unwrap())
         .map(|m| m.len()).sum()
      };
      assert_eq!(contabilizacao, (total as u64) * 60 * 8);
      let tudo = coletagem().unwrap();
      /* por ser uma seleção aleatória de inserções
       * às vezes a contagem não pode bater, pois 
       * pode ter escolhidos valores iguais. 
       * Rode duas vezes para garantir isso.*/
      assert_eq!(tudo.len(), 60 * total);
   }

   #[test]
   fn DespejoMesmoResultado() {
      cria_arquivos(5);
      let mut conjuntos = vec![
         Primos::with_capacity(205),
         Primos::with_capacity(205),
         Primos::with_capacity(205),
         Primos::with_capacity(205),
         Primos::with_capacity(205)
      ];
      for i in 1..=1_000 
         { conjuntos[i % 5].insert(i as u64); }
      for conjunto in conjuntos.drain(..)
         { despeja(conjunto); }
      let total_de_bytes: u64 = {
         read_dir(RAIZ).unwrap()
         .map(|e| e.unwrap().path())
         .map(|t| t.metadata().unwrap().len())
         .sum()
      };
      assert_eq!(total_de_bytes / 8, 1_000);
      remove_dir_all(RAIZ);
   }
}
