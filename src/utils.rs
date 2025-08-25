pub mod single_element_array {
	use serde::{ser::SerializeSeq, Deserialize, Deserializer, Serialize, Serializer};

	pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
	where
		T: Serialize,
		S: Serializer,
	{
		let mut seq = serializer.serialize_seq(Some(1))?;
		seq.serialize_element(value)?;
		seq.end()
	}

	pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
	where
		T: Deserialize<'de>,
		D: Deserializer<'de>,
	{
		let params = Vec::<T>::deserialize(deserializer)?;
		if params.len() != 1 {
			return Err(serde::de::Error::custom("expected a single-element array"));
		}
		Ok(params.into_iter().next().unwrap())
	}
}

