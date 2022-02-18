
use std::io::Error; 

pub mod source; 
use crate::grammaire::source::Source; 

macro_rules! espaces_optionnels {
    ($index:ident,$corpus:ident) => {
        match terminal_espace( $index, $corpus ) {
			Ok( taille ) => $index += taille, 
			_ => ()
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


macro_rules! terminal_texte {
	($index:ident,$corpus:ident,$texte:expr,$ajouter:expr) => { 
		let taille = terminal_texte( $index, $texte, $corpus ).unwrap(); 
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
	} 
}

macro_rules! ajouter_lemme_grammatical { 
	($index:ident,$corpus:ident,$lemme:path) => { 
		$corpus.lemmes.push( 
			$lemme( $index ) 
		); 
	} 
} 

#[derive(Debug)] 
enum Lemmes { 
	// terminaux 
	Espaces(usize, String), 
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

fn terminal_texte( mut index: usize, texte: &str, corpus: &mut Corpus ) -> RetourTerminaux { 
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

fn terminal_nom_variable( mut index: usize, corpus: &mut Corpus, ajouter: bool ) -> RetourTerminaux { 
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

fn nonterminal_variable( mut index: usize, corpus: &mut Corpus, ajouter: bool ) -> RetourTerminaux { 
	let origine = index.clone(); 
	espaces_optionnels!( index, corpus ); 
	terminal_texte!( index, corpus, "Variable", false ); 
	ajouter_lemme_grammatical!( index, corpus, Lemmes::Regle_Depart ); 
	espaces_optionnels!( index, corpus ); 

	// le reste 
	// println!("{:?}", corpus.source.contenu[0..index].iter().collect::<String>() );
	Ok( index - origine ) 
} 

pub fn charger() -> Result<Corpus, Error> { 

	let mut corpus = Corpus {
		source: Source::creer( "regles.txt".to_string() ).unwrap(), 
		lemmes: vec!() 
	}; 

	println!( "r = {:?}", nonterminal_variable( 0, &mut corpus, true ) ); 

	Ok( corpus ) 

}


