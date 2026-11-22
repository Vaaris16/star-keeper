pub enum GameFonts {
    MontserratMedium,
}

impl GameFonts {
    pub fn path(&self) -> &'static str {
        match self {
            GameFonts::MontserratMedium => "fonts/montserrat/static/Montserrat-Medium.ttf",
        }
    }
}
