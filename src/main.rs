use rand::seq::SliceRandom;
use rand::Rng;
fn main() {
	println!("Losowy dodatek:");
	print_addon();
}
fn print_addon() {
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
		"Jedna klasa dla wszystkich",
		"Stala minimapa",
		"1 vs 1",
		"3 ostatnie killstreaki",
		"Zakaz sentry gunow",
		"Zakaz jednostki K9",	
		"Gra na samych oknach",
		"sens",
	];
	let random_addon = addons.choose(&mut rand::thread_rng()).unwrap();
	match random_addon {
		&"x_items" => println!("Max {} itemow w klasie", rand::thread_rng().gen_range(1,17)),
		&"no_perk" => {
			println!("Zakaz atutu '{}'", vec![
				"Waga Lekka",
				"Wojak", 
				"Padlinozerca",
				"Szybkie dlonie", 
				"Zrecznosc", 
				"Ekstremalna kondycja"
			].choose(&mut rand::thread_rng()).unwrap());
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
			].choose(&mut rand::thread_rng()).unwrap());
		},
		&"health_regen" => {
			println!("Regeneracja zywotnosci: {}", vec![
				"Brak",
				"Wolno",
				"Szybko"
			].choose(&mut rand::thread_rng()).unwrap());
		},
		&"sens" => {
			let sensitivities: Vec<u8> = (1..15).collect();
			println!("Sensitivity (czulosc) {}", sensitivities.choose(&mut rand::thread_rng()).unwrap());
		},
		_ => println!("{}", random_addon),
	};
}
