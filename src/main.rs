mod wizlight;

use clap::Clap;

use palette::Hue;

/// Handles a wiz light bulb from the console
#[derive(clap::Clap)]
#[clap(version = "0.1", author = "Federico Palacios <fedenator7@gmail.com>")]
struct CliOptions
{
	/// Bulb host
	#[clap(short, long, default_value = "192.168.0.70:38899")]
	host: String,

	#[clap(subcommand)]
	subcommand: SubCommands,
}

#[derive(clap::Clap)]
enum SubCommands
{
	/// Turns on the bulb
	TurnOn,

	/// Turns off the bulb
	TurnOff,

	/// Activates party mode!!!
	Party,

	/// Changes the bulb color
	Color
	{
		#[clap(short)]
		r: f32,
		#[clap(short)]
		g: f32,
		#[clap(short)]
		b: f32,

		#[clap(short)]
		d: u32,
	},
}

fn party_mode(client: wizlight::Client) -> Result<(), wizlight::Error>
{
	client.set_turned_on(true)?;
	let mut current_color = palette::Hsv::new(0_f32, 1_f32, 1_f32);
	loop
	{
		let aux = palette::Srgb::from(current_color.clone());
		client.set_color(&aux, 100)?;
		std::thread::sleep(std::time::Duration::from_millis(250));
		current_color = current_color.shift_hue(70_f32);
	}
}

fn main() -> Result<(), wizlight::Error>
{
	let cli_options: CliOptions = CliOptions::parse();

	let client = wizlight::Client::new(&cli_options.host)?;

	match cli_options.subcommand
	{
	    SubCommands::TurnOn =>
		{
			client.set_turned_on(true)?;
		},
	    SubCommands::TurnOff =>
		{
			client.set_turned_on(false)?;
		},
	    SubCommands::Party =>
		{
			party_mode(client)?;
		},
	    SubCommands::Color { r, g, b, d } =>
		{
			client.set_color(&palette::Srgb::new(r, g, b), d)?;
		},
	};

	return Ok(());
}
