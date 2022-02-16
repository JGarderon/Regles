
use std::env; 

pub mod traitement; 
use crate::resolution::traitement::executer as resolution_traitement_executer; 

static ENV_RESOLUTION: &'static str = "RESOLUTION_TYPE"; 

pub fn executer() { 

	match env::var( ENV_RESOLUTION ) {
		Ok( resolution ) => match &resolution[..] { 
			"traitement" => resolution_traitement_executer(), 
			indetermine => panic!( 
				"La résolution {:?} n'est pas supportée", 
				indetermine 
			) 
		} 
		Err( erreur ) => panic!( 
			"La variable déclarant la résolution a rencontré une erreur : {:?}", 
			erreur
		) 
	} 

} 
