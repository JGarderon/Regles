
use std::env; 
use std::fs; 

static ENV_VAR_CHEMIN: &'static str = "SOURCE_CHEMIN"; 

pub fn executer() { 
	
	let mut fichiers_source: Vec<String> = vec!(); 
	match env::var( ENV_VAR_CHEMIN ) {
		Ok( chemin ) => match fs::read_dir( &chemin ) {
			Ok( curseur ) => for fichier in curseur { 
				fichiers_source.push( 
					fichier.unwrap().path().display().to_string() 
				); 
			}, 
			Err( _ ) => panic!( 
				"Le chemin {:?} n'est pas accessible", 
				chemin 
			) 
		},
		Err( erreur ) => panic!( 
			"Erreur lors de la lecture de la variable d'environnement {:?} : {:?}", 
			ENV_VAR_CHEMIN, 
			erreur 
		) 
	} 

	println!("{:?}", fichiers_source); 

} 



