
use crate::Contexte; 

use crate::contexte_resolution;
use crate::grammaire::constructeur::Environnement; 

use crate::communs::Types; 
use crate::communs::Dialogue; 

fn resoudre( contexte: &mut Contexte, dialogue: &mut Dialogue ) -> Result<Option<String>,&'static str> { 
	if contexte.position >= contexte.regles.len() { 
		return Ok( None ); 
	} 
	let regle = &contexte.regles[contexte.position]; 
	for index in regle.clauses.iter() { 
		match &mut contexte.clauses[*index] {
			Types::Appelable( fct, ref mut etat, args ) if *etat == None => *etat = Some( 
				dialogue.soumettre( &fct, &args )? 
			), 
			_ => () 
		}
	} 
	contexte.position += 1; 
	return Ok( None ); 
}

pub fn executer( environnement: &Environnement ) -> Result<(), &'static str> { 
	let mut contexte = contexte_resolution( &environnement)?; 
	let mut dialogue: Dialogue = Dialogue::creer();  
	loop { 
		match resoudre( &mut contexte, &mut dialogue ) { 
			Ok( etat ) => match etat { 
				Some( message ) => println!("message = {:?}", message), // à terminer 
				None => break 
			} 
			Err( erreur ) => return Err( erreur ) 
		} 
	} 
	Ok( () ) 
} 



