
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
	return ActionResolution::Continuer; 
} 

fn determiner( contexte: &Contexte ) -> Result<bool, &'static str> { 
	// à consolider : rendre plus efficace et plus sûr 
	let mut actions: Vec<Vec<bool>> = vec!(); 
	let mut etats: Vec<bool> = vec!(); 
	for index in contexte.regles[contexte.position].clauses.iter() { 
		match contexte.clauses.iter().nth( *index ).unwrap() { 
			Types::Ouverture => actions.push( vec!() ), 
			Types::Et => actions.iter_mut().last().unwrap().push( true ), 
			Types::Ou => actions.iter_mut().last().unwrap().push( false ), 
			Types::Appelable( _, etat, _ ) => etats.push( etat.unwrap() ), 
			Types::Fermeture => { 
				let mut etat = etats.pop().unwrap(); 
				for action in actions.pop().unwrap().iter() { 
					if *action == true { 
						etat = etat && etats.pop().unwrap(); 
					} else { 
						etat = etat || etats.pop().unwrap(); 
					} 
				} 
				etats.push( etat ); 
			} 
			_ => () 
		} 
	} 
	Ok( etats.pop().unwrap() ) 
} 

fn appliquer( contexte: &mut Contexte, dialogue: &mut Dialogue, etat: bool ) -> Result<(), &'static str> { 
	let fin = if etat { 
		for etape in contexte.regles[contexte.position].parent.alors.iter() { 
			match etape { 
				Types::Appelable( fct, _, args ) => { 
					dialogue.soumettre( &fct, &args )?; 
				}, 
				_ => return Err( "Type invalide lors de l'application de la règle (partie 'Alors')" ) 
			} 
		} 
		if contexte.regles[contexte.position].parent.alors.len() > 0 { 
			true 
		} else { 
			false 
		} 
	} else { 
		for etape in contexte.regles[contexte.position].parent.sinon.iter() { 
			match etape { 
				Types::Appelable( fct, _, args ) => { 
					dialogue.soumettre( &fct, &args )?; 
				}, 
				_ => return Err( "Type invalide lors de l'application de la règle (partie 'Sinon')" ) 
			} 
		} 
		if contexte.regles[contexte.position].parent.sinon.len() > 0 { 
			true 
		} else { 
			false 
		} 
	}; 
	if fin { 
		for etape in contexte.regles[contexte.position].parent.finalement.iter() { 
			match etape { 
				Types::Appelable( fct, _, args ) => { 
					dialogue.soumettre( &fct, &args )?; 
				}, 
				_ => return Err( "Type invalide lors de l'application de la règle (partie 'Finalement')" ) 
			} 
		} 
	} 
	contexte.position += 1; 
	Ok( () )
}

pub fn executer( environnement: &Environnement ) -> Result<(), &'static str> { 
	let mut contexte = contexte_resolution( &environnement)?; 
	let mut dialogue: Dialogue = Dialogue::creer(); 
	loop {
		match dialogue.parler( "initier" )?.trim_end() { 
			"o" => (), 
			"a" => break, 
			"n" => return Err( "Le processus distant n'est pas prêt à exécuter les consignes du moteur de règles" ), 
			_ => return Err( "Le processus distant a répondu hors des valeurs autorisées au moment de l'initialisation" ) 
		} 
		contexte.raz(); 
		loop { 
			match resoudre( &mut contexte, &mut dialogue ) { 
				ActionResolution::Continuer => { 
					let etat = determiner( &contexte )?; 
					appliquer( 
						&mut contexte, 
						&mut dialogue, 
						etat 
					)?; 
				}, 
				ActionResolution::Arreter => break, 
				ActionResolution::Erreur( erreur ) => return Err( erreur ) 
			} 
		} 
	}
	Ok( () ) 
} 



