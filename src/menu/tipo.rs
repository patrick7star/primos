

/* refatoração: os enumeradores e funções
 * ou métodos referentes a eles serão 
 * deslocados para aqui.
 */

use std::str::FromStr;
use std::time::Duration;
use std::fmt::Error;
//use std::ops::RangeInclusive;

#[derive(Debug)]
pub enum Funcao { 
   /* tenta potencializar todo poder computacional
    * disponível, usando de todas ferramentas de
    * paralelismo disponível. */
   Chamada, Processo, 
   Inversao, Deleta, 
   /* alternativa ao primeiro, usa só 
    * subprocessos para alavancar o processamento,
    * pois não faz diferença lança-lô através
    * de threads juntos ao mesmo tempo. */
   PoderTotal, NovaChamada
}

// argumentos comuns ao executar o programa.
#[derive(Debug)]
pub enum Argumentos {
   // querendo a info sobre todo banco de dados.
   Infomarcao,
   // fazer uma varredura padrão(qtd. já determinada).
   Procura,
   // faz varredura por dado tempo demandado.
   ProcuraTempo(f32),
   // faz varredura por dada quantia demandada.
   ProcuraQtd(u64),
   // 'arm' para futuro backup, hoje apenas cópia para novo BD.
   Backup,
   // ajuda, info de como o programa funciona.
   Ajuda,
   /* não aberto ao público apenas para 
    * ajudar na codificação. */
   Privado(Funcao)
}

/// tem os acrônimos simbolizando a escala de tempo.
#[allow(dead_code)]
fn argumento_tempo_valido(arg:&str) -> bool {
   let arg = arg.to_ascii_uppercase();
   let tem_acronimo = {
      arg.contains("min") ||
      arg.contains("h") || 
      arg.contains("seg")
   };
   
   // parte do presuposto que não.
   let mut tem_parte_numerica = false;
   // nenhum símbolo forasteiro.
   let mut nao_simbolo_forasteiro = true;
   // varre cada caractére.
   for ch in arg.chars() {
      if ch.is_ascii_digit() {
         tem_parte_numerica = true;
      }
      // têm que ser caractére e, ser um
      // dos específicos abaixo.
      else if ch.is_ascii_alphanumeric() &&
         (ch == 'm' || ch == 'i' ||
         ch == 's' || ch == 'e' ||
         ch == 'g' || ch == 'h' ||
         ch == 'n') { }
      else {
         nao_simbolo_forasteiro = false;
      }
   }

   // sendo as três premissas verdadeiras, então
   // a string é válida.
   tem_parte_numerica && 
   nao_simbolo_forasteiro && 
   tem_acronimo
}

/* string, que contém caractéres, porém
 * não apenas eles, deve ter uma contagem
 * correta. */
fn n_caracteres(string: &str, n: usize) -> bool {
   string.chars()
   .filter(|c| c.is_alphabetic())
   .count() == n
}

/* verifica se a string têm uma parte númerica
 * representando o tempo demandado, e, se 
 * tem um peso válido, ou seja, uma slice-string
 * valuando aquela parte númerica num total
 * de segundos. */
fn argumento_valido(arg: &str) -> bool {
   let tem_acronimo = {
      arg.contains("minuto") && n_caracteres(arg, 6)  ||
      arg.contains("hora") && n_caracteres(arg, 4) || 
      arg.contains("segundo") && n_caracteres(arg, 7) ||
      arg.contains("m") && n_caracteres(arg, 1) ||
      arg.contains("s") && n_caracteres(arg, 1) ||
      arg.contains("h") && n_caracteres(arg, 1) ||
      arg.contains("minutos") && n_caracteres(arg, 7)  ||
      arg.contains("horas") && n_caracteres(arg, 5) || 
      arg.contains("segundos") && n_caracteres(arg, 8) ||
      arg.contains("min") && n_caracteres(arg, 3) ||
      arg.contains("seg") && n_caracteres(arg, 3)
   };
   
   // parte do presuposto que não.
   let tem_parte_numerica = {
      arg.chars()
      .filter(char::is_ascii_digit)
      .count() >= 1
   };

   tem_parte_numerica && tem_acronimo
}

/// parte do pressuposto de apenas números.
fn apenas_numerica(arg:&str) -> bool {
   for alg in arg.chars() {
      if !alg.is_ascii_digit() {
         return false;
      }
   }
   // se chega até aqui é numérica a string.
   return true;
}

/** retorna um tipo de enum mais adequado 
 para os dados argumentos passados. */
pub fn transforma(argumentos:&Vec<String>) -> Argumentos {
   let total = argumentos.len();
   /* se nenhum argumento for cedido, mostrar opção 'ajuda'.
    * incompleta, retorna pedido de ajuda. */
   if total == 1 {
      return Argumentos::Ajuda;
   } else if total == 2 {
      // para melhorar legibilidade.
      let opcao = argumentos[1].clone();
      // um argumento em terminal(apenas opção).
      if argumentos[1] == "info" 
         { return Argumentos::Infomarcao; }
      else if argumentos[1] == "procura" 
         { return Argumentos::Procura; }
      else if argumentos[1] == "ajuda" 
         { return Argumentos::Ajuda; }
      // opções privadas, inacessíveis ao usuário.
      else if opcao == "inverte-última-inserção" 
         { return Argumentos::Privado(Funcao::Inversao); }
      // continuação das opções "públicas".
      else if argumentos[1] == "backup"
         { return Argumentos::Backup; }
      else { return Argumentos::Ajuda; }
   } else if total == 3 {
      // para melhorar legibilidade.
      let opcao = &argumentos[1];
      let arg = &argumentos[2];
      // proposições:
      let texto_de_tempo = argumento_valido(arg.as_str());
      let valor_numerico = apenas_numerica(arg.as_str());
      // dois argumentos(opção + argumento):
      if opcao == "tempo" && texto_de_tempo {
         let valor = argumento_em_tempo(arg.as_str()).unwrap();
         return Argumentos::ProcuraTempo(valor.as_secs_f32());
      } else if opcao == "quantia" && valor_numerico {
         let valor:u64 = arg.parse().unwrap();
         return Argumentos::ProcuraQtd(valor);
      } else if opcao  == "ignição" { 
         return Argumentos::Privado(Funcao::Chamada);
      } else if opcao == "varre" {
         return Argumentos::Privado(Funcao::Processo);
      } else if opcao == "processa" {
         return Argumentos::Privado(Funcao::PoderTotal);
      } else if opcao == "nova-ignição" {
         return Argumentos::Privado(Funcao::NovaChamada);
      } else 
         { return Argumentos::Ajuda; }
   } else if total == 4 {
      let opcao = &argumentos[1];
      if opcao == "função-deleta-caminho" {
         return Argumentos::Privado(Funcao::Deleta); 
      } else 
         { return Argumentos::Ajuda; }
   }else { 
      // três ou mais argumentos, ainda não implementado...
      println!("só um argumento permitido por vez!");
      return Argumentos::Ajuda;
   }
}

/// partindo do presuposto que está neste formato...
fn argumento_em_tempo(string:&str) -> Result<Duration, Error> {
   let e_hora = {
      string.contains("h") ||
      string.contains("hora")
   };
   let e_minuto = {
      string.contains("min") ||
      string.contains("m") ||
      string.contains("minuto")
   };
   
   // que tipo é o valor.
   let tipo:&str = {
      if e_hora 
         { "h" }
      else if e_minuto 
         { "m" }
      else 
         { "s" }
   };
   
   // extraindo parte numérica...
   let numero = match string.find(tipo) {
      Some(p) => string.get(0..p).unwrap(),
      None => 
         { return Err(Error); }
   };
   
   // convertendo para floating...
   let tempo:f32 = f32::from_str(numero).unwrap();
   
   // retorno mutiplicado pelo peso adequado.
   if e_minuto {
      // converte minutos em segundos.
      Ok(Duration::from_secs_f32(60.0 * tempo))
   } else if e_hora {
      // converte horas em segundos.
      Ok(Duration::from_secs_f32(3600.0 * tempo))
   } else 
      { Ok(Duration::from_secs_f32(tempo)) }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   #[allow(non_snake_case)]
   fn ArgumentoValido() {
      // horas:
      assert!(argumento_valido("12h"));
      assert!(argumento_valido("12horas"));
      // minutos:
      assert!(argumento_valido("3minutos"));
      assert!(argumento_valido("24min"));
      assert!(argumento_valido("42m"));
      // segundos:
      assert!(argumento_valido("57s"));
      assert!(argumento_valido("99seg"));
      assert!(argumento_valido("18segundos"));
      // versões no singular:
      assert!(argumento_valido("1hora"));
      assert!(argumento_valido("1minuto"));
      assert!(argumento_valido("1segundo"));
      // inválidos:
      assert!(!argumento_valido("1hr"));
      assert!(!argumento_valido("1hour"));
      assert!(!argumento_valido("24dias"));
      assert!(!argumento_valido("4meses"));
      assert!(!argumento_valido("28segs"));
      assert!(!argumento_valido("37mins"));
   }

}
