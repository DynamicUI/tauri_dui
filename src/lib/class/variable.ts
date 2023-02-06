import { writable } from 'svelte/store';

// specifique a la class Variable

export const variables = writable(new Map<string, Variable>());
export const variable_names = writable(new Set<string>(['']));

variable_names.subscribe((names) => {
	console.log('variable_names', names);
});

/// Class specifique qui contient tout ce qui concerne les Box Varibles
export class Variable {
	id: string;
	name: string = '';
	value: string = '';

	constructor(name: string, value: any = null) {
		Variable.variable_quantity += 1;
		this.id =
			'variable_declaration_' +
			((Date.now() / 10000) * Variable.variable_quantity - Variable.variable_quantity)
				.toFixed()
				.toString();
		this.name = name;
		this.value = value;
	}

	static variable_quantity = 0;
}

/* reflection sur comment s'occuper de tous ces blocks:
- avoir une AST direct ? 
- avoir un tableau d'un class block de base ?

- avoir une class pour chaque type de boite 
	et une class box qui compose les autres (ou heritage)
	+ avoir un lien vers le parent ?


- Ce qu'il faut se dire c'est qu'a un moment ou a un autre, on va devoir creer
	un interpreteur/compilateur et qu'un des premieres etapes,
	la tokenisation (et le parsing) va etre grandement simplifiee
	si on a une structure de donnee qui est deja "simple" et qui
	va dans le sens ou qui est directement la structure de donnee qu'on
	veut pour le compilateur
 
*/
