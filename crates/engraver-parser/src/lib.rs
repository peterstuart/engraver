use color_eyre::eyre::Result;
use engraver::model::{
    self, duration, key_signature, Alteration, Barline, Clef, Duration, KeySignature, Step,
    TimeSignature,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, i8, multispace0, u8},
    combinator::opt,
    error::ParseError,
    multi::{fold_many_m_n, many0, many_m_n, separated_list1},
    sequence::{delimited, separated_pair},
    Finish, IResult, Parser,
};
use strum::EnumCount;

/// Parses a string into a [`engraver::model::Staff`].
///
/// # Format
///
/// ## Staff
///
/// ```text
/// [clef]? [key signature]? [time signature]? [measure]*
/// ```
///
/// ## Measure
///
/// ```text
/// [element]* [barline]
/// ```
///
/// # Examples
///
/// A staff with a treble clef, key signature with 2 sharps, 4/4 time signature,
/// and 2 measures, the final measure having a double barline:
///
/// ```
/// use engraver_parser::parse_staff;
///
/// parse_staff("treble ## 4/4 d4 e f# g | a b c# d ||").unwrap();
/// ```
///
/// A more concise example, defaulting to treble clef and octave 4:
///
/// ```
/// use engraver_parser::parse_staff;
///
/// parse_staff("c d e f ||").unwrap();
pub fn parse_staff(input: &'static str) -> Result<model::Staff> {
    let (_, staff) = staff(input).finish()?;
    let staff = staff.into_model(&mut Context::default());

    Ok(staff)
}

#[derive(Debug)]
struct Context {
    octave: i8,
    duration: Duration,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            octave: 4,
            duration: Duration {
                value: duration::Value::Quarter,
                dots: None,
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Pitch {
    step: Step,
    alteration: Alteration,
    octave: Option<i8>,
}

impl Pitch {
    fn into_model(self, context: &mut Context) -> model::Pitch {
        if let Some(octave) = self.octave {
            context.octave = octave;
        }

        model::Pitch {
            step: self.step,
            alteration: self.alteration,
            octave: context.octave,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Note {
    pitch: Pitch,
    duration: Option<Duration>,
}

impl Note {
    fn into_model(self, context: &mut Context) -> model::Note {
        if let Some(duration) = self.duration {
            context.duration = duration;
        }

        model::Note {
            pitch: self.pitch.into_model(context),
            duration: context.duration,
            id: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Chord {
    pitches: Vec<Pitch>,
    duration: Option<Duration>,
}

impl Chord {
    fn into_model(self, context: &mut Context) -> model::Chord {
        if let Some(duration) = self.duration {
            context.duration = duration;
        }

        model::Chord {
            pitches: self
                .pitches
                .into_iter()
                .map(|pitch| pitch.into_model(context))
                .collect(),
            duration: context.duration,
            id: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Rest {
    duration: Option<Duration>,
}

impl Rest {
    fn into_model(self, context: &mut Context) -> model::Rest {
        if let Some(duration) = self.duration {
            context.duration = duration;
        }

        model::Rest {
            duration: context.duration,
            id: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Element {
    Note(Note),
    Chord(Chord),
    Rest(Rest),
}

impl Element {
    fn into_model(self, context: &mut Context) -> model::measure::Element {
        match self {
            Self::Note(note) => model::measure::Element::Note(note.into_model(context)),
            Self::Chord(chord) => model::measure::Element::Chord(chord.into_model(context)),
            Self::Rest(note) => model::measure::Element::Rest(note.into_model(context)),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Measure {
    elements: Vec<Element>,
    barline: Barline,
}

impl Measure {
    fn into_model(self, context: &mut Context) -> model::Measure {
        model::Measure {
            elements: self
                .elements
                .into_iter()
                .map(|element| element.into_model(context))
                .collect(),
            barline: self.barline,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Staff {
    clef: Option<Clef>,
    key_signature: Option<KeySignature>,
    time_signature: Option<TimeSignature>,
    measures: Vec<Measure>,
}

impl Staff {
    fn into_model(self, context: &mut Context) -> model::Staff {
        model::Staff {
            clef: self.clef.unwrap_or_default(),
            time_signature: self.time_signature,
            key_signature: self.key_signature,
            measures: self
                .measures
                .into_iter()
                .map(|measure| measure.into_model(context))
                .collect(),
        }
    }
}

fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

fn step(input: &str) -> IResult<&str, Step> {
    alt((
        tag("c").map(|_| Step::C),
        tag("d").map(|_| Step::D),
        tag("e").map(|_| Step::E),
        tag("f").map(|_| Step::F),
        tag("g").map(|_| Step::G),
        tag("a").map(|_| Step::A),
        tag("b").map(|_| Step::B),
    ))(input)
}

fn alteration(input: &str) -> IResult<&str, Alteration> {
    alt((
        tag("bb").map(|_| Alteration::DoubleFlat),
        tag("b").map(|_| Alteration::Flat),
        tag("#").map(|_| Alteration::Sharp),
        tag("x").map(|_| Alteration::DoubleSharp),
    ))(input)
}

fn octave(input: &str) -> IResult<&str, i8> {
    i8(input)
}

fn pitch(input: &str) -> IResult<&str, Pitch> {
    let (input, step) = step(input)?;
    let (input, alteration) = opt(alteration)(input)?;
    let (input, octave) = opt(octave)(input)?;

    Ok((
        input,
        Pitch {
            step,
            alteration: alteration.unwrap_or(Alteration::Natural),
            octave,
        },
    ))
}

fn value(input: &str) -> IResult<&str, duration::Value> {
    alt((
        char('1').map(|_| duration::Value::Whole),
        char('2').map(|_| duration::Value::Half),
        char('4').map(|_| duration::Value::Quarter),
        char('8').map(|_| duration::Value::Eighth),
        tag("16").map(|_| duration::Value::Sixteenth),
        tag("32").map(|_| duration::Value::ThirtySecond),
        tag("64").map(|_| duration::Value::SixtyFourth),
        tag("128").map(|_| duration::Value::OneHundredTwentyEighth),
        tag("256").map(|_| duration::Value::TwoHundredFiftySixth),
    ))(input)
}

fn duration(input: &str) -> IResult<&str, Duration> {
    let (input, value) = value(input)?;
    let (input, num_dots) = fold_many_m_n(0, 2, char('.'), || 0, |count, _| count + 1)(input)?;
    let dots = match num_dots {
        0 => None,
        1 => Some(duration::Dots::Dot),
        2 => Some(duration::Dots::DoubleDot),
        _ => unreachable!(),
    };

    Ok((input, Duration { value, dots }))
}

fn bracketed_duration(input: &str) -> IResult<&str, Duration> {
    delimited(char('['), duration, char(']'))(input)
}

fn note(input: &str) -> IResult<&str, Note> {
    let (input, pitch) = pitch(input)?;
    let (input, duration) = opt(bracketed_duration)(input)?;

    Ok((input, Note { pitch, duration }))
}

fn chord(input: &str) -> IResult<&str, Chord> {
    let (input, pitches) =
        delimited(char('{'), separated_list1(char(' '), pitch), char('}'))(input)?;
    let (input, duration) = opt(bracketed_duration)(input)?;

    Ok((input, Chord { pitches, duration }))
}

fn rest(input: &str) -> IResult<&str, Rest> {
    let (input, _) = char('r')(input)?;
    let (input, duration) = opt(bracketed_duration)(input)?;

    Ok((input, Rest { duration }))
}

fn barline(input: &str) -> IResult<&str, Barline> {
    alt((
        tag("||").map(|_| Barline::Final),
        char('|').map(|_| Barline::Thin),
    ))(input)
}

fn element(input: &str) -> IResult<&str, Element> {
    alt((
        note.map(Element::Note),
        chord.map(Element::Chord),
        rest.map(Element::Rest),
    ))(input)
}

fn measure(input: &str) -> IResult<&str, Measure> {
    let (input, elements) = many0(ws(element))(input)?;
    let (input, barline) = ws(barline)(input)?;

    Ok((input, Measure { elements, barline }))
}

fn clef(input: &str) -> IResult<&str, Clef> {
    alt((
        tag("treble").map(|_| Clef::Treble),
        tag("alto").map(|_| Clef::Alto),
        tag("tenor").map(|_| Clef::Tenor),
        tag("bass").map(|_| Clef::Bass),
    ))(input)
}

fn time_signature(input: &str) -> IResult<&str, TimeSignature> {
    let (input, (numerator, denominator)) = separated_pair(u8, char('/'), u8)(input)?;

    Ok((
        input,
        TimeSignature {
            numerator,
            denominator,
        },
    ))
}

fn key_signature(input: &str) -> IResult<&str, KeySignature> {
    let key_signature_parser = |kind, c| {
        many_m_n(1, Step::COUNT, char(c))
            .map(move |characters| KeySignature::new(kind, characters.len() as u8))
    };

    alt((
        key_signature_parser(key_signature::Kind::Sharps, '#'),
        key_signature_parser(key_signature::Kind::Flats, 'b'),
    ))(input)
}

fn staff(input: &str) -> IResult<&str, Staff> {
    let (input, clef) = opt(ws(clef))(input)?;
    let (input, key_signature) = opt(ws(key_signature))(input)?;
    let (input, time_signature) = opt(ws(time_signature))(input)?;
    let (input, measures) = ws(many0(measure))(input)?;

    Ok((
        input,
        Staff {
            clef,
            key_signature,
            time_signature,
            measures,
        },
    ))
}
