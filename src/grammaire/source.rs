
use std::fs::File; 
use std::io::Read; 
use std::io::Error; 

#[derive(Debug)] 
pub struct Source {
	pub chemin: String, 
	pub contenu: Vec<char>  
} 

impl Source {
	pub fn creer( chemin: String ) -> Result<Self, Error> { 
		let mut curseur = match File::open( &chemin[..] ) {
			Ok( c ) => c, 
			Err( erreur ) => return Err( erreur )  
		}; 
		let mut buffer = String::new(); 
		match curseur.read_to_string( &mut buffer ) {
			Ok( 0 ) => panic!( 
				"Le fichier des règles '{}' semble vide", 
				chemin 
			), 
			Err( erreur ) => return Err( erreur ), 
			_ => () 
		} 
		Ok( Source {
			chemin: chemin, 
			contenu: buffer.chars().collect::<Vec<char>>() 
		} ) 
	}
}


