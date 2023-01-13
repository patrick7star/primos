
/*! Todos os valores que foram gravados
 com a ordem de byte "big endian", serão
 trocados pelo seu inverso, "little endian".
 E vice-versa caso troque de ideia, coloquei
 como "big endian" por uma simples escolha,
 mas quero trocar -- até o porquê é a verdadeira
 arquitetura da máquina.
 */

use super::{
   ultima_insercao_feita as ler_uif,
   ultima_insercao::atualiza_indice_de_insercao
};

// específico para 4 bytes.
type Quatro = [u8; 4];
#[allow(dead_code)]
type Oito = [u8; 8];

// tipo de ordem de armazenamento.
pub enum ByteOrdem { LittleEndian, BigEndian }

fn inverte_array_32bits(bytes: Quatro) -> Quatro {
   let mut invertido = [0u8; 4];
   for k in 0..4 
      { invertido[(4-1)-k] = bytes[k]; }
   return invertido;
}

#[allow(dead_code)]
fn inverte_array64bits(bytes: Oito) -> Oito {
   let mut invertido = [0u8; 8];
   for k in 0..8 
      { invertido[(8-1)-k] = bytes[k]; }
   return invertido;
}

/* troca o dado armazenado no arquivo
 * para a ordem específicada. Para que vai,
 * ou baseado no que está, fica à cargo
 * do programador. */
pub fn inverte_dados_ultima_insercao(ordem: ByteOrdem) {
   let atual = ler_uif().unwrap();
   let inverte: fn(Quatro) -> Quatro;
   inverte = inverte_array_32bits;
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
      funcao = inverte_array_32bits;
      println!("{:?}", funcao(bytes_em_bigendian));
   }
}
