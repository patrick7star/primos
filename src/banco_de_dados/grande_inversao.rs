
/*! Todos os valores que foram gravados
 com a ordem de byte "big endian", serão
 trocados pelo seu inverso, "little endian".
 E vice-versa caso troque de ideia, coloquei
 como "big endian" por uma simples escolha,
 mas quero trocar -- até o porquê é a verdadeira
 arquitetura da máquina.
 */

use super::{
   ultima_insercao_feita as ler_uif, DIR, NOME_BD,
   ultima_insercao::atualiza_indice_de_insercao
};
use std::io::{self, Read, Write};
use std::collections::VecDeque;
use std::fs::OpenOptions;
use std::path::Path;

// específico para 4 bytes.
type Quatro = [u8; 4];
type Oito = [u8; 8];
type Fila = VecDeque<Oito>;

// tipo de ordem de armazenamento.
#[derive(Debug)]
pub enum ByteOrdem { LittleEndian, BigEndian }

fn inverte_array_fixa_32bits(bytes: Quatro) -> Quatro {
   let mut invertido = [0u8; 4];
   for k in 0..4 
      { invertido[(4-1)-k] = bytes[k]; }
   return invertido;
}

fn inverte_array_fixa_64bits(bytes: Oito) -> Oito {
   let mut invertido = [0u8; 8];
   for k in 0..8 
      { invertido[(8-1)-k] = bytes[k]; }
   return invertido;
}

/* troca o dado armazenado no arquivo
 * para a ordem específicada. Para que vai,
 * ou baseado no que está, fica à cargo
 * do programador. */
pub fn inverte_byte_order_de_todos_dados(ordem: ByteOrdem) {
   // se for a mesma que atual, não faz nada.
   match atual_byte_order() {
      ordem => 
         { panic!("já está nesta ByteOrder({:#?})", ordem); } 
      #[allow(unreachable_patterns)]
      _ => 
         { println!("continuando..."); }
   };
   /* arquivo de 'última inserção'. */
   let atual = ler_uif().unwrap();
   let inverte: fn(Quatro) -> Quatro;
   inverte = inverte_array_fixa_32bits;
   let novo_valor_le: u32;

   match ordem {
      ByteOrdem::LittleEndian => {
         // encurtando para caber na tela.
         let bytes = inverte(atual.to_le_bytes());
         novo_valor_le = u32::from_le_bytes(bytes);
      }
      ByteOrdem::BigEndian => {
         // encurtando para caber na tela.
         let bytes = inverte(atual.to_le_bytes());
         novo_valor_le = u32::from_le_bytes(bytes);
      }
   }

   let atualiza: fn(u32);
   atualiza = atualiza_indice_de_insercao;
   atualiza(novo_valor_le);

   // agora faz o mesmo com o arquivo "banco de dados".
   inverte_byteorder_do_bd();

   // o que foi gravado.
   salva_byteorder(ordem);
}

/* pega todos os bytes do arquivo "banco de dados",
 * e arranja-os de 8 em 8, sendo isto posteriormente
 * retornado por meio de uma fila. */
fn filtra_em_containers_de_8_bytes() -> io::Result<Fila> {
   let mut fila = Fila::with_capacity(1_050_000);
   let caminho = Path::new(DIR).join(NOME_BD);
   let mut arquivo = {
      OpenOptions::new()
      .read(true)
      .open(caminho)?
   };

   // pegando 8 bytes de cada vez até o fim do arquivo.
   'ate_o_fim: loop {
      let mut buffer = [0u8; 8];
      match arquivo.read_exact(&mut buffer) {
         Ok(_) => 
            { fila.push_back(buffer); }
         Err(_) => 
            { break 'ate_o_fim; }
      };
   }

   return Ok(fila);
}

const ORDEM_DOS_BYTES: &'static str = "data/byte-order.dat";
/* salva a atual "ordem de byte" imposta aos 
 * valores do banco de dados. Zero representa
 * 'BigEndian', já "duzentos e cinquenta e 
 * cinco", representa o 'LittleEndian'.
 */
fn salva_byteorder(tipo: ByteOrdem) {
   let mut arquivo = {
      OpenOptions::new()
      .create(true)
      .write(true)
      .open(ORDEM_DOS_BYTES)
      .unwrap()
   };
   let mut byte: [u8; 1] = [255 / 2];
   match tipo {
      ByteOrdem::BigEndian =>
         { byte[0] = 0; }
      ByteOrdem::LittleEndian =>
         { byte[0] = 255; }
   };
   arquivo.write(&byte[..]).unwrap();
}

/* inverte, específicamente, o arquivo 
 * "banco de dados", o que guarda quase
 * toda informação relevante. */
fn inverte_byteorder_do_bd() {
   /* arquivo 'banco de dados', com todos
    * dados gerados e coletados. */
   let mut nova_fila = Fila::with_capacity(1_500_000);
   let inverte: fn(Oito) -> Oito;
   inverte = inverte_array_fixa_64bits;

   if let Ok(mut fila) = filtra_em_containers_de_8_bytes() {
      while !fila.is_empty() {
         // retira o primeiro elemento, da primeira fila.
         let atual = fila.pop_front().unwrap();
         // executa inversão.
         let bytes = inverte(atual);
         // adiciona na nova fila..
         nova_fila.push_back(bytes);
      }
   }

   /* reinsere os containers de bytes, más
    * agora invertidos, no * arquivo 
    * novamente.. */
   let caminho = Path::new(DIR).join(NOME_BD);
   let mut arquivo = {
      OpenOptions::new()
      .write(true)
      .open(caminho)
      .unwrap()
   };

   while !nova_fila.is_empty() {
      let container = nova_fila.pop_front().unwrap();
      arquivo.write(&container[..]).unwrap();
   }
}

/* a atual ordem que os bytes de todos dados
 * armazenado no programa, estão armazenados.
 */
fn atual_byte_order() -> ByteOrdem {
   let mut arquivo = {
      OpenOptions::new()
      .read(true)
      .open(ORDEM_DOS_BYTES)
      .unwrap()
   };
   let mut conteudo: [u8; 1] = [255/3];
   arquivo.read(&mut conteudo[..]).unwrap();

   if conteudo[0] == 0
      { ByteOrdem::BigEndian }
   else if conteudo[0] == 255
      { ByteOrdem::LittleEndian }
   else 
      { panic!("[error]arquivo deve ter sido comrropido"); }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
   use super::*;
   use crate::banco_de_dados::ultima_insercao_feita;

   #[test]
   #[ignore="não é um teste, mas um utilitário escondido"]
   fn amostraTrocaOrdemDeByte() {
      let bytes_em_bigendian = {
         ultima_insercao_feita()
         .unwrap()
         .to_be_bytes()
      };
      println!("{:?}", bytes_em_bigendian);
      let funcao: fn(Quatro) -> Quatro;
      // encurtando para caber na tela...
      funcao = inverte_array_fixa_32bits;
      println!("{:?}", funcao(bytes_em_bigendian));
   }

   #[test]
   fn inversao8Bytes() {
      let inverte: fn(Oito) -> Oito;
      inverte = inverte_array_fixa_64bits;
      let numero: u64 = 26;
      let bytes = [0u8, 0, 0, 0, 0, 0, 0, 26];
      assert_eq!(
         dbg!(inverte(bytes)), 
         numero.to_le_bytes()
      );
      let bytes = [0u8, 0, 0, 0, 211, 157, 8, 15];
      assert_eq!(
         dbg!(inverte(bytes)), 
         [15, 8, 157, 211, 0, 0, 0, 0u8]
      );
   }

   #[test]
   fn visualizaAtualByteOrder() 
      { println!("{:#?}", atual_byte_order());}

   #[test]
   #[should_panic]
   fn funcao_IBoTD() {
      let atual = atual_byte_order();
      inverte_byte_order_de_todos_dados(atual);
   }
}
