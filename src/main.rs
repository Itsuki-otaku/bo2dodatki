use rand::seq::SliceRandom;
use rand::Rng;
fn main() {
	println!("Losowy dodatek:");
	print_addon();
}
fn print_addon() {
	let mut rng = rand::thread_rng();
	let addons = vec![
		"Same pistolety",
		"Same headshoty",
		"Zakaz granatow",
		"Zakaz atutow",
		"Same snajperki",
		"Zakaz killstreakow",
		"Ja jestem sam ty z botami",
		"Zakaz machine gunow",
		"Same noze",
		"Robimy sobie klasy",
		"x_items",
		"Zakaz dodatkow na broniach",
		"Zakaz powiekszonego magazynka",
		"Zakaz niestandardowych celownikow",
		"Boty doswiadczone",
		"Zakaz granatow IEM",
		"Same shotguny",
		"Same karabiny szturmowe",
		"Same granaty",
		"Same tarcze szturmowe",
		"Zakaz broni dodatkowej",
		"Zakaz dzikich kart",
		"Maksymalnie jedna klasa",
		"Zakaz semtexow",
		"Zakaz ladunkow porazajacych",
		"Tylko jeden atut",
		"Tylko jedna dzika karta",
		"no_perk",
		"no_card",
		"30% zycia",
		"Hardcore",
		"health_regen",
		"Stala minimapa",
		"1 vs 1",
		"3 ostatnie killstreaki",
		"Zakaz sentry gunow",
		"Zakaz jednostki K9",	
		"Gra na samych oknach",
		"sens",
		"Brak minimapy",
		"Musi byc karabin szturmowy",
		"Zakaz niszczenia sentry gunow",
		"Defaultowe klasy",
		"Nie mozna wchodzic na srodek",
		"Nie mozna wchodzic do domkow",
		"Odwrocony widok",
		"200% zycia, szybka regeneracja",
		"30% zycia, brak regeneracji",
		"Zakaz aim assist",
	];
	let random_addon = addons.choose(&mut rng).unwrap();
	match random_addon {
		&"x_items" => println!("Max {} itemow w klasie", rng.gen_range(1,17)),
		&"no_perk" => {
			println!("Zakaz atutu '{}'", vec![
				"Waga Lekka",
				"Wojak", 
				"Padlinozerca",
				"Szybkie dlonie", 
				"Zrecznosc", 
				"Ekstremalna kondycja"
			].choose(&mut rng).unwrap());
		},
		&"no_card" => {
			println!("Zakaz dzikiej karty '{}'", vec![
				"Atut 1 Chciwosc",
				"Atut 2 Chciwosc",
				"Atut 3 Chciwosc",
				"Przesada",
				"Dodatkowy strzelec",
				"Glowny strzelec",
				"Taktyk",
				"Niebezpieczne blisko",
			].choose(&mut rng).unwrap());
		},
		&"health_regen" => {
			println!("Regeneracja zywotnosci: {}", vec![
				"Brak",
				"Wolno",
				"Szybko"
			].choose(&mut rng).unwrap());
		},
		&"sens" => {
			let sensitivities: Vec<u8> = (1..15).collect();
			println!("Sensitivity (czulosc) {}", sensitivities.choose(&mut rng).unwrap());
		},
		_ => println!("{}", random_addon),
	};
}
