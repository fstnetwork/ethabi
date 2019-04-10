use ParamType;

/// Output formatter for param type.
pub struct Writer;

impl Writer {
	/// Returns string which is a formatted represenation of param.
	pub fn write(param: &ParamType) -> String {
		match *param {
			ParamType::Address => "address".to_string(),
			ParamType::Bytes => "bytes".to_string(),
			ParamType::FixedBytes(len) => format!("bytes{}", len),
			ParamType::Int(len) => format!("int{}", len),
			ParamType::Uint(len) => format!("uint{}", len),
			ParamType::Bool => "bool".to_string(),
			ParamType::String => "string".to_string(),
			ParamType::FixedArray(ref param, len) => format!("{}[{}]", Self::write(param), len),
			ParamType::Array(ref param) => format!("{}[]", Self::write(param)),
			ParamType::Tuple(ref params) => format!("({})", params.iter().map(Self::write).collect::<Vec<String>>().join(",")),
		}
	}
}

#[cfg(test)]
mod tests {
	use ParamType;
	use super::Writer;

	#[test]
	fn test_write_param() {
		assert_eq!(Writer::write(&ParamType::Address), "address".to_owned());
		assert_eq!(Writer::write(&ParamType::Bytes), "bytes".to_owned());
		assert_eq!(Writer::write(&ParamType::FixedBytes(32)), "bytes32".to_owned());
		assert_eq!(Writer::write(&ParamType::Uint(256)), "uint256".to_owned());
		assert_eq!(Writer::write(&ParamType::Int(64)), "int64".to_owned());
		assert_eq!(Writer::write(&ParamType::Bool), "bool".to_owned());
		assert_eq!(Writer::write(&ParamType::String), "string".to_owned());
		assert_eq!(Writer::write(&ParamType::Array(Box::new(ParamType::Bool))), "bool[]".to_owned());
		assert_eq!(Writer::write(&ParamType::FixedArray(Box::new(ParamType::String), 2)), "string[2]".to_owned());
		assert_eq!(Writer::write(&ParamType::FixedArray(Box::new(ParamType::Array(Box::new(ParamType::Bool))), 2)), "bool[][2]".to_owned());
		assert_eq!(
			Writer::write(&ParamType::Tuple(vec![
				ParamType::FixedArray(Box::new(ParamType::Tuple(vec![ParamType::Array(Box::new(ParamType::Uint(256))), ParamType::Bool])), 3),
				ParamType::String
			])),
			"((uint256[],bool)[3],string)"
		);
	}
}
