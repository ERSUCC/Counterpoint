use std::fmt::{ self, Debug, Display, Formatter };
use std::ops::{ Add, Sub };

#[derive(Clone, Copy)]
pub enum Name
{
    C,
    D,
    E,
    F,
    G,
    A,
    B
}

impl Name
{
    pub fn from_string(s: &str) -> Name
    {
        match s
        {
            "c" => Name::C,
            "d" => Name::D,
            "e" => Name::E,
            "f" => Name::F,
            "g" => Name::G,
            "a" => Name::A,
            "b" => Name::B,
            _ => panic!("Invalid note name \"{}\".", s)
        }
    }

    pub fn from_number(n: f32) -> Name
    {
        if n == 0.0
        {
            return Name::C;
        }

        if n == 1.0
        {
            return Name::D;
        }

        if n == 2.0
        {
            return Name::E;
        }

        if n == 3.0
        {
            return Name::F;
        }

        if n == 4.0
        {
            return Name::G;
        }

        if n == 5.0
        {
            return Name::A;
        }

        if n == 6.0
        {
            return Name::B;
        }
        
        panic!("Invalid number \"{}\".", n);
    }

    pub fn to_number(&self) -> f32
    {
        match self
        {
            Name::C => 0.0,
            Name::D => 1.0,
            Name::E => 2.0,
            Name::F => 3.0,
            Name::G => 4.0,
            Name::A => 5.0,
            Name::B => 6.0
        }
    }

    pub fn offset(&self) -> f32
    {
        match self
        {
            Name::C => 0.0,
            Name::D => 2.0,
            Name::E => 4.0,
            Name::F => 5.0,
            Name::G => 7.0,
            Name::A => 9.0,
            Name::B => 11.0
        }
    }
}

#[derive(Clone, Copy)]
pub enum Accidental
{
    Natural,
    Sharp,
    Flat
}

impl Accidental
{
    pub fn from(s: &str) -> Accidental
    {
        match s
        {
            "s" => Accidental::Sharp,
            "f" => Accidental::Flat,
            "" => Accidental::Natural,
            _ => panic!("Invalid accidental name \"{}\".", s)
        }
    }

    pub fn offset(&self) -> f32
    {
        match self
        {
            Accidental::Sharp => 1.0,
            Accidental::Flat => -1.0,
            Accidental::Natural => 0.0
        }
    }
}

#[derive(Clone, Copy)]
pub struct Note
{
    pub name: Name,
    pub accidental: Accidental,
    pub octave: f32
}

impl Note
{
    pub fn from_string(s: &str) -> Note
    {
        Note
        {
            name: Name::from_string(&s[0..1]),
            accidental: Accidental::from(&s[1..s.len() - 1]),
            octave: s[s.len() - 1..].parse::<f32>().unwrap()
        }
    }

    pub fn from_number(n: f32, o: f32) -> Note
    {
        Note
        {
            name: Name::from_number(n),
            accidental: Accidental::Natural,
            octave: o
        }
    }

    pub fn from_number_full(n: f32) -> Note
    {
        Note
        {
            name: Name::from_number(n % 7.0),
            accidental: Accidental::Natural,
            octave: (n / 7.0).floor()
        }
    }

    pub fn to_number(&self) -> f32
    {
        self.name.to_number()
    }

    pub fn to_number_full(&self) -> f32
    {
        self.octave * 7.0 + self.name.to_number()
    }

    pub fn frequency(&self) -> f32
    {
        16.35 * 2_f32.powf((12.0 * self.octave + self.name.offset() + self.accidental.offset()) / 12.0)
    }
}

impl Debug for Note
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.to_number_full())
    }
}

impl Display for Note
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.to_number_full())
    }
}

impl Add for Note
{
    type Output = f32;

    fn add(self, other: Note) -> Self::Output
    {
        self.to_number_full() + other.to_number_full()
    }
}

impl Sub for Note
{
    type Output = f32;

    fn sub(self, other: Note) -> Self::Output
    {
        self.to_number_full() - other.to_number_full()
    }
}