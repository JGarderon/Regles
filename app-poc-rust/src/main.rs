#![allow(warnings, unused)] 

use std::env; 

mod resolution_traitement; 
use crate::resolution_traitement::resolution_traitement; 

static ENV_RESOLUTION: &'static str = "RESOLUTION_TYPE"; 

fn main() { 

	match env::var( ENV_RESOLUTION ) {
		Ok( resolution ) => match &resolution[..] { 
			"traitement" => resolution_traitement(), 
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


/*	Version de réserve 	*/ 

// mod clauses; 
// mod jetons; 
// mod feuilles; 
// mod regles; 
// mod conditions; 

// use crate::clauses::Clause; 
// use crate::jetons::Jeton; 
// use crate::feuilles::Feuille; 
// use crate::regles::Regle; 
// use crate::conditions::Condition; 

// fn main() {

// 	let liste = vec!( 
// 		Jeton::GroupeOuvrant, 
// 			Jeton::Appelable( Clause::creer( "a".to_string() ) ), 
// 			Jeton::LiaisonOu, 
// 			Jeton::Appelable( Clause::creer( "b".to_string() ) ), 
// 			Jeton::LiaisonEt, 
// 			Jeton::Appelable( Clause::creer( "c".to_string() ) ), 
// 		Jeton::GroupeFermant, 
// 		Jeton::LiaisonEt, 
// 		Jeton::GroupeOuvrant, 
// 			Jeton::Appelable( Clause::creer( "d".to_string() ) ), 
// 			Jeton::LiaisonEt, 
// 			Jeton::GroupeOuvrant, 
// 				Jeton::Appelable( Clause::creer( "e".to_string() ) ), 
// 				Jeton::LiaisonOu, 
// 				Jeton::Appelable( Clause::creer( "f".to_string() ) ), 
// 			Jeton::GroupeFermant, 
// 		Jeton::GroupeFermant, 
// 		Jeton::LiaisonEt, 
// 		Jeton::Appelable( Clause::creer( "g".to_string() ) ) 
// 	); 

// 	let mut condition = Condition::creer( liste ); 
// 	condition.resoudre(); 

// 	println!( 
// 		"résultat final = {:?}",  
// 		condition.etat 
// 	); 

// } 




