
use std::fs::File; 
use std::io::Read; 

use crate::grammaire::Erreur;

#[derive(Debug)] 
pub struct Source {
	pub chemin: String, 
	pub contenu: Vec<char>  
} 

impl Source {
	pub fn creer( chemin: String ) -> Result<Self, Erreur> { 
		let mut curseur = match File::open( &chemin[..] ) {
			Ok( c ) => c, 
			Err( erreur ) => return Err( 
				Erreur::creer_chaine( 
					format!( "La source demandée '{}' n'est pas disponible", chemin ) 
				) 
			) 
		}; 
		let mut buffer = String::new(); 
		match curseur.read_to_string( &mut buffer ) {
			Ok( 0 ) => panic!( 
				"Le fichier des règles '{}' semble vide", 
				chemin 
			), 
			Err( erreur ) => return Err( 
				Erreur::creer_chaine( 
					format!( "La source demandée '{}' a rencontré des erreurs lors de la lecture", chemin ) 
				) 
			), 
			_ => () 
		} 
		Ok( Source {
			chemin: chemin, 
			contenu: buffer.chars().collect::<Vec<char>>() 
		} ) 
	}
}


