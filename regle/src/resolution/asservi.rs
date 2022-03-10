
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
	// eprintln!( "position = {:#?} ; état = {:#?}", contexte.position, etats ); 
	Ok( etats.pop().unwrap() ) 
} 

fn appliquer( contexte: &mut Contexte, dialogue: &mut Dialogue, etat: bool ) -> Result<(), &'static str> { 
	let mut avancement = true; 
	let fin = if etat == true { 
		// eprintln!( "position : {:?}", contexte.position ); 
		let mut i = 0; 
		for etape in contexte.regles[contexte.position].parent.alors.iter() { 
			match etape { 
				Types::Appelable( fct, _, args ) => { 
					dialogue.soumettre( &fct, &args )?; 
					i += 1; 
				}, 
				Types::Renvoi( nom_regle ) if &nom_regle[..] == "" => contexte.repositionner_index( None ), 
				Types::Renvoi( nom_regle ) => { 
					// eprintln!( "alors - reposition : {:?}", nom_regle ); 
					contexte.repositionner( &nom_regle )?; 
					avancement = false; 
				}, 
				_ => return Err( "Type invalide lors de l'application de la règle (partie 'Alors')" ) 
			} 
		} 
		if i > 0 { 
			true 
		} else { 
			false 
		} 
	} else { 
		let mut i = 0; 
		for etape in contexte.regles[contexte.position].parent.sinon.iter() { 
			match etape { 
				Types::Appelable( fct, _, args ) => { 
					dialogue.soumettre( &fct, &args )?; 
					i += 1; 
				}, 
				Types::Renvoi( nom_regle ) if &nom_regle[..] == "" => contexte.repositionner_index( None ), 
				Types::Renvoi( nom_regle ) => { 
					// eprintln!( "sinon - reposition : {:?}", nom_regle ); 
					contexte.repositionner( &nom_regle )?; 
					avancement = false; 
				}, 
				_ => return Err( "Type invalide lors de l'application de la règle (partie 'Sinon')" ) 
			} 
		} 
		if i > 0 { 
			true 
		} else { 
			false 
		} 
	}; 
	let fin = if fin { 
		for etape in contexte.regles[contexte.position].parent.finalement.iter() { 
			match etape { 
				Types::Appelable( fct, _, args ) => { 
					dialogue.soumettre( &fct, &args )?; 
				}, 
				Types::Renvoi( nom_regle ) if &nom_regle[..] == "" => contexte.repositionner_index( None ), 
				Types::Renvoi( nom_regle ) => { 
					// eprintln!( "finalement - reposition : {:?}", nom_regle ); 
					contexte.repositionner( &nom_regle )?; 
					avancement = false; 
				}, 
				_ => return Err( "Type invalide lors de l'application de la règle (partie 'Finalement')" ) 
			} 
		} 
		true 
	} else { 
		false 
	}; 
	if fin == true { 
		contexte.raz( None ); 
	} 
	if avancement == true { 
		contexte.position += 1; 
	} 
	Ok( () )
}

pub fn executer( environnement: &Environnement ) -> Result<(), &'static str> { 
	let mut contexte = contexte_resolution( &environnement)?; 
	let mut dialogue: Dialogue = Dialogue::creer(); 
	// println!("contexte = {:#?}", contexte);
	loop {
		if !dialogue.initier( environnement )? { 
			break; 
		} 
		contexte.raz( Some( 0 ) ); 
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



