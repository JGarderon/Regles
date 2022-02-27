
use std::io;

use crate::resoudre;
use crate::contexte_resolution;
use crate::grammaire::constructeur::Environnement;

pub fn executer( environnement: &Environnement ) -> Result<(), &'static str> { 
	
	let mut contexte = contexte_resolution( &environnement)?; 

	// let mut buffer = String::new();
	// io::stdin().read_line(&mut buffer)?;

	loop {
		match resoudre( &mut contexte ) { 
			Ok( etat ) => match etat { 
				Some( message ) => println!("message = {:?}", message), 
				None => break 
			} 
			Err( erreur ) => return Err( erreur ) 
		} 
	} 

	Ok( () )

} 



