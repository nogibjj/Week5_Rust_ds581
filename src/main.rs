use week5_rust_ds581::generate_song_name;

fn main() {
    let words = &[
        "Moonlight",
	"Starry",
	"Oceanic",
	"Rainy",
	"Misty",
	"Sun-kissed",
	"Heartfelt",
	"Soulful",
	"Electric",
	"Firelight",
	"Dreamy",
	"Ethereal",
	"Midnight",
	"Sunset",
	"Sunrise",
	"Enchanted",
	"Wistful",
	"Serene",
	"Cosmic",
	"Melodic",
    ];
    let name = generate_song_name(words);
    println!("How about the song name: {}", name);
}




