
type RetourTerminaux = Result<usize, &'static str>;

pub mod source; 
use crate::grammaire::source::Source; 

pub mod parseur; 
use crate::grammaire::parseur::Corpus; 
use crate::grammaire::parseur::nonterminal_variable; 
use crate::grammaire::parseur::nonterminal_condition; 
use crate::grammaire::parseur::nonterminal_regle; 

pub mod constructeur; 

#[macro_export] 
macro_rules! nonterminal_regle_partie {
	($index:ident,$corpus:ident,$cle:expr,$taille:expr,$obligatoire:expr,$lemme_depart:path,$lemme_fin:path) => {
		espaces!( $index, $corpus ); 
		if terminal_cle!( $index, $corpus, $cle ) == 0 { 
			if $obligatoire == true {
				return Err( "Une clé est obligatoire" ); 
			} 
		} else {
			ajouter_lemme_grammatical!( $index, $corpus, $lemme_depart ); 
			$index += $taille; 
			match nonterminal_appelable( $index, $corpus, true ) { 
				Ok( 0 ) => return Err( "Aucune clause appelable après une clé" ), 
				Ok( taille ) => $index += taille, 
				Err( erreur ) => return Err( erreur ) 
			} 
			loop { 
				espaces!( $index, $corpus ); 
				match terminal_cle( $index, ",", $corpus ) { 
					Ok( 0 ) | Err( _ ) => break, 
					Ok( taille ) => { 
						ajouter_lemme_grammatical!( $index, $corpus, Lemmes::Suite ); 
						$index += 1; 
					} 
				} 
				espaces!( $index, $corpus ); 
				match nonterminal_appelable( $index, $corpus, true ) { 
					Ok( 0 ) => return Err( "Aucune clause appelable dans une clé après un opérateur logique" ), 
					Ok( taille ) => $index += taille, 
					Err( erreur ) => return Err( erreur ) 
				} 
			} 
			ajouter_lemme_grammatical!( $index, $corpus, $lemme_fin ); 
		}
	}
}


#[macro_export]
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

#[macro_export]
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

#[macro_export]
macro_rules! terminal_cle {
	($index:ident,$corpus:ident,$texte:expr) => { 
		terminal_cle( $index, $texte, $corpus ).unwrap(); 
	}; 
	($index:ident,$corpus:ident,$texte:expr,$ajouter:expr) => { 
		match terminal_cle( $index, $texte, $corpus ) {
			Ok( taille ) => if taille == 0 { 
				return Ok( 0 ); 
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
			Err( _ ) => return Ok( 0 ) 
		} 
	}; 
	($index:ident,$corpus:ident,$texte:expr,$ajouter:expr,$erreur:expr) => { 
		match terminal_cle( $index, $texte, $corpus ) {
			Ok( taille ) => if taille == 0 { 
				return $erreur; 
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
			Err( _ ) => return $erreur 
		} 
	} 
} 

#[macro_export]
macro_rules! ajouter_lemme_grammatical { 
	($index:ident,$corpus:ident,$lemme:path) => { 
		$corpus.lemmes.push( 
			$lemme( $index ) 
		); 
	} 
} 

#[macro_export]
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







