
use crate::Jeton;
use crate::Clause;
use crate::Feuille;

#[derive(Debug)] 
pub struct Condition { 
	pub etat: Option<bool>, 
	pub clauses: Vec<Jeton<Clause>>, 
	pub declenchable: bool, 
	pub declenchee: bool 
} 

impl Condition { 
	pub fn creer( clauses: Vec<Jeton<Clause>> ) -> Self { 
		Condition { 
			etat: None, 
			clauses: clauses, 
			declenchable: false, 
			declenchee: false 
		} 
	} 
	pub fn preparer( &mut self ) -> Vec<String> { 
		self.clauses.iter().filter_map( 
			|item| match item { 
				Jeton::Appelable( c ) => Some( c.appelable.clone() ), 
				_ => None 
			}
		).collect::<Vec<String>>() 
	} 
	pub fn resoudre( &mut self ) { 
		let mut condition = vec!( Feuille::<Clause>::creer() ); 
		for element in &self.clauses { 
			match element { 
				Jeton::Appelable( c ) => { 
					&condition[..].last_mut().unwrap().pile.push( c.etat ); 
				} 
				Jeton::LiaisonOu | Jeton::LiaisonEt => { 
					&condition[..].last_mut().unwrap().action.push( element.clone() ); 
				}, 
				Jeton::GroupeOuvrant => { 
					condition.push( Feuille::<Clause>::creer() ); 
				}, 
				Jeton::GroupeFermant => { 
					let e = condition.pop().unwrap().resoudre(); 
					&condition[..].last_mut().unwrap().pile.push( e ); 
				} 
			} 
		} 
		let feuille = condition.pop().unwrap(); 
		self.etat = Some( feuille.resoudre() ); 
	} 
} 
