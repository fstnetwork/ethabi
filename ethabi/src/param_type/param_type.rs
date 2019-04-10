//! Function and event param types.

use std::fmt;
use super::Writer;

/// Function and event param types.
#[derive(Debug, Clone, PartialEq)]
pub enum ParamType {
	// elementary types:

	/// Unsigned integer.
	Uint(usize),
	/// Two’s complement signed integer.
	Int(usize),
	/// Address.
	Address,
	/// Boolean.
	Bool,
	/// Vector of bytes with fixed size.
	FixedBytes(usize),

	// fixed-size array types:

	/// Array with fixed size.
	FixedArray(Box<ParamType>, usize),

	// non-fixed-size types:

	/// Bytes.
	Bytes,
	/// String.
	String,
	/// Array of unknown size.
	Array(Box<ParamType>),
	/// Tuple.
	Tuple(Vec<ParamType>),
}

impl fmt::Display for ParamType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", Writer::write(self))
	}
}

impl ParamType {
    /// returns whether a zero length byte slice (`0x`) is
    /// a valid encoded form of this param type
    pub fn is_empty_bytes_valid_encoding(&self) -> bool {
        match self {
            ParamType::FixedBytes(len) => *len == 0,
            ParamType::FixedArray(_, len) => *len == 0,
            _ => false,
        }
    }

	/// returns wheater this param type is dynamic
	pub fn is_dynamic(&self) -> bool {
		match self {
			ParamType::Bytes | ParamType::String | ParamType::Array(_) => true,
			ParamType::FixedArray(ref param, _) => param.is_dynamic(),
			ParamType::Tuple(ref params) => params.iter().any(|ref param| param.is_dynamic()),
			_ => false,
		}
	}
}

#[cfg(test)]
mod tests {
	use ParamType;

	#[test]
	fn test_param_type_display() {
		assert_eq!(format!("{}", ParamType::Address), "address".to_owned());
		assert_eq!(format!("{}", ParamType::Bytes), "bytes".to_owned());
		assert_eq!(format!("{}", ParamType::FixedBytes(32)), "bytes32".to_owned());
		assert_eq!(format!("{}", ParamType::Uint(256)), "uint256".to_owned());
		assert_eq!(format!("{}", ParamType::Int(64)), "int64".to_owned());
		assert_eq!(format!("{}", ParamType::Bool), "bool".to_owned());
		assert_eq!(format!("{}", ParamType::String), "string".to_owned());
		assert_eq!(format!("{}", ParamType::Array(Box::new(ParamType::Bool))), "bool[]".to_owned());
		assert_eq!(format!("{}", ParamType::FixedArray(Box::new(ParamType::Uint(256)), 2)), "uint256[2]".to_owned());
		assert_eq!(format!("{}", ParamType::FixedArray(Box::new(ParamType::String), 2)), "string[2]".to_owned());
		assert_eq!(format!("{}", ParamType::FixedArray(Box::new(ParamType::Array(Box::new(ParamType::Bool))), 2)), "bool[][2]".to_owned());
		assert_eq!(format!("{}", ParamType::Tuple(vec![ParamType::Bool, ParamType::Uint(256)])), "(bool,uint256)".to_owned());
		assert_eq!(format!("{}", ParamType::Tuple(vec![ParamType::Bool, ParamType::String])), "(bool,string)".to_owned());
	}

	#[test]
	fn test_is_dynamic() {
		assert_eq!(ParamType::Address.is_dynamic(), false);
		assert_eq!(ParamType::Bytes.is_dynamic(), true);
		assert_eq!(ParamType::FixedBytes(32).is_dynamic(), false);
		assert_eq!(ParamType::Uint(256).is_dynamic(), false);
		assert_eq!(ParamType::Int(64).is_dynamic(), false);
		assert_eq!(ParamType::Bool.is_dynamic(), false);
		assert_eq!(ParamType::String.is_dynamic(), true);
		assert_eq!(ParamType::Array(Box::new(ParamType::Bool)).is_dynamic(), true);
		assert_eq!(ParamType::FixedArray(Box::new(ParamType::Uint(256)), 2).is_dynamic(), false);
		assert_eq!(ParamType::FixedArray(Box::new(ParamType::String), 2).is_dynamic(), true);
		assert_eq!(ParamType::FixedArray(Box::new(ParamType::Array(Box::new(ParamType::Bool))), 2).is_dynamic(), true);
		assert_eq!(ParamType::Tuple(vec![ParamType::Bool, ParamType::Uint(256)]).is_dynamic(), false);
		assert_eq!(ParamType::Tuple(vec![ParamType::Bool, ParamType::String]).is_dynamic(), true);
	}
}
