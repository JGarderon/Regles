#!/usr/bin/env python3 

liste = [ "(", True, "|", False, "&", False, ")", "&", "(", True, "&", "(", True, "|", False, ")", ")", "&", True ] 

contextes = [{"pile":[],"action":[]}] 
for element in liste: 
	print(contextes)
	if type( element ) == bool: 
		contextes[-1]["pile"].append( element ) 
	elif element in ( "&", "|" ): 
		contextes[-1]["action"].append( element )
	elif element == "(": 
		contextes.append( {"pile":[],"action":[]} )
	elif element == ")": 
		contexte = contextes.pop() 
		# print("à résoudre: ", contexte ) 
		etat = contexte["pile"].pop( 0 ) 
		for (supp, liaison) in zip( contexte["pile"], contexte["action"] ): 
			# print("intermédiaire 1", etat, supp, liaison) 
			if liaison == "|": 
				etat |= supp 
			elif liaison == "&": 
				etat &= supp 
		contextes[-1]["pile"].append( etat ) 
	else: 
		raise Exception( f"élément '{element}' inconnu" ) 

contexte = contextes.pop() 
etat = contexte["pile"].pop( 0 ) 
for (supp, liaison) in zip( contexte["pile"], contexte["action"] ): 
	# print("intermédiaire 1", etat, supp, liaison) 
	if liaison == "|": 
		etat |= supp 
	elif liaison == "&": 
		etat &= supp 

print(etat) 






