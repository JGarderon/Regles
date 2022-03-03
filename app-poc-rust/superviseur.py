#!/usr/bin/env python3 

import subprocess 
import random 
import asyncio 

### ----------------------------------------------

class Moteur: 

	processus = None 
	
	def __init__( self, executeur ): 
		self.processus = subprocess.Popen( 
			"target/release/app-poc-rust", 
			stdin=subprocess.PIPE, 
			stdout=subprocess.PIPE, 
			bufsize=1,
		    universal_newlines=True
		) 
		self.executeur = executeur

	async def lancer( self ): 
		for ligne in self.processus.stdout: 
			ligne = ligne.strip() 
			if await self.poursuire( ligne ) is False: 
				break 

	async def poursuire( self, ligne ): 
		if ligne == "initier": 
			suite = await self.executeur.initier() 
			self.processus.stdin.write( 
				"o\n" if suite else "a\n" 
			) 
			self.processus.stdin.flush() 
			return suite 
		else: 
			self.processus.stdin.write( 
				await self.executeur.faire( ligne ) 
			) 
		return True 


### ----------------------------------------------

class ExecuteurSimulation: 

	nbre = 0 
	maxi = None 
	etat = True 

	def __init__( self ): 
		self.maxi = random.randint( 1, 5 ) 

	async def initier( self ): 
		print( "exécuteur - initier", id(self), self.nbre, self.maxi ) 
		etat = False if self.maxi <= self.nbre else True 
		self.nbre += 1 
		await asyncio.sleep( 1 ) 
		return etat 

	async def faire( self, ligne ): 
		print( "exécuteur - faire", id(self), ligne ) 
		await asyncio.sleep( random.randint( 1, 3 ) ) 
		return "v\n" if random.randint( 1, 2 )%2 == 0 else "f\n" 

### ----------------------------------------------

async def main( *moteurs ): 
	await asyncio.gather( *moteurs )

if __name__ == "__main__": 
	asyncio.run( 
		main( 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 

			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 

			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
			Moteur( ExecuteurSimulation() ).lancer(), 
		) 
	)  
	# iterables = list( map( lambda m: iter( m ), moteurs ) ) 
	# print( iterables[0] ) 
	# for iterable in iterables: 
	# 	iterable.next() 


