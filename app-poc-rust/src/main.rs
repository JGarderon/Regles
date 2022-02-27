#![allow(warnings, unused)] 

mod communs; 
use crate::communs::Types; 

mod grammaire; 
use crate::grammaire::constructeur::construire as environnement_construire; 

mod resolution; 
use crate::resolution::contexte::Contexte;
use crate::resolution::contexte::construire as contexte_resolution; 
use crate::resolution::executer as resolution_executer; 

fn resoudre( contexte: &mut Contexte ) -> Result<Option<String>,&'static str> { 
	if contexte.position >= contexte.regles.len() { 
		return Ok( None ); 
	} 
	let regle = &contexte.regles[contexte.position]; 
	for index in regle.clauses.iter() { 
		match &mut contexte.clauses[*index] {
			Types::Appelable( fct, ref mut etat, args ) if *etat == None => {
				println!("fct: {:?} ; args: {:?}", fct, args); 
				*etat = Some( true ); 
			} 
			_ => () 
		}
	} 
	// println!( "{:?}", contexte.clauses); 
	contexte.position += 1; 
	return Ok( None ); 
}


fn main() -> Result<(), &'static str> { 

	println!("--- début ---");

	resolution_executer()?; 
	
	println!("--- fin ---");

	Ok( () ) 

}

