
use std::io::Error; 

pub mod source; 
use crate::grammaire::source::Source; 

#[derive(Debug)]
pub struct Corpus {
	regles: Vec<()> 
}

fn terminal_espace( mut index: usize, source: &Source ) -> usize {
	let max = source.contenu.len(); 
	while index < max { 
		match source.contenu[index] { 
			' ' | '\t' | '\r' | '\n' => (), 
			_ => return index 
		} 
		index += 1; 
	} 
	index 
} 

fn nonterminal_variable( mut index: usize, source: &Source ) -> usize { 
	index = terminal_espace( index, source ); 
	// le reste 
	index 
} 

pub fn charger() -> Result<Corpus, Error> { 

	let source = Source::creer( "regles.txt".to_string() ).unwrap(); 

	nonterminal_expression_variable( 0, &source ); 

	Ok( Corpus {
		regles: vec!() 
	} ) 

}


