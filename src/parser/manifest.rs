//! Rainstash Manifest Objects for structs and functions within this library.
//!
//! The structs offer ways to destruct the Option value into its value with
//! error handling, this helps to reduce boilerplate code when making
//! applications using this library.
use std::fmt;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Risk of Rain Item struct, values like name, description and itemClass are required when
/// deserializing and serializing JSON objects. All other values are Option<T> as some items
/// do not contain values and this can help with formatting if outputting.
/// All Option<T> values have match functions for easy matching and error handling.
#[derive(Deserialize, Debug)]
pub struct RiskItem {
    pub name: String,
    pub description: String,
    pub cooldown: Option<f32>,
    pub embryo: Option<String>,
    pub stack: Option<String>,
    pub unlock: Option<String>,
    pub drop: Option<String>,
    #[serde(rename = "hasVideo")]
    pub has_video: Option<bool>,
    #[serde(rename = "maxStacks")]
    pub max_stacks: Option<i32>,
    #[serde(deserialize_with = "deserialize_class_field", rename = "itemClass")]
    pub item_class: RiskClass,
}

/// Display implementation for the RiskItem struct.
/// This function orders the values as they are in the struct.
/// Values that are "N/A" or "0" get displayed in their correct place.
impl fmt::Display for RiskItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {}, \
             Description: {}, \
             Cooldown: {}, \
             Embryo: {}, \
             Stack: {}, \
             Unlock: {}, \
             Drop: {}, \
             Has Video: {}, \
             Max Stack Size: {}, \
             Item Class: {}",
            self.name,
            self.description,
            self.match_cooldown(),
            self.match_embryo(),
            self.match_stack(),
            self.match_unlock(),
            self.match_drop(),
            self.match_video(),
            self.match_stack_size(),
            self.item_class.class_to_string()
        )
    }
}

/// Hash implementation for the RiskItem struct.
impl Hash for RiskItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.description.hash(state);
        self.match_cooldown().to_string().hash(state);
        self.match_embryo().hash(state);
        self.match_stack().hash(state);
        self.match_unlock().hash(state);
        self.match_drop().hash(state);
        self.match_video().hash(state);
        self.match_stack_size().hash(state);
        // TODO: Add ItemClass here.
    }
}

/// RiskItem implementation, contains general functions for matching values.
impl RiskItem {
    /// Safely match Cooldown into a usable value.
    /// Returns cooldown in object if value exists.
    /// Returns 0 if no cooldown is found.
    pub fn match_cooldown(&self) -> f32 {
        match self.cooldown {
            Some(v) => v,
            None => 0f32,
        }
    }

    /// Safely match Embryo into a usable value.
    /// Returns embryo string literal in object if value exists.
    /// Returns "N/A" as a string literal if no embryo is found.
    pub fn match_embryo(&self) -> &str {
        match self.embryo {
            Some(ref v) => v.as_str(),
            None => "N/A",
        }
    }

    /// Safely match Stack into a usable value.
    /// Returns stack string literal in object if value exists.
    /// Returns "N/A" as a string literal if no stack is found.
    pub fn match_stack(&self) -> &str {
        match self.stack {
            Some(ref v) => v.as_str(),
            None => "N/A",
        }
    }

    /// Safely match Unlock into a usable value.
    /// Returns unlock string literal in object if value exists.
    /// Returns "N/A" as a string literal if no unlock is found.
    pub fn match_unlock(&self) -> &str {
        match self.unlock {
            Some(ref v) => v.as_str(),
            None => "N/A",
        }
    }

    /// Safely match Drop into a usable value.
    /// Returns drop string literal in object if value exists.
    /// Returns "N/A" as a string literal if no drop is found.
    pub fn match_drop(&self) -> &str {
        match self.drop {
            Some(ref v) => v.as_str(),
            None => "N/A",
        }
    }

    /// Safely match Video into a usable value.
    /// Returns true if video for item exists in the object.
    /// Returns false if no video is found.
    pub fn match_video(&self) -> bool {
        match self.has_video {
            Some(true) => true,
            Some(false) => false,
            None => false,
        }
    }

    /// Safely match maxStack into a usable value.
    /// Returns stack size in object if value exists.
    /// Returns 0 if no stack size is found.
    pub fn match_stack_size(&self) -> i32 {
        match self.max_stacks {
            Some(v) => v,
            None => 0,
        }
    }
}

/// Risk of Rain Item Class struct, this enum is used when deserializing and serializing from
/// Strings contained in the JSON struct under the "itemClass" value.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub enum RiskClass {
    White,
    Green,
    Red,
    Yellow,
    Orange,
    Purple,
    Grey,
    None,
}

/// RiskClass implementation, contains general functions for matching values.
impl RiskClass {
    /// Safely match RiskClass colors into a usable string literal.
    /// Returns the name as a string literal if item exists in the object.
    /// Returns "None" if item does not contain a RiskClass in the object.
    pub fn class_to_string(&self) -> &str {
        match self {
            RiskClass::White => "White",
            RiskClass::Green => "Green",
            RiskClass::Red => "Red",
            RiskClass::Yellow => "Yellow",
            RiskClass::Orange => "Orange",
            RiskClass::Purple => "Purple",
            RiskClass::Grey => "Grey",
            RiskClass::None => "None",
        }
    }
}

/// Custom parsing function for itemClass string value into RiskClass Enum using Serde.
/// If value does not exist on a object it will return "None" for the variable.
fn deserialize_class_field<'de, D>(de: D) -> Result<RiskClass, D::Error>
where
    D: Deserializer<'de>,
{
    let result: Value = Deserialize::deserialize(de)?;
    match result {
        Value::String(ref s) if &*s == "white" => Ok(RiskClass::White),
        Value::String(ref s) if &*s == "green" => Ok(RiskClass::Green),
        Value::String(ref s) if &*s == "red" => Ok(RiskClass::Red),
        Value::String(ref s) if &*s == "yellow" => Ok(RiskClass::Yellow),
        Value::String(ref s) if &*s == "orange" => Ok(RiskClass::Orange),
        Value::String(ref s) if &*s == "purple" => Ok(RiskClass::Purple),
        Value::String(ref s) if &*s == "misc" => Ok(RiskClass::Grey),
        _ => Ok(RiskClass::None),
    }
}
