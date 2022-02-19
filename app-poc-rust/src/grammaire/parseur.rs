
use crate::espaces; 
use crate::ajouter_lemme_terminal; 
use crate::ajouter_lemme_grammatical; 
use crate::terminal_cle; 

use crate::grammaire::source::Source; 

#[derive(Debug)] 
pub enum Lemmes { 
	// terminaux 
	Espaces(usize, String), 
	Variable(usize, String), 
	Nombre(usize, String), 
	Texte(usize, String), 
	// non-terminaux 
	Regle_Depart(usize), 
	Regle_Fin(usize), 
	Condition_Depart(usize), 
	Condition_Fin(usize), 
	Clause_Depart(usize), 
	Clause_Fin(usize), 
} 

#[derive(Debug)]
pub struct Corpus { 
	pub source: Source, 
	pub lemmes: Vec<Lemmes> 
} 

type RetourTerminaux = Result<usize, &'static str>;

fn terminal_espace( mut index: usize, corpus: &mut Corpus ) -> RetourTerminaux {
	let max = corpus.source.contenu.len(); 
	let origine = index.clone(); 
	while index < max { 
		match corpus.source.contenu[index] { 
			' ' 
				| '\t' 
				| '\r' 
				| '\n' => (), 
			_ => break
		} 
		index += 1; 
	} 
	Ok( index - origine ) 
} 

fn terminal_cle( mut index: usize, texte: &str, corpus: &mut Corpus ) -> RetourTerminaux { 
	let taille = texte.chars().count(); 
	if ( index + taille ) >= corpus.source.contenu.len() { 
		return Err( "terminal_texte source insuffisante" ); 
	} 
	if corpus.source.contenu[index..index+taille].iter().eq( 
		&texte.chars().collect::<Vec<char>>() 
	) {
		Ok( taille ) 
	} else {
		Ok( 0 ) 
	}
} 

fn terminal_variable( mut index: usize, corpus: &mut Corpus ) -> RetourTerminaux { 
	let max = corpus.source.contenu.len(); 
	let origine = index.clone(); 
	while index < max { 
		match corpus.source.contenu[index] { 
			'a' ... 'z' 
				| 'A' ... 'Z' 
				| '0' ... '9' 
				| '.' 
				| '_'  => (), 
			_ => break 
		} 
		index += 1; 
	} 
	Ok( index - origine ) 
} 

fn terminal_nombre( mut index: usize, corpus: &mut Corpus ) -> RetourTerminaux { 
	let max = corpus.source.contenu.len(); 
	let origine = index.clone(); 
	let mut point: usize = 0; 
	while index < max { 
		match corpus.source.contenu[index] { 
			'0' ... '9' => (), 
			'.' if point == 0 => point += 1, 
			'.' if point > 0 => return Err( "Un nombre est en erreur" ), 
			_ => break 
		} 
		index += 1; 
	} 
	Ok( index - origine ) 
} 

fn terminal_texte( mut index: usize, corpus: &mut Corpus ) -> RetourTerminaux { 
	let max = corpus.source.contenu.len(); 
	let origine = index.clone(); 
	if index < max && corpus.source.contenu[index] != '"' { 
		return Ok( 0 ); 
	} 
	index += 1; 
	let mut ouvert = true; 
	while index < max && ouvert { 
		match corpus.source.contenu[index] { 
			'"' => ouvert = false, 
			_ => () 
		} 
		index += 1; 
	} 
	if ouvert { 
		Err( "Un texte n'a pas été fermé alors que la source est tarie" ) 
	} else { 
		Ok( index - origine ) 
	} 
} 

pub fn nonterminal_valeur( mut index: usize, corpus: &mut Corpus, _ajouter: bool ) -> RetourTerminaux { 
	let origine = index.clone(); 
	espaces!( index, corpus ); 
	ajouter_lemme_terminal!( index, corpus, terminal_nombre, Lemmes::Nombre ); 
	if index != origine { 
		return Ok( index - origine ); 
	}
	ajouter_lemme_terminal!( index, corpus, terminal_texte, Lemmes::Texte ); 
	if index != origine { 
		return Ok( index - origine ); 
	} 
	Ok( 0 ) 
} 

pub fn nonterminal_variable( mut index: usize, corpus: &mut Corpus, ajouter: bool ) -> RetourTerminaux { 
	let origine = index.clone(); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, "Variable", false ); 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Regle_Depart ); 
	espaces!( index, corpus, Err( "Le séparateur espace non-trouvé" ) ); 
	ajouter_lemme_terminal!( 
		index, 
		corpus, 
		terminal_variable, 
		Lemmes::Variable, 
		Err( "Un nom de variable est obligatoire" ) 
	); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, ":", false, Err("Le séparateur n'a pas été trouvé") ); 
	espaces!( index, corpus, Err( "Le séparateur espace non-trouvé" ) ); 
	match nonterminal_valeur( index, corpus, true ) {
		Ok( 0 ) => return Err( "Le séparateur espace non-trouvé" ), 
		Ok( taille ) => index += taille, 
		Err( erreur ) => return Err( erreur ) 
	} 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Regle_Fin ); 
	// println!("{:?}", corpus.source.contenu[0..index].iter().collect::<String>() );
	Ok( index - origine ) 
} 

pub fn nonterminal_condition( mut index: usize, corpus: &mut Corpus, ajouter: bool ) -> RetourTerminaux { 
	let origine = index.clone(); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, "Condition", true ); 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Condition_Depart ); 
	espaces!( index, corpus, Err( "Le séparateur espace non-trouvé" ) ); 
	ajouter_lemme_terminal!( 
		index, 
		corpus, 
		terminal_texte, 
		Lemmes::Texte, 
		Err( "Un nom de variable est obligatoire" ) 
	); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, ":", false, Err("Le séparateur n'a pas été trouvé") ); 
	espaces!( index, corpus, Err( "Le séparateur espace non-trouvé" ) ); 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Clause_Depart ); 
	
	// println!("{:?}", corpus.source.contenu[0..index].iter().collect::<String>() );
	Ok( index - origine ) 
} 


