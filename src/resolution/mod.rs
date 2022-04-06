
use std::env; 

pub mod contexte; 

pub mod asservi; 
use crate::resolution::asservi::executer as resolution_asservi_executer; 

use crate::environnement_construire;
use crate::communs::Erreur; 
use crate::communs::Types; 

use crate::configuration::conf_obtenir; 

pub fn executer() -> Result<(), Erreur> { 

	let environnement = if let Some( chemin ) = conf_obtenir( "regles_source" )? { 
		match chemin { 
			Types::Texte( valeur ) => environnement_construire( valeur )?, 
			_ => return Err( 
				Erreur::creer( 
					"La variable déclarant le chemin source des règles est de type invalide" 
				) 
			) 
		} 
	} else { 
		return Err( 
			Erreur::creer( 
				"La variable déclarant le chemin source des règles n'est pas disponible" 
			) 
		); 
	}; 

	if let Some( chemin ) = conf_obtenir( "resolution_type" )? { 
		match chemin { 
			Types::Texte( valeur ) => match &valeur[..] { 
				"asservi" => resolution_asservi_executer( &environnement ), 
				indetermine => Err( 
					Erreur::creer( "La résolution demandée en appel du programme n'est pas supportée" ) 
				) 
			}, 
			_ => return Err( 
				Erreur::creer( 
					"La variable déclarant le type de résolution est de type invalide" 
				) 
			) 
		} 
	} else { 
		return Err( 
			Erreur::creer( 
				"La variable déclarant le type de résolution n'est pas disponible" 
			) 
		); 
	} 

} 
