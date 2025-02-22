use crate::output::Output;
use dynfmt::{Argument, FormatArgs};
use std::collections::HashMap;

/// Parameters define the parameters sent into each step. The parameters are used to fill in the prompt template, and are also filled in by the output of the previous step. Parameters have a special key, `text`, which is used as a default key for simple use cases.
///
/// Parameters also implement a few convenience conversion traits to make it easier to work with them.
///
/// # Examples
///
/// **Creating a default parameter from a string**
/// ```
/// use llm_chain::Parameters;
/// let p: Parameters = "Hello world!".into();
/// assert_eq!(p.get("text").unwrap().as_str(), "Hello world!");
/// ```
/// **Creating a list of parameters from a list of pairs**
/// ```
/// use llm_chain::Parameters;
/// let p: Parameters = vec![("text", "Hello world!"), ("name", "John Doe")].into();
/// assert_eq!(p.get("text").unwrap().as_str(), "Hello world!");
/// assert_eq!(p.get("name").unwrap().as_str(), "John Doe");
/// ```
#[derive(Clone, Default, Debug)]
pub struct Parameters {
    map: HashMap<String, String>,
    non_strict: bool,
}

const TEXT_KEY: &str = "text";

impl FormatArgs for Parameters {
    fn get_index(&self, index: usize) -> Result<Option<Argument<'_>>, ()> {
        let res = if index == 0 {
            self.get_key(TEXT_KEY)?
        } else {
            self.map.get_index(index)?
        };

        if self.non_strict && res.is_none() {
            Ok(Some(&""))
        } else {
            Ok(res)
        }
    }

    fn get_key(&self, key: &str) -> Result<Option<Argument<'_>>, ()> {
        let res = self.map.get_key(key)?;

        if self.non_strict && res.is_none() {
            Ok(Some(&""))
        } else {
            Ok(res)
        }
    }
}

impl Parameters {
    /// Creates a new empty set of parameters.
    pub fn new() -> Parameters {
        Default::default()
    }
    /// Creates a new set of parameters with a single key, `text`, set to the given value.
    pub fn new_with_text<T: Into<String>>(text: T) -> Parameters {
        let mut map = HashMap::new();
        map.insert(TEXT_KEY.to_string(), text.into());
        Parameters {
            map,
            ..Default::default()
        }
    }
    /// Copies the parameters and adds a new key-value pair.
    pub fn with<K: Into<String>, V: Into<String>>(&self, key: K, value: V) -> Parameters {
        let mut copy = self.clone();
        copy.map.insert(key.into(), value.into());
        copy
    }
    /// Copies the parameters and adds a new key-value pair with the key `text`, which is the default key.
    pub fn with_text<K: Into<String>>(&self, text: K) -> Parameters {
        self.with(TEXT_KEY, text)
    }
    pub async fn with_text_from_output<O: Output>(&self, output: &O) -> Parameters {
        output
            .primary_textual_output()
            .await
            .map_or(self.clone(), |text| self.with_text(text))
    }
    /// Combines two sets of parameters, returning a new set of parameters with all the keys from both sets.
    pub fn combine(&self, other: &Parameters) -> Parameters {
        let mut copy = self.clone();
        copy.map.extend(other.map.clone());
        copy
    }
    /// Returns the value of the given key, or `None` if the key does not exist.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn new_non_strict() -> Parameters {
        Parameters {
            non_strict: true,
            ..Default::default()
        }
    }

    pub fn forked<K, V1, V2>(&self, key: K, a: V1, b: V2) -> (Parameters, Parameters)
    where
        K: Into<String> + Copy,
        V1: Into<String>,
        V2: Into<String>,
    {
        let mut copy = self.clone();
        copy.map.insert(key.into(), a.into());
        let mut copy2 = self.clone();
        copy2.map.insert(key.into(), b.into());
        (copy, copy2)
    }

    pub fn forked_text<V1, V2>(&self, a: V1, b: V2) -> (Parameters, Parameters)
    where
        V1: Into<String>,
        V2: Into<String>,
    {
        self.forked(TEXT_KEY, a, b)
    }

    pub fn get_text(&self) -> Option<&String> {
        self.get(TEXT_KEY)
    }

    #[cfg(feature = "tera")]
    pub fn to_tera(&self) -> tera::Context {
        let mut context = tera::Context::new();
        for (key, value) in self.map.iter() {
            context.insert(key, value);
        }
        context
    }
}

impl From<String> for Parameters {
    fn from(text: String) -> Self {
        Parameters::new_with_text(text)
    }
}

impl From<&str> for Parameters {
    fn from(text: &str) -> Self {
        Parameters::new_with_text(text)
    }
}

impl From<HashMap<String, String>> for Parameters {
    fn from(map: HashMap<String, String>) -> Self {
        Parameters {
            map,
            ..Default::default()
        }
    }
}

impl From<Vec<(String, String)>> for Parameters {
    fn from(data: Vec<(String, String)>) -> Self {
        let map: HashMap<String, String> = data.into_iter().collect();
        Parameters {
            map,
            ..Default::default()
        }
    }
}

impl From<Vec<(&str, &str)>> for Parameters {
    fn from(data: Vec<(&str, &str)>) -> Self {
        let map: HashMap<String, String> = data
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        Parameters {
            map,
            ..Default::default()
        }
    }
}
