
use crate::grammaire::RetourTerminaux;

use crate::espaces; 
use crate::ajouter_lemme_terminal; 
use crate::ajouter_lemme_grammatical; 
use crate::terminal_cle; 
use crate::nonterminal_regle_partie; 

use crate::grammaire::source::Source; 
use crate::grammaire::Erreur;

#[derive(Debug)] 
pub enum Lemmes { 
	// terminaux 
	Espaces(usize, String), 
	Variable(usize, String), 
	Nombre(usize, String), 
	Texte(usize, String), 
	// non-terminaux 
	Regle_Depart(usize), 
	Regle_Poids(usize, String), 
	Regle_Fin(usize), 
	Variable_Depart(usize), 
	Variable_Fin(usize), 
	Condition_Depart(usize), 
	Condition_Fin(usize), 
	Clause_Depart(usize), 
	Clause_Fin(usize), 
	Appelable_Depart(usize), 
	Appelable_Fin(usize), 
	Conditionnel(usize, String), 
	Renvoi(usize, String), 
	// logique 
	Suite(usize), 
	Et(usize), 
	Ou(usize), 
	// règle 
	Si_Depart(usize), 
	Si_Fin(usize), 
	Alors_Depart(usize), 
	Alors_Fin(usize), 
	Sinon_Depart(usize), 
	Sinon_Fin(usize), 
	Finalement_Depart(usize), 
	Finalement_Fin(usize), 
} 

#[derive(Debug)]
pub struct Corpus { 
	pub source: Source, 
	pub lemmes: Vec<Lemmes> 
} 

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
		return Ok( 0 );  
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
			'-' if origine == index => (), 
			'-' if origine < index => return Err( 
				Erreur::creer( "Expression de nombre en erreur" ) 
			), 
			'0' ... '9' => (), 
			'.' if point == 0 => point += 1, 
			'.' if point > 0 => return Err( 
				Erreur::creer( "Un nombre est en erreur" ) 
			), 
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
		Err( 
			Erreur::creer( "Un texte n'a pas été fermé alors que la source est tarie" ) 
		) 
	} else { 
		Ok( index - origine ) 
	} 
} 

pub fn nonterminal_conditionnel( mut index: usize, corpus: &mut Corpus, _ajouter: bool ) -> RetourTerminaux { 
	let origine = index.clone(); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, "?", false ); 
	ajouter_lemme_terminal!( 
		index, 
		corpus, 
		terminal_texte, 
		Lemmes::Conditionnel, 
		Err( 
			Erreur::creer( "Un nom de condition appelable est obligatoire pour définir un conditionnel" ) 
		) 
	); 
	Ok( index - origine ) 
} 

pub fn nonterminal_renvoi( mut index: usize, corpus: &mut Corpus, _ajouter: bool ) -> RetourTerminaux { 
	let origine = index.clone(); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, "!", false ); 
	ajouter_lemme_terminal!( 
		index, 
		corpus, 
		terminal_texte, 
		Lemmes::Renvoi, 
		Err( 
			Erreur::creer( "Un nom de règle est obligatoire pour définir un renvoi" ) 
		) 
	); 
	Ok( index - origine ) 
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
	ajouter_lemme_terminal!( index, corpus, terminal_variable, Lemmes::Variable ); 
	if index != origine { 
		return Ok( index - origine ); 
	} 
	Ok( 0 ) 
} 

pub fn nonterminal_variable( mut index: usize, corpus: &mut Corpus, ajouter: bool ) -> RetourTerminaux { 
	let origine = index.clone(); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, "Variable", false ); 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Variable_Depart ); 
	espaces!( index, corpus, Err( Erreur::creer( "Le séparateur espace non-trouvé #6" ) ) ); 
	ajouter_lemme_terminal!( 
		index, 
		corpus, 
		terminal_variable, 
		Lemmes::Variable, 
		Err( 
			Erreur::creer( "Un nom de variable à définir est obligatoire" ) 
		) 
	); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, ":", false, Err( Erreur::creer( "Le séparateur n'a pas été trouvé" ) ) ); 
	espaces!( index, corpus, Err( Erreur::creer( "Le séparateur espace non-trouvé  #5" ) ) ); 
	match nonterminal_valeur( index, corpus, true ) { 
		Ok( 0 ) => return Err( 
			Erreur::creer( "Une valeur est attendue lors de la définition d'une variable" ) 
		), 
		Ok( taille ) => index += taille, 
		Err( erreur ) => return Err( erreur ) 
	} 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Variable_Fin ); 
	// println!("{:?}", corpus.source.contenu[0..index].iter().collect::<String>() );
	Ok( index - origine ) 
} 

pub fn nonterminal_appelable( mut index: usize, corpus: &mut Corpus, ajouter: bool ) -> RetourTerminaux { 
	let origine = index.clone(); 
	espaces!( index, corpus ); 
	ajouter_lemme_terminal!( 
		index, 
		corpus, 
		terminal_variable, 
		Lemmes::Variable, 
		Err( 
			Erreur::creer( "Un nom de variable appelable est obligatoire" ) 
		) 
	); 
	espaces!( index, corpus ); 
	if terminal_cle!( index, corpus, "(" ) == 0 { 
		return Err( 
			Erreur::creer( "L'appel ouvrant n'est pas trouvé" ) 
		); 
	} else { 
		index += 1; 
	}
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Appelable_Depart ); 
	loop {
		espaces!( index, corpus ); 
		match nonterminal_valeur( index, corpus, true ) { 
			Ok( 0 ) => break, 
			Ok( taille ) => index += taille, 
			Err( erreur ) => break 
		} 
		espaces!( index, corpus ); 
		if terminal_cle!( index, corpus, "," ) == 0 { 
			break; 
		} else { 
			index += 1; 
		}
	} 
	espaces!( index, corpus ); 
	if terminal_cle!( index, corpus, ")" ) == 0 { 
		return Err( 
			Erreur::creer( "L'appel fermant n'est pas trouvé" ) 
		); 
	} else {
		index += 1; 
	}
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Appelable_Fin ); 
	// println!("{:?}", corpus.source.contenu[0..index].iter().collect::<String>() ); 
	Ok( index - origine ) 
} 

pub fn nonterminal_conditionnel_ou_appelable( mut index: usize, corpus: &mut Corpus, _ajouter: bool ) -> RetourTerminaux { 
	match nonterminal_conditionnel( index, corpus, true ) { 
		Ok( 0 ) => (), 
		Ok( taille ) => return Ok( taille ), 
		Err( erreur ) => return Err( erreur.empiler( "Appelable 'nonterminal_conditionnel_ou_appelable'" ) ) 
	} 
	match nonterminal_appelable( index, corpus, true ) { 
		Ok( taille ) => return Ok( taille ), 
		Err( erreur ) => return Err( erreur.empiler( "Appelable 'nonterminal_conditionnel_ou_appelable'" ) ) 
	} 
} 

pub fn nonterminal_condition( mut index: usize, corpus: &mut Corpus, ajouter: bool ) -> RetourTerminaux { 
	let origine = index.clone(); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, "Condition", false ); 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Condition_Depart ); 
	espaces!( index, corpus, Err( Erreur::creer( "Le séparateur espace non-trouvé #2" ) ) ); 
	ajouter_lemme_terminal!( 
		index, 
		corpus, 
		terminal_texte, 
		Lemmes::Texte, 
		Err( 
			Erreur::creer( "Un nom de condition à définir est obligatoire" ) 
		) 
	); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, ":", false, Err( Erreur::creer( "Le séparateur n'a pas été trouvé") ) ); 
	espaces!( index, corpus, Err( Erreur::creer( "Le séparateur espace non-trouvé #1" ) ) ); 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Clause_Depart ); 
	match nonterminal_conditionnel_ou_appelable( index, corpus, true ) { 
		Ok( 0 ) => return Err( 
			Erreur::creer( "Une condition est sans aucune clause appelable" )
		), 
		Ok( taille ) => index += taille, 
		Err( erreur ) => return Err( erreur.empiler( "Appelable 'nonterminal_condition'" ) ) 
	} 
	loop { 
		espaces!( index, corpus ); 
		if terminal_cle!( index, corpus, "et" ) == 0 { 
			if terminal_cle!( index, corpus, "ou" ) == 0 { 
				break; 
			} else { 
				ajouter_lemme_grammatical!( index, corpus, Lemmes::Ou ); 
				index += 2; 
			}
		} else { 
			ajouter_lemme_grammatical!( index, corpus, Lemmes::Et ); 
			index += 2; 
		} 
		espaces!( index, corpus ); 
		match nonterminal_appelable( index, corpus, true ) { 
			Ok( 0 ) => return Err( 
				Erreur::creer( "Un opérateur logique de condition est sans aucune clause appelable" ) 
			), 
			Ok( taille ) => index += taille, 
			Err( erreur ) => return Err( 
				erreur.empiler( "Appelable nonterminal_condition" )
			) 
		} 
	} 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Clause_Fin ); 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Condition_Fin ); 
	// println!("{:?}", corpus.source.contenu[0..index].iter().collect::<String>() );
	Ok( index - origine ) 
} 

pub fn nonterminal_regle_partie_si( mut index: usize, corpus: &mut Corpus, _ajouter: bool ) -> RetourTerminaux { 
	let origine = index.clone(); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, "Si", false, Err( Erreur::creer( "L'opérateur de règle 'Si' est obligatoire" ) ) ); 
	match nonterminal_conditionnel( index, corpus, true ) { 
		Ok( 0 ) => (), 
		Ok( taille ) => index += taille, 
		Err( erreur ) => return Err( erreur.empiler( "Appelable 'nonterminal_regle_partie_si'" ) ) 
	} 
	loop { 
		espaces!( index, corpus ); 
		if terminal_cle!( index, corpus, "et" ) == 0 { 
			if terminal_cle!( index, corpus, "ou" ) == 0 { 
				break; 
			} else { 
				ajouter_lemme_grammatical!( index, corpus, Lemmes::Ou ); 
				index += 2; 
			}
		} else { 
			ajouter_lemme_grammatical!( index, corpus, Lemmes::Et ); 
			index += 2; 
		} 
		espaces!( index, corpus ); 
		match nonterminal_conditionnel( index, corpus, true ) { 
			Ok( 0 ) => return Err( Erreur::creer( "Une condition doit être ajouté après un opérateur 'Si'" ) ), 
			Ok( taille ) => index += taille, 
			Err( erreur ) => return Err( erreur.empiler( "Appelable 'nonterminal_regle_partie_si'" ) ) 
		} 
	} 
	Ok( index - origine ) 
} 

pub fn nonterminal_regle( mut index: usize, corpus: &mut Corpus, ajouter: bool ) -> RetourTerminaux { 
	let origine = index.clone(); 
	espaces!( index, corpus ); 
	if terminal_cle!( index, corpus, "Règle" ) == 0 { 
		return Ok( 0 ); 
	} 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Regle_Depart ); 
	index += 5; 
	espaces!( index, corpus, Err( Erreur::creer( "Un séparateur est obligatoire à la déclaration d'une règle" ) ) ); 
	ajouter_lemme_terminal!( 
		index, 
		corpus, 
		terminal_texte, 
		Lemmes::Texte, 
		Err( 
			Erreur::creer( "Un nom de règle à définir est obligatoire" ) 
		) 
	); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, "(", false, Err( Erreur::creer( "L'ouverture de poids n'est pas trouvée") ) ); 
	ajouter_lemme_terminal!( 
		index, 
		corpus, 
		terminal_nombre, 
		Lemmes::Regle_Poids, 
		Err( Erreur::creer( "Un poids de règle est obligatoire" ) ) 
	); 
	terminal_cle!( index, corpus, ")", false, Err( Erreur::creer( "La fermeture de poids n'est pas trouvée") ) ); 
	espaces!( index, corpus ); 
	terminal_cle!( index, corpus, ":", false, Err( Erreur::creer( "Le séparateur de règle n'a pas été trouvé") ) ); 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Si_Depart ); 
	index += nonterminal_regle_partie_si( index, corpus, true )?; 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Si_Fin ); 
	nonterminal_regle_partie!( index, corpus, "Alors", 5, false, Lemmes::Alors_Depart, Lemmes::Alors_Fin ); 
	nonterminal_regle_partie!( index, corpus, "Sinon", 5, false, Lemmes::Sinon_Depart, Lemmes::Sinon_Fin ); 
	nonterminal_regle_partie!( index, corpus, "Finalement", 10, false, Lemmes::Finalement_Depart, Lemmes::Finalement_Fin ); 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Regle_Fin ); 
	Ok( index - origine ) 
} 

pub fn charger( chemin: String ) -> Result<Corpus, Erreur> { 
	let mut corpus = Corpus {
		source: match Source::creer( chemin ) { 
			Ok( source ) => source, 
			Err( _ ) => return Err( 
				Erreur::creer( "le fichier source n'est pas disponible" )
			) 
		}, 
		lemmes: vec!() 
	}; 
	let mut index: usize = 0; 
	let mut stop: bool = false; 
	while index < corpus.source.contenu.len() && stop == false { 
		stop = true; 
		let taille = nonterminal_variable( index, &mut corpus, true )?; 
		if taille > 0 { 
			// eprintln!( "Définition 'Variable' trouvée ({})", index ); 
			stop = false; 
			index += taille; 
			continue; 
		} 
		let taille = nonterminal_condition( index, &mut corpus, true )?; 
		if taille > 0 { 
			// eprintln!( "Définition 'Condition' trouvée ({})", index ); 
			stop = false; 
			index += taille; 
			continue; 
		} 
		let taille = nonterminal_regle( index, &mut corpus, true )?; 
		if taille > 0 { 
			// eprintln!( "Définition 'Règle' trouvée ({})", index ); 
			stop = false; 
			index += taille; 
			continue; 
		}  
	} 
	Ok( corpus ) 
} 


