
use crate::Contexte; 

use crate::contexte_resolution;
use crate::grammaire::constructeur::Environnement; 

use crate::communs::Types; 
use crate::communs::Dialogue; 

use crate::communs::ActionResolution;

fn resoudre( contexte: &mut Contexte, dialogue: &mut Dialogue ) -> ActionResolution { 
	if contexte.position >= contexte.regles.len() { 
		return ActionResolution::Arreter; 
	} 
	let regle = &contexte.regles[contexte.position]; 
	for index in regle.clauses.iter() { 
		match &mut contexte.clauses[*index] {
			Types::Appelable( fct, ref mut etat, args ) if *etat == None => match dialogue.soumettre( &fct, &args ) { 
				Ok( e ) => *etat = Some( e ), 
				Err( erreur ) => return ActionResolution::Erreur( erreur ) 
			} 
			_ => () 
		} 
	} 
	contexte.position += 1; 
	return ActionResolution::Continuer; 
} 

pub fn executer( environnement: &Environnement ) -> Result<(), &'static str> { 
	let mut contexte = contexte_resolution( &environnement)?; 
	let mut dialogue: Dialogue = Dialogue::creer();  
	loop {
		match dialogue.parler( "initier".as_bytes() )?.trim_end() { 
			"o" => (), 
			"a" => break, 
			"n" => return Err( "Le processus distant n'est pas prêt à exécuter les consignes du moteur de règles" ), 
			_ => return Err( "Le processus distant a répondu hors des valeurs autorisées au moment de l'initialisation" ) 
		} 
		contexte.raz(); 
		loop { 
			match resoudre( &mut contexte, &mut dialogue ) { 
				ActionResolution::Continuer => (), 
				ActionResolution::Arreter => break, 
				ActionResolution::Erreur( erreur ) => return Err( erreur ) 
			} 
		} 
	}
	Ok( () ) 
} 



