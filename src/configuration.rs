
use std::env; 
// use std::sync::{Arc, Mutex}; # prochainement, pour la partie 'fil' ? 

use std::collections::HashMap; 
use crate::communs::Erreur; 
use crate::Types;

static mut CONFIGURATION: Option<Configuration> = None; 
static mut CONF_ENV_OBLIGATOIRES: Option<HashMap<String, String>> = None; 
static mut CONF_VARS_OBLIGATOIRES: Option<HashMap<String, String>> = None; 

#[derive(Debug)] 
pub struct Configuration( HashMap<String,Types> ); 

impl Configuration {
	pub fn creer() -> Result<(), Erreur> { 
		unsafe { 
			if CONFIGURATION.is_some() { 
				return Err( 
					Erreur::creer( 
						"La configuration a déjà été faite : impossible de relancer" 
					) 
				); 
			} 
			let env_obligatoires = vec!( 
				( "RESOLUTION_TYPE".to_string(), "resolution_type".to_string() ), 
				( "REGLES_SOURCE".to_string(), "regles_source".to_string() ),  
			).into_iter().collect::<HashMap<_, _>>(); 
			let args_optionnels = vec!( 
				( "resolution".to_string(), "resolution_type".to_string() ), 
				( "regles".to_string(), "regles_source".to_string() ),  
			).into_iter().collect::<HashMap<_, _>>(); 
			let mut conf: HashMap<String,Types> = HashMap::new(); 
			for item in env_obligatoires.iter() { 
				match env::var( item.0 ) { 
					Ok( valeur ) => { 
						conf.insert( 
							item.1.to_string(), 
							Types::Texte( valeur ) 
						); 
					} 
					Err( _ ) => () 
				}; 
			} 
			let mut cle_precedent: Option<String> = None; 
			let mut valeur_suivant = false; 
			for argument in env::args().skip( 1 ) { 
				if valeur_suivant { 
					conf.insert( 
						cle_precedent.unwrap(), // à voir pour sécuriser davantage à l'avenir 
						Types::Texte( 
							argument 
						) 
					); 
					cle_precedent = None; 
					valeur_suivant = false; 
				} else { 
					if argument.starts_with("--") { 
						valeur_suivant = true; 
						let cle = argument.strip_prefix("--").unwrap().to_string(); 
						match args_optionnels.get( &cle ) { 
							Some( valeur ) => cle_precedent = Some( valeur.clone() ), 
							None => return Err( 
								Erreur::creer_chaine( 
									format!( 
										"La valeur '{}' n'a pas de sens en ligne de commande à cette position", 
										argument 
									) 
								) 
							)
						} 
					} else { 
						return Err( 
							Erreur::creer_chaine( 
								format!( 
									"La valeur '{}' n'a pas de sens en ligne de commande à cette position", 
									argument 
								) 
							) 
						); 
					} 
				} 
			} 
			CONFIGURATION = Some( 
				Configuration { 
					0: conf 
				}
			); 
			CONF_ENV_OBLIGATOIRES = Some( env_obligatoires ); 
			CONF_VARS_OBLIGATOIRES = Some( args_optionnels ); 
		} 
		Ok( () ) 
	}  
} 

pub fn conf_definir( cle: String, valeur: Types ) -> Result<(), Erreur>  { 
	unsafe { 
		if let Some( conf ) = &mut CONFIGURATION { 
			conf.0.insert( 
				cle, 
				valeur 
			); 
			Ok( () ) 
		} else { 
			Err( 
				Erreur::creer( 
					"La configuration n'a pas été faite : possible de définir une valeur" 
				)
			) 
		} 
	} 
} 

pub fn conf_obtenir( cle: &str ) -> Result<Option<Types>, Erreur>  { 
	unsafe { 
		if let Some( conf ) = &CONFIGURATION { 
			return Ok( 
				match conf.0.get( cle ) { 
					Some( v ) => Some( v.clone() ), 
					None => None 
				} 
			); 
		} else { 
			return Err( 
				Erreur::creer( 
					"La configuration n'a pas été faite : possible d'obtenir une valeur" 
				)
			); 
		} 
	} 
} 






