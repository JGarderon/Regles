
#[derive(Debug,PartialEq,PartialOrd)] 
pub enum Types {
	Nombre(f64), 
	Texte(String), 
	Conditionnel(String), 
	Appelable(String, bool, Vec<Types>), 
	Et, 
	Ou, 
}



