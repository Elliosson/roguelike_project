use rltk::RGB;

pub struct GameLog {
    pub entries: Vec<String>,
}

pub struct WorldStatLog {
    pub entries: Vec<String>,
}

pub struct GeneralLog {
    pub entries: Vec<String>,
}

pub struct SpeciesInstantLog {
    pub entries: Vec<(Vec<String>, RGB, u8)>,
}
