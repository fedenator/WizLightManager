#[derive(Debug)]
pub enum Error
{
	NetWorkError,
	ParseError,
	ServerError,
	UnexpectedError,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ColorDto
{
	r: u8,
	g: u8,
	b: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pilot
{
	#[serde(flatten)]
	#[serde(skip_serializing_if = "Option::is_none")]
	color: Option<ColorDto>,
	#[serde(skip_serializing_if = "Option::is_none")]
	dimming: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	state: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserConfigDto
{
	#[serde(rename="fadeIn")]
	fade_in: u32,

	#[serde(rename="fadeOut")]
	fade_out: u32,

	#[serde(rename="fadeNight")]
	fade_night: bool,

	#[serde(rename="dftDim")]
	dft_dim: u32,

	#[serde(rename="pwmRange")]
	pwm_range: [u32; 2],

	#[serde(rename="whiteRange")]
	white_range: [u32; 2],

	#[serde(rename="extRange")]
	ext_range: [u32; 2],

	po: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag="method", content="params")]
pub enum CommandDto
{
	#[serde(rename="setPilot")]
	SetPilot(Pilot),
	#[serde(rename="getPilot")]
	GetPilot(Pilot),
	#[serde(rename="getUserConfig")]
	GetUserConfig
	{
	}
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag="method", content="result")]
pub enum ResponsePayloadDto
{
	#[serde(rename="setPilot")]
	SetPilot
	{
		success: bool
	},
	#[serde(rename="getUserConfig")]
	GetUserConfig(UserConfigDto),
}
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ResponseDto
{
	#[serde(flatten)]
	response_payload: ResponsePayloadDto,

	env: String
}

pub struct Client
{
	socket: std::net::UdpSocket,
}

impl Client
{
	fn send_command(
		&self,
		command: &CommandDto
	)
	-> Result<ResponseDto, Error>
	{
		let msg = serde_json::to_string(command).unwrap();
		self.socket.send(msg.as_bytes()).expect("Error al enviar el mensaje a la lampara");
		let mut buffer = [0_u8; 4096];

		let bytes_count = match self.socket.recv(&mut buffer)
		{
			Ok(bytes_count) => bytes_count,
			Err(_error) => return Err(Error::NetWorkError),
		};

		let response = match std::str::from_utf8(&buffer[..bytes_count])
		{
			Ok(response) => response,
			Err(_error) => return Err(Error::ParseError)
		};

		let response_dto = match serde_json::from_str(response)
		{
			Ok(response_dto) => response_dto,
			Err(_error) => return Err(Error::ParseError),
		};

		return Ok(response_dto);
	}

	pub fn new(reciver: &str) -> Result<Client, Error>
	{
		let socket = std::net::UdpSocket::bind("0.0.0.0:0").expect("No se pudo bindear a un puerto local");
		if let Err(_error) = socket.connect(reciver)
		{
			return Err(Error::NetWorkError);
		}
		return Ok( Client{ socket: socket } );
	}

	pub fn set_turned_on(&self, turned_on: bool) -> Result<(), Error>
	{
		let response = self.send_command(
			&CommandDto::SetPilot
			{
			    color  : None,
				dimming: None,
			    state  : Some(turned_on),
			}
		)?;

		if let ResponsePayloadDto::SetPilot{success} = response.response_payload
		{
			if success { return Ok(()); }
			else       { return Err(Error::ServerError); }
		}
		else
		{
			return Err(Error::UnexpectedError);
		}
	}

	pub fn set_color(&self, color: &palette::Srgb<f32>, dimming: u32) -> Result<(), Error>
	{
		let response = self.send_command(
			&CommandDto::SetPilot
			{
			    color: Some(
					ColorDto
					{
						r: (color.red * 255_f32) as u8,
						g: (color.green * 255_f32) as u8,
						b: (color.blue * 255_f32) as u8
					},
				),
				dimming: Some(dimming),
			    state: None,
			}
		)?;


		if let ResponsePayloadDto::SetPilot{success} = response.response_payload
		{
			if success { return Ok(()); }
			else       { return Err(Error::ServerError); }
		}
		else
		{
			return Err(Error::UnexpectedError);
		}
	}

	pub fn get_config(&self) -> Result<UserConfigDto, Error>
	{
		let response = self.send_command(&CommandDto::GetUserConfig{})?;

		if let ResponsePayloadDto::GetUserConfig(user_config) = response.response_payload
		{
			return Ok(user_config);
		}
		else
		{
			return Err(Error::UnexpectedError);
		}
	}

	pub fn get_pilot(&self) -> Result<UserConfigDto, Error>
	{
		let response = self.send_command(&CommandDto::GetUserConfig{})?;

		if let ResponsePayloadDto::GetUserConfig(user_config) = response.response_payload
		{
			return Ok(user_config);
		}
		else
		{
			return Err(Error::UnexpectedError);
		}
	}
}
