
use crate::Jeton;
use crate::Condition;

#[derive(Debug)] 
pub struct Regle { 
	pub etat: Option<bool>, 
	pub conditions: Vec<Jeton<Condition>>, 
	pub declenchable: bool, 
	pub declenchee: bool 
}

impl Regle { 
	pub fn creer( conditions: Vec<Jeton<Condition>> ) -> Self { 
		Regle { 
			etat: None, 
			conditions: conditions, 
			declenchable: false, 
			declenchee: false 
		} 
	} 
	pub fn preparer( &mut self ) -> Vec<String> { 
		// self.conditions.iter().filter_map( 
		// 	|item| match item { 
		// 		Jeton::Appelable( c ) => Some( c.appelable.clone() ), 
		// 		_ => None 
		// 	}
		// ).collect::<Vec<String>>() 
		vec!() 
	} 
	pub fn resoudre( &mut self ) { 
		// let mut contexte = vec!( Feuille::<Condition>::creer() ); 
		// for element in &self.conditions { 
		// 	match element { 
		// 		Jeton::Appelable( c ) => { 
		// 			&contexte[..].last_mut().unwrap().pile.push( c.etat ); 
		// 		} 
		// 		Jeton::LiaisonOu | Jeton::LiaisonEt => { 
		// 			&contexte[..].last_mut().unwrap().action.push( element.clone() ); 
		// 		}, 
		// 		Jeton::GroupeOuvrant => { 
		// 			contexte.push( Feuille::creer() ); 
		// 		}, 
		// 		Jeton::GroupeFermant => { 
		// 			let e = contexte.pop().unwrap().resoudre(); 
		// 			&contexte[..].last_mut().unwrap().pile.push( e ); 
		// 		} 
		// 	} 
		// } 
		// let feuille = contexte.pop().unwrap(); 
		// self.etat = Some( feuille.resoudre() ); 
	} 
} 
