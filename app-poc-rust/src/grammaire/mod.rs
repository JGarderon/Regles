
use std::io::Error; 

pub mod source; 
use crate::grammaire::source::Source; 

macro_rules! espaces { 
    ($index:ident,$corpus:ident) => {
        match terminal_espace( $index, $corpus ) {
        	Ok( taille ) => $index += taille, 
        	_ => () 
		} 
    }; 
    ($index:ident,$corpus:ident,$erreur:expr) => {
        match terminal_espace( $index, $corpus ) {
        	Ok( 0 ) => return $erreur, 
			Ok( taille ) => $index += taille, 
			retour_erreur @ _ => return retour_erreur 
		} 
    }
} 

macro_rules! espaces_optionnels_enregistres { 
    ($index:ident,$corpus:ident) => { 
        match terminal_espace( $index, $corpus ) {
			Ok( taille ) => { 
				$corpus.lemmes.push( 
					Lemmes::Espaces( 
						$index, 
						$corpus.source.contenu[$index..$index+taille].iter().collect::<String>() 
					) 
				); 
				$index += taille; 
			} 
			_ => ()
		}  
    }
}

macro_rules! terminal_cle {
	($index:ident,$corpus:ident,$texte:expr,$ajouter:expr) => { 
		let taille = terminal_cle( $index, $texte, $corpus ).unwrap(); 
		if taille == 0 { 
			return Ok( 0 ) 
		} else { 
			if $ajouter { 
				$corpus.lemmes.push( 
					Lemmes::Texte( 
						$index, 
						$corpus.source.contenu[$index..$index+taille].iter().collect::<String>() 
					) 
				); 
			} 
			$index += taille; 
		} 
	}; 
	($index:ident,$corpus:ident,$texte:expr,$ajouter:expr,$erreur:expr) => { 
		let taille = terminal_cle( $index, $texte, $corpus ).unwrap(); 
		if taille == 0 { 
			return $erreur 
		} else { 
			if $ajouter { 
				$corpus.lemmes.push( 
					Lemmes::Texte( 
						$index, 
						$corpus.source.contenu[$index..$index+taille].iter().collect::<String>() 
					) 
				); 
			} 
			$index += taille; 
		} 
	} 
} 

macro_rules! ajouter_lemme_grammatical { 
	($index:ident,$corpus:ident,$lemme:path) => { 
		$corpus.lemmes.push( 
			$lemme( $index ) 
		); 
	} 
} 

macro_rules! ajouter_lemme_terminal { 
	($index:ident,$corpus:ident,$fct:path,$lemme:path) => { 
		match $fct ( 
			$index, 
			$corpus 
		) { 
			Ok( taille ) if taille > 0 => { 
				$corpus.lemmes.push( 
					$lemme( 
						$index, 
						$corpus.source.contenu[$index..$index+taille].iter().collect::<String>()
					)
				); 
				$index += taille; 
			} 
			_ => () 
		} 
	}; 
	($index:ident,$corpus:ident,$fct:path,$lemme:path,$erreur:expr) => { 
		match $fct ( 
			$index, 
			$corpus 
		) { 
			Ok( taille ) if taille > 0 => { 
				$corpus.lemmes.push( 
					$lemme( 
						$index, 
						$corpus.source.contenu[$index..$index+taille].iter().collect::<String>()
					)
				); 
				$index += taille; 
			} 
			_ => return $erreur 
		} 
	} 
} 

#[derive(Debug)] 
enum Lemmes { 
	// terminaux 
	Espaces(usize, String), 
	Variable(usize, String), 
	Texte(usize, String), 
	// non-terminaux 
	Regle_Depart(usize), 
	Regle_Fin(usize) 
} 

#[derive(Debug)]
pub struct Corpus { 
	source: Source, 
	lemmes: Vec<Lemmes> 
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

fn nonterminal_variable( mut index: usize, corpus: &mut Corpus, ajouter: bool ) -> RetourTerminaux { 
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
	espaces!( index, corpus, Err( "Le séparateur espace non-trouvé" ) ); 
	terminal_cle!( index, corpus, ":", false, Err("Le séparateur n'a pas été trouvé") ); 
	espaces!( index, corpus, Err( "Le séparateur espace non-trouvé" ) ); 
	ajouter_lemme_terminal!( 
		index, 
		corpus, 
		terminal_texte, 
		Lemmes::Texte, 
		Err( "Un texte est obligatoire" ) 
	); 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Regle_Fin ); 
	// println!("{:?}", corpus.source.contenu[0..index].iter().collect::<String>() );
	Ok( index - origine ) 
} 

pub fn charger() -> Result<Corpus, Error> { 

	let mut corpus = Corpus {
		source: Source::creer( "regles.txt".to_string() ).unwrap(), 
		lemmes: vec!() 
	}; 

	let i = nonterminal_variable( 0, &mut corpus, true ); 
	println!( "r (1) = {:?}", i ); 
	let i = nonterminal_variable( i.unwrap(), &mut corpus, true ); 
	println!( "r (2) = {:?}", i ); 

	Ok( corpus ) 

}


