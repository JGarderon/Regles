
use std::env; 
// use std::sync::{Arc, Mutex}; # prochainement, pour la partie 'fil' ? 

use std::collections::HashMap; 
use crate::communs::Erreur; 
use crate::Types;

static mut CONFIGURATION: Option<Configuration> = None; 


#[derive(Debug)] 
pub struct Configuration( HashMap<&'static str,Types> ); 

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
			let mut conf = HashMap::new(); 
			for item in vec!( 
				vec!( "RESOLUTION_TYPE", "resolution_type" ), 
				vec!( "REGLES_SOURCE", "regles_source" ) 
			).iter() { 
				match env::var( item[0] ) { 
				    Ok( valeur ) => { 
				    	conf.insert( 
					    	item[1], 
					    	Types::Texte( valeur.to_string() ) 
					    ); 
					} 
				    Err( _ ) => () 
				}; 
			} 
			// for argument in env::args() { 
			//     argument.starts_with("--"); 
			// } 
			CONFIGURATION = Some( 
				Configuration { 
					0: conf 
				}
			); 
			println!("{:?}", CONFIGURATION); 
		} 
		Ok( () ) 
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
					"La configuration n'a pas été faite : possible de relancer" 
				)
			); 
		} 
	} 
}






