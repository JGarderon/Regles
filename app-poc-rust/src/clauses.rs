
#[derive(Clone,Debug)] 
pub struct Clause { 
	pub appelable: String, 
	pub etat: bool 
} 

impl Clause { 
	pub fn creer( appelable: String ) -> Self { 
		Clause { 
			appelable: appelable, 
			etat: false 
		} 
	} 
} 

