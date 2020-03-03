pub mod entities {
  #[derive(Serialize, Deserialize, Debug, Clone)]
  pub struct Jar {
    pub id: String,
    pub name: String,
    pub location: String,
    pub bar: Bars,
  }

  impl Jar {
    pub fn change_location(self, _location: String) -> Self {
      Jar {
        location: _location,
        ..self
      }
    }

    pub fn change_bar(self, _bar: Bars) -> Self {
      Jar { bar: _bar, ..self }
    }
  }
  #[derive(Serialize, Deserialize, Debug, Clone)]
  pub enum Bars {
    QuintaCamacho,
    Usaquen,
    Candelaria,
  }
}
