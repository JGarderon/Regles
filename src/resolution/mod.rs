
use std::env; 

pub mod contexte; 

pub mod asservi; 
use crate::resolution::asservi::executer as resolution_asservi_executer; 

use crate::environnement_construire;
use crate::communs::Erreur;

static ENV_RESOLUTION: &'static str = "RESOLUTION_TYPE"; 
static ENV_REGLES: &'static str = "REGLES_SOURCE"; 

pub fn executer() -> Result<(), Erreur> { 
	let environnement = match env::var( ENV_REGLES ) { 
		Ok( chemin ) => environnement_construire( chemin )?, 
		Err( _ ) => return Err( 
			Erreur::creer( 
				"La variable déclarant le chemin source des règles n'est pas disponible" 
			) 
		) 
	}; 
	match env::var( ENV_RESOLUTION ) {
		Ok( resolution ) => match &resolution[..] { 
			// "traitement" => resolution_traitement_executer(), 
			"asservi" => resolution_asservi_executer( &environnement ), 
			indetermine => Err( 
				Erreur::creer( "La résolution demandée en appel du programme n'est pas supportée" ) 
			) 
		} 
		Err( _ ) => Err( 
			Erreur::creer( 
				"La variable déclarant la résolution n'est pas disponible" 
			) 
		) 
	} 
} 
