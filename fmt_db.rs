// cargo-deps: toml="*", serde = "*"

use std::fmt;
use std::io;
use std::fs;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::path::Path;

fn main() -> io::Result<()> 
{
    //Takes path from the enviroment
    let path = std::env::var("RUST_SCRIPT_PATH").unwrap();
    let mut dir = Path::new(&path).parent().unwrap().to_path_buf();
    //Adds the folder structure to base path
    dir.push("pad_db");
    dir.push("pad");
    dir.push("mapping");
    //printing for clarity
    println!("The directory is: {:?}", dir);
    if dir.is_dir()
    {
        order_dir(dir.as_path())?;
        Ok(())
    }
    else
    {
        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}


/// Loop through each folder in the directory
fn order_dir(dir: &Path) -> io::Result<()>
{
    for file in Path::new(dir).read_dir()?
    {
        order_file(file?.path().as_path())?;
    }
    Ok(())
}

/// This reads a file into a string
/// Tries to parse it from toml into a Controller struct
/// Sorts the inner structs (button, axis, etc.) by code
/// Tries to parse to toml again
/// Write to file
fn order_file(file_path: &Path) -> io::Result<()>
{
    println!("{:?}", file_path);
    let content = fs::read_to_string(file_path)?;
    let value: Result<Controller, toml::de::Error> = toml::from_str(&content);
    if let Ok(mut controller) = value
    {
        controller.sort_by_code();
        let toml_result = toml::to_string(&controller);
        if let Ok(toml) = toml_result
        {
            fs::write(file_path, &toml)?;
        }
        else if let Err(error) = toml_result
        {
            println!("{}", error);
        }
    }
    else if let Err(error) = value
    {
        println!("{}", error);
    }
    Ok(())
}


/// Controller struct
/// its fields are options because not every controller has every input type.
struct Controller
{
    name: String,
    r#type: String,
    deadzone: Option<f64>,
    button: Option<Vec<Button>>,
    axis: Option<Vec<Axis>>,
    trigger: Option<Vec<Trigger>>,
    three_way: Option<Vec<ThreeWay>>,
    wheel: Option<Vec<Wheel>>,
    throttle: Option<Vec<Throttle>>,
}

impl Serialize for Controller
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Controller", 8)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("deadzone", &self.deadzone)?;
        state.serialize_field("button", &self.button)?;
        state.serialize_field("axis", &self.axis)?;
        state.serialize_field("trigger", &self.trigger)?;
        state.serialize_field("three_way", &self.three_way)?;
        state.serialize_field("wheel", &self.wheel)?;
        state.serialize_field("throttle", &self.throttle)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Controller {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Name, Type, Deadzone, Button, Axis, Trigger, ThreeWay, Wheel, Throttle};

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`controller stuff`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "name" => Ok(Field::Name),
                            "type" => Ok(Field::Type),
                            "deadzone" => Ok(Field::Deadzone),
                            "button" => Ok(Field::Button),
                            "axis" => Ok(Field::Axis),
                            "trigger" => Ok(Field::Trigger),
                            "three_way" => Ok(Field::ThreeWay),
                            "wheel" => Ok(Field::Wheel),
                            "throttle" => Ok(Field::Throttle),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ControllerVisitor;

        impl<'de> Visitor<'de> for ControllerVisitor {
            type Value = Controller;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Controller")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Controller, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let name = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let r#type = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let deadzone = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let button = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let axis = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let trigger = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                let three_way = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(6, &self))?;
                let wheel = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(7, &self))?;
                let throttle = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(8, &self))?;
                Ok(Controller {name, r#type, deadzone, button, axis, trigger, three_way, wheel, throttle})
            }

            fn visit_map<V>(self, mut map: V) -> Result<Controller, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut r#type = None;
                let mut deadzone = None;
                let mut button = None;
                let mut axis = None;
                let mut trigger = None;
                let mut three_way = None;
                let mut wheel = None;
                let mut throttle = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map.next_value()?);
                        }
                        Field::Deadzone =>
                        {
                            if deadzone.is_some() {
                                return Err(de::Error::duplicate_field("deadzone"));
                            }
                            deadzone = Some(map.next_value()?);
                        }
                        Field::Button => {
                            if button.is_some() {
                                return Err(de::Error::duplicate_field("button"));
                            }
                            button = Some(map.next_value()?);
                        }
                        Field::Axis => {
                            if axis.is_some() {
                                return Err(de::Error::duplicate_field("axis"));
                            }
                            axis = Some(map.next_value()?);
                        }
                        Field::Trigger => {
                            if trigger.is_some() {
                                return Err(de::Error::duplicate_field("trigger"));
                            }
                            trigger = Some(map.next_value()?);
                        }
                        Field::ThreeWay => {
                            if three_way.is_some() {
                                return Err(de::Error::duplicate_field("three_way"));
                            }
                            three_way = Some(map.next_value()?);
                        }
                        Field::Wheel => {
                            if wheel.is_some()
                            {
                                return Err(de::Error::duplicate_field("wheel"));
                            }
                            wheel = Some(map.next_value()?);
                        }
                        Field::Throttle => {
                            if throttle.is_some()
                            {
                                return Err(de::Error::duplicate_field("throttle"));
                            }
                            throttle = Some(map.next_value()?);
                        }
                    }
                }                
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let r#type = r#type.ok_or_else(|| de::Error::missing_field("r#type"))?;
                Ok( Controller { name, r#type, deadzone, button, axis, trigger, three_way, wheel, throttle})
            }
        }

        const FIELDS: &'static [&'static str] = &["name", "r#type", "deadzone", "button", "axis", "trigger", "three_way", "wheel", "throttle"];
        deserializer.deserialize_struct("Controller", FIELDS, ControllerVisitor)
    }
}

impl Controller
{
    fn sort_by_code(&mut self)
    {
        self.sort_by_code_button();
        self.sort_by_code_axis();
        self.sort_by_code_three_way();
        self.sort_by_code_trigger();
        self.sort_by_code_throttle();
        self.sort_by_code_wheel();
    }

    fn sort_by_code_button(&mut self)
    {
        if let Some(v) = &mut self.button
        {
            v.sort_by(|a, b| a.code.cmp(&b.code));
        }
    }
    fn sort_by_code_axis(&mut self)
    {
        if let Some(v) = &mut self.axis
        {
            v.sort_by(|a, b| a.code.cmp(&b.code));
        }
    }
    fn sort_by_code_trigger(&mut self)
    {
        if let Some(v) = &mut self.trigger
        {
            v.sort_by(|a, b| a.code.cmp(&b.code));
        }
    }
    fn sort_by_code_three_way(&mut self)
    {
        if let Some(v) = &mut self.three_way
        {
            v.sort_by(|a, b| a.code.cmp(&b.code));
        }
    }

    fn sort_by_code_wheel(&mut self)
    {
        if let Some(v) = &mut self.wheel
        {
            v.sort_by(|a, b| a.code.cmp(&b.code));
        }
    }

    fn sort_by_code_throttle(&mut self)
    {
        if let Some(v) = &mut self.throttle
        {
            v.sort_by(|a, b| a.code.cmp(&b.code));
        }
    }
}

struct Button
{
    code: i16,
    event: String,
}

impl serde::ser::Serialize for Button
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Button", 2)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("event", &self.event)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Button {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Code, Event };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "code" => Ok(Field::Code),
                            "event" => Ok(Field::Event),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ButtonVisitor;

        impl<'de> Visitor<'de> for ButtonVisitor {
            type Value = Button;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Button, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let code = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let event = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Button { code, event})
            }

            fn visit_map<V>(self, mut map: V) -> Result<Button, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut code = None;
                let mut event = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Code => {
                            if code.is_some() {
                                return Err(de::Error::duplicate_field("code"));
                            }
                            code = Some(map.next_value()?);
                        }
                        Field::Event => {
                            if event.is_some() {
                                return Err(de::Error::duplicate_field("event"));
                            }
                            event = Some(map.next_value()?);
                        }
                    }
                }
                let code = code.ok_or_else(|| de::Error::missing_field("code"))?;
                let event = event.ok_or_else(|| de::Error::missing_field("event"))?;
                Ok(Button { code, event })
            }
        }

        const FIELDS: &'static [&'static str] = &["code", "event"];
        deserializer.deserialize_struct("Button", FIELDS, ButtonVisitor)
    }
}

struct Axis
{
    code: u8,
    event: String,
    max: Option<f64>
}

impl Serialize for Axis
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Axis", 3)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("event", &self.event)?;
        state.serialize_field("max", &self.max)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Axis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Code, Event, Max};

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("Code, event or max")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "code" => Ok(Field::Code),
                            "event" => Ok(Field::Event),
                            "max" => Ok(Field::Max),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct AxisVisitor;

        impl<'de> Visitor<'de> for AxisVisitor {
            type Value = Axis;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Axis")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Axis, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let code = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let event = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let max = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(Axis { code, event, max})
            }

            fn visit_map<V>(self, mut map: V) -> Result<Axis, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut code = None;
                let mut event = None;
                let mut max = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Code => {
                            if code.is_some() {
                                return Err(de::Error::duplicate_field("code"));
                            }
                            code = Some(map.next_value()?);
                        }
                        Field::Event => {
                            if event.is_some() {
                                return Err(de::Error::duplicate_field("event"));
                            }
                            event = Some(map.next_value()?);
                        }
                        Field::Max => {
                            if max.is_some() {
                                return Err(de::Error::duplicate_field("max"));
                            }
                            max = Some(map.next_value()?);
                        }
                    }
                }
                let code = code.ok_or_else(|| de::Error::missing_field("code"))?;
                let event = event.ok_or_else(|| de::Error::missing_field("event"))?;
                Ok(Axis { code, event, max })
            }
        }

        const FIELDS: &'static [&'static str] = &["code", "event", "max"];
        deserializer.deserialize_struct("Axis", FIELDS, AxisVisitor)
    }
}
struct Trigger
{
    code: u8,
    event: String,
    max: Option<f64>,
    deadzone: Option<f64>,
    invert: Option<bool>,
}
impl serde::ser::Serialize for Trigger
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Trigger", 5)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("event", &self.event)?;
        state.serialize_field("max", &self.max)?;
        state.serialize_field("deadzone", &self.deadzone)?;
        state.serialize_field("invert", &self.invert)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Trigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Code, Event, Max, Deadzone, Invert};

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("Code, event, max, deadzone or trigger")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "code" => Ok(Field::Code),
                            "event" => Ok(Field::Event),
                            "max" => Ok(Field::Max),
                            "deadzone" => Ok(Field::Deadzone),
                            "invert" => Ok(Field::Invert),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct TriggerVisitor;

        impl<'de> Visitor<'de> for TriggerVisitor {
            type Value = Trigger;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Trigger")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Trigger, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let code = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let event = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let max = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let deadzone = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let invert = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                Ok(Trigger { code, event, max, deadzone, invert})
            }

            fn visit_map<V>(self, mut map: V) -> Result<Trigger, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut code = None;
                let mut event = None;
                let mut max = None;
                let mut deadzone = None;
                let mut invert = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Code => {
                            if code.is_some() {
                                return Err(de::Error::duplicate_field("code"));
                            }
                            code = Some(map.next_value()?);
                        }
                        Field::Event => {
                            if event.is_some() {
                                return Err(de::Error::duplicate_field("event"));
                            }
                            event = Some(map.next_value()?);
                        }
                        Field::Max => {
                            if max.is_some() {
                                return Err(de::Error::duplicate_field("max"));
                            }
                            max = Some(map.next_value()?);
                        }
                        Field::Deadzone => {
                            if deadzone.is_some() {
                                return Err(de::Error::duplicate_field("deadzone"));
                            }
                            deadzone = Some(map.next_value()?);
                        }
                        Field::Invert => {
                            if invert.is_some() {
                                return Err(de::Error::duplicate_field("invert"));
                            }
                            invert = Some(map.next_value()?);
                        }
                    }
                }
                let code = code.ok_or_else(|| de::Error::missing_field("code"))?;
                let event = event.ok_or_else(|| de::Error::missing_field("event"))?;
                Ok(Trigger { code, event, max, deadzone, invert })
            }
        }

        const FIELDS: &'static [&'static str] = &["code", "event", "max", "deadzone", "invert"];
        deserializer.deserialize_struct("Trigger", FIELDS, TriggerVisitor)
    }
}

struct ThreeWay
{
    code: i16,
    neg: String,
    pos: String,
}

impl serde::ser::Serialize for ThreeWay
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ThreeWay", 3)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("neg", &self.neg)?;
        state.serialize_field("pos", &self.pos)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for ThreeWay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Code, Neg, Pos};

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`code, neg or pos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "code" => Ok(Field::Code),
                            "neg" => Ok(Field::Neg),
                            "pos" => Ok(Field::Pos),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ThreeWayVisitor;

        impl<'de> Visitor<'de> for ThreeWayVisitor {
            type Value = ThreeWay;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ThreeWay")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<ThreeWay, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let code = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let neg = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let pos = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                Ok(ThreeWay { code, neg, pos})
            }

            fn visit_map<V>(self, mut map: V) -> Result<ThreeWay, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut code = None;
                let mut neg = None;
                let mut pos = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Code => {
                            if code.is_some() {
                                return Err(de::Error::duplicate_field("code"));
                            }
                            code = Some(map.next_value()?);
                        }
                        Field::Neg => {
                            if neg.is_some() {
                                return Err(de::Error::duplicate_field("neg"));
                            }
                            neg = Some(map.next_value()?);
                        }
                        Field::Pos => {
                            if pos.is_some(){
                                return Err(de::Error::duplicate_field("pos"));
                            }
                            pos = Some(map.next_value()?);
                        }
                    }
                }
                let code = code.ok_or_else(|| de::Error::missing_field("code"))?;
                let neg = neg.ok_or_else(|| de::Error::missing_field("neg"))?;
                let pos = pos.ok_or_else(|| de::Error::missing_field("pos"))?;
                Ok(ThreeWay { code, neg, pos })
            }
        }

        const FIELDS: &'static [&'static str] = &["code", "neg", "pos"];
        deserializer.deserialize_struct("ThreeWay", FIELDS, ThreeWayVisitor)
    }
}

struct Wheel
{
    code: u8,
    event: String,
}
impl serde::ser::Serialize for Wheel
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Wheel", 2)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("event", &self.event)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Wheel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Code, Event };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "code" => Ok(Field::Code),
                            "event" => Ok(Field::Event),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct WheelVisitor;

        impl<'de> Visitor<'de> for WheelVisitor {
            type Value = Wheel;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Wheel, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let code = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let event = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Wheel { code, event})
            }

            fn visit_map<V>(self, mut map: V) -> Result<Wheel, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut code = None;
                let mut event = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Code => {
                            if code.is_some() {
                                return Err(de::Error::duplicate_field("code"));
                            }
                            code = Some(map.next_value()?);
                        }
                        Field::Event => {
                            if event.is_some() {
                                return Err(de::Error::duplicate_field("event"));
                            }
                            event = Some(map.next_value()?);
                        }
                    }
                }
                let code = code.ok_or_else(|| de::Error::missing_field("code"))?;
                let event = event.ok_or_else(|| de::Error::missing_field("event"))?;
                Ok(Wheel { code, event })
            }
        }

        const FIELDS: &'static [&'static str] = &["code", "event"];
        deserializer.deserialize_struct("Wheel", FIELDS, WheelVisitor)
    }
}


struct Throttle
{
    code: i16,
    event: String,
}

impl serde::ser::Serialize for Throttle
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Throttle", 2)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("event", &self.event)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Throttle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Code, Event };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "code" => Ok(Field::Code),
                            "event" => Ok(Field::Event),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ThrottleVisitor;

        impl<'de> Visitor<'de> for ThrottleVisitor {
            type Value = Throttle;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Throttle, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let code = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let event = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Throttle { code, event})
            }

            fn visit_map<V>(self, mut map: V) -> Result<Throttle, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut code = None;
                let mut event = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Code => {
                            if code.is_some() {
                                return Err(de::Error::duplicate_field("code"));
                            }
                            code = Some(map.next_value()?);
                        }
                        Field::Event => {
                            if event.is_some() {
                                return Err(de::Error::duplicate_field("event"));
                            }
                            event = Some(map.next_value()?);
                        }
                    }
                }
                let code = code.ok_or_else(|| de::Error::missing_field("code"))?;
                let event = event.ok_or_else(|| de::Error::missing_field("event"))?;
                Ok(Throttle { code, event })
            }
        }

        const FIELDS: &'static [&'static str] = &["code", "event"];
        deserializer.deserialize_struct("Throttle", FIELDS, ThrottleVisitor)
    }
}