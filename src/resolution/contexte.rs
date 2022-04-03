
use std::mem; 
use core::slice::Iter; 

use crate::grammaire::constructeur::EnvironnementRegle; 
use crate::grammaire::constructeur::Environnement; 
use crate::communs::Types; 

use crate::communs::Erreur; 

#[derive(Debug)] 
pub struct ContexteRegle<'env> { 
	pub parent: &'env EnvironnementRegle, 
	pub clauses: Vec<usize> 
}

#[derive(Debug)] 
pub struct Contexte<'env> { 
	pub environnement: &'env Environnement, 
	pub clauses: Vec<Types>, 
	pub regles: Vec<ContexteRegle<'env>>, 
	pub position: usize 
} 

impl<'env> Contexte<'env> { 
	pub fn repositionner_index( &mut self, index: Option<usize> ) { 
		match index { 
			Some( p ) => self.position = p, 
			None => self.position = self.regles.len() 
		}; 
	} 
	pub fn repositionner( &mut self, nom: &String ) -> Result<(),Erreur> { 
		match self.regles.iter().position(
			|env_regle| &env_regle.parent.nom == nom 
		) { 
			Some( p ) => self.position = p, 
			None => return Err( Erreur::creer( "Appel d'une règle inconnue lors d'un repositionnement" ) ) 
		} 
		Ok( () )
	} 
	pub fn raz( &mut self, position: Option<usize> ) { 
		for clause in self.clauses.iter_mut() { 
			match clause { 
				Types::Appelable( _, etat, _ ) if *etat != None => *etat = None, 
				_ => () 
			} 
		} 
		if let Some( p ) = position { 
			self.position = p; 
		} 
	} 
} 

pub fn construire<'env>( environnement: &'env Environnement ) -> Result<Contexte,Erreur> { 
	let mut clauses = environnement.conditions.values().map( 
		|condition| { 
			condition.iter().filter_map( 
				|clause| match clause { 
					Types::Appelable( _, _, _ ) 
						| Types::Et 
						| Types::Ou => Some( clause.clone() ), 
					_ => None 
				} 
			).collect::<Vec<Types>>()
		} 
	).collect::<Vec<Vec<Types>>>().into_iter().flat_map( 
		|item| item 
	).collect::<Vec<Types>>(); // flatten ?
	clauses.sort_unstable_by( 
		// échouera si NaN/inf pour les f64 
		// https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by 
		|a, b| a.partial_cmp(b).unwrap() 
	); 
	clauses.dedup(); 
	clauses.push( Types::Fermeture ); 
	clauses.push( Types::Ouverture ); 
	clauses.reverse(); 
	let mut contexte = Contexte { 
		environnement: environnement, 
		clauses: clauses, 
		regles: vec!(), 
		position: 0 
	}; 
	let mut contexte_regles: Vec<(f64,ContexteRegle)> = vec!(); 
	for (_, regle_valeur) in environnement.regles.iter() { 
		let mut contexteregle_clauses: Vec<usize> = vec!(); 
		let mut positions: Vec<Iter<Types>> = vec!(); 
		let mut actuel =  regle_valeur.clauses.iter(); 
		contexteregle_clauses.push( 0 ); 
		loop { 
			while let Some( actuel_item ) = actuel.next() { 
				match actuel_item { 
					Types::Et | Types::Ou => match contexte.clauses.iter().position( 
						|item_clause| { 
							item_clause == actuel_item 
						} 
					) { 
						Some( index ) => contexteregle_clauses.push( index ), 
						_ => return Err( 
							Erreur::creer( "Liaison invalide au sein des clauses via le contexte (logique)" ) 
						) 
					} 
					Types::Appelable( _, _, _ ) => match contexte.clauses.iter().position( 
						|item_clause| { 
							item_clause == actuel_item 
						} 
					) { 
						Some( index ) => contexteregle_clauses.push( index ), 
						_ => return Err( 
							Erreur::creer( "Liaison invalide au sein des clauses via le contexte (appelable)" ) 
						) 
					} 
					Types::Conditionnel( nom ) => match environnement.conditions.get( nom ) { 
						Some( conditionnel_clauses ) => { 
							contexteregle_clauses.push( 0 ); 
							let mut ancien = actuel.clone(); 
							positions.push( ancien ); 
							actuel = conditionnel_clauses.iter(); 
							if positions.len() > 15 { 
								return Err( 
									Erreur::creer( 
										"Profondeur de récursion maximale autorisée des clauses conditionnelles atteinte" 
									) 
								) 
							} 
						} 
						None => return Err( 
							Erreur::creer( "Demande de condition inconnue" ) 
						) 
					} 
					_ => () 
				} 
			} 
			contexteregle_clauses.push( 1 ); 
			match positions.pop() { 
				Some( position ) => actuel = position, 
				None => break 
			} 
		} 
		// println!("{:#?}", contexteregle_clauses.iter().map( 
		// 	|index| match contexte.clauses.iter().nth( *index ) { 
		// 		Some( t ) => t.clone(), 
		// 		None => panic!("fdsfsd") 
		// 	} 
		// ).collect::<Vec<Types>>()); 
		contexte_regles.push( 
			( 
				regle_valeur.poids, 
				ContexteRegle { 
					parent: &regle_valeur, 
					clauses: contexteregle_clauses 
				} 
			) 
		); 
	} 
	contexte_regles.sort_unstable_by( 
		|a, b| a.0.partial_cmp( &b.0 ).unwrap() 
	); 
	contexte.regles = contexte_regles.into_iter().map( 
		|item| item.1 
	).collect::<Vec<ContexteRegle>>(); 
	Ok( 
		contexte 
	) 
} 

