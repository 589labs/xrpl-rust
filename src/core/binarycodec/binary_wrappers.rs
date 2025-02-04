use super::definitions::*;
use super::types::TryFromParser;
use crate::core::binarycodec::exceptions::XRPLBinaryCodecException;
use crate::core::binarycodec::utils::*;
use crate::core::exceptions::XRPLCoreException;
use crate::core::exceptions::XRPLCoreResult;
use crate::utils::ToBytes;
use alloc::borrow::ToOwned;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::convert::TryInto;

/// Serializes JSON to XRPL binary format.
pub type BinarySerializer = Vec<u8>;

/// Deserializes from hex-encoded XRPL binary format to
/// serde JSON fields and values.
///
/// # Examples
///
/// ## Basic usage
///
/// ```
/// use xrpl::core::binarycodec::BinaryParser;
/// use xrpl::core::Parser;
/// use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
///
/// let test_bytes: &[u8] = &[0, 17, 34, 51, 68, 85, 102];
/// let binary_parser: BinaryParser = BinaryParser::from(test_bytes);
///
/// assert_eq!(binary_parser, test_bytes[..]);
/// ```
#[derive(Debug, Clone)]
pub struct BinaryParser(Vec<u8>);

/// Helper function for length-prefixed fields including
/// Blob types and some AccountID types. Calculates the
/// prefix of variable length bytes.
///
/// The length of the prefix is 1-3 bytes depending on the
/// length of the contents:
/// Content length <= 192 bytes: prefix is 1 byte
/// 192 bytes < Content length <= 12480 bytes: prefix is 2 bytes
/// 12480 bytes < Content length <= 918744 bytes: prefix is 3 bytes
///
/// See Length Prefixing:
/// `<https://xrpl.org/serialization.html#length-prefixing>`
fn _encode_variable_length_prefix(length: &usize) -> XRPLCoreResult<Vec<u8>> {
    if length <= &MAX_SINGLE_BYTE_LENGTH {
        Ok([*length as u8].to_vec())
    } else if length < &MAX_DOUBLE_BYTE_LENGTH {
        let mut bytes = vec![];
        let b_length = *length - (MAX_SINGLE_BYTE_LENGTH + 1);
        let val_a: u8 = ((b_length >> 8) + (MAX_SINGLE_BYTE_LENGTH + 1))
            .try_into()
            .map_err(XRPLBinaryCodecException::TryFromIntError)?;
        let val_b: u8 = (b_length & 0xFF)
            .try_into()
            .map_err(XRPLBinaryCodecException::TryFromIntError)?;

        bytes.extend_from_slice(&[val_a]);
        bytes.extend_from_slice(&[val_b]);

        Ok(bytes)
    } else if length <= &MAX_LENGTH_VALUE {
        let mut bytes = vec![];
        let b_length = *length - MAX_DOUBLE_BYTE_LENGTH;
        let val_a: u8 = ((MAX_SECOND_BYTE_VALUE + 1) + (b_length >> 16))
            .try_into()
            .map_err(XRPLBinaryCodecException::TryFromIntError)?;
        let val_b: u8 = ((b_length >> 8) & 0xFF)
            .try_into()
            .map_err(XRPLBinaryCodecException::TryFromIntError)?;
        let val_c: u8 = (b_length & 0xFF)
            .try_into()
            .map_err(XRPLBinaryCodecException::TryFromIntError)?;

        bytes.extend_from_slice(&[val_a]);
        bytes.extend_from_slice(&[val_b]);
        bytes.extend_from_slice(&[val_c]);

        Ok(bytes)
    } else {
        Err(XRPLBinaryCodecException::InvalidVariableLengthTooLarge {
            max: MAX_LENGTH_VALUE,
        }
        .into())
    }
}

pub trait Parser {
    /// Peek the first byte of the BinaryParser.
    ///
    /// # Examples
    ///
    /// ## Basic usage
    ///
    /// ```
    /// use xrpl::core::binarycodec::BinaryParser;
    /// use xrpl::core::Parser;
    /// use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
    ///
    /// let test_bytes: &[u8] = &[0, 17, 34, 51, 68, 85, 102];
    /// let binary_parser: BinaryParser = BinaryParser::from(test_bytes);
    /// let first_byte: Option<[u8; 1]> = binary_parser.peek();
    ///
    /// assert_eq!(Some([test_bytes[0]; 1]), first_byte);
    /// ```
    fn peek(&self) -> Option<[u8; 1]>;

    /// Consume the first n bytes of the BinaryParser.
    ///
    /// # Examples
    ///
    /// ## Basic usage
    ///
    /// ```
    /// use xrpl::core::binarycodec::BinaryParser;
    /// use xrpl::core::Parser;
    /// use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
    /// use xrpl::core::exceptions::XRPLCoreException;
    ///
    /// let test_bytes: &[u8] = &[0, 17, 34, 51, 68, 85, 102];
    /// let mut binary_parser: BinaryParser = BinaryParser::from(test_bytes);
    ///
    /// match binary_parser.skip_bytes(4) {
    ///     Ok(parser) => assert_eq!(*parser, test_bytes[4..]),
    ///     Err(e) => match e {
    ///         XRPLCoreException::XRPLBinaryCodecError(XRPLBinaryCodecException::UnexpectedParserSkipOverflow {
    ///             max: _,
    ///             found: _,
    ///         }) => assert!(false),
    ///         _ => assert!(false)
    ///     }
    /// }
    /// ```
    fn skip_bytes(&mut self, n: usize) -> XRPLCoreResult<&Self>;

    /// Consume and return the first n bytes of the BinaryParser.
    ///
    /// # Examples
    ///
    /// ## Basic usage
    ///
    /// ```
    /// use xrpl::core::binarycodec::BinaryParser;
    /// use xrpl::core::Parser;
    /// use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
    /// use xrpl::core::exceptions::XRPLCoreException;
    ///
    /// let test_bytes: &[u8] = &[0, 17, 34, 51, 68, 85, 102];
    /// let mut binary_parser: BinaryParser = BinaryParser::from(test_bytes);
    ///
    /// match binary_parser.read(5) {
    ///     Ok(data) => assert_eq!(test_bytes[..5], data),
    ///     Err(e) => match e {
    ///         XRPLCoreException::XRPLBinaryCodecError(XRPLBinaryCodecException::UnexpectedParserSkipOverflow {
    ///             max: _,
    ///             found: _,
    ///         }) => assert!(false),
    ///         _ => assert!(false)
    ///     }
    /// }
    /// ```
    fn read(&mut self, n: usize) -> XRPLCoreResult<Vec<u8>>;

    /// Read 1 byte from parser and return as unsigned int.
    ///
    /// # Examples
    ///
    /// ## Basic usage
    ///
    /// ```
    /// use xrpl::core::binarycodec::BinaryParser;
    /// use xrpl::core::Parser;
    /// use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
    /// use xrpl::core::exceptions::XRPLCoreException;
    ///
    /// let test_bytes: &[u8] = &[0, 17, 34, 51, 68, 85, 102];
    /// let mut binary_parser: BinaryParser = BinaryParser::from(test_bytes);
    ///
    /// match binary_parser.read_uint8() {
    ///     Ok(data) => assert_eq!(0, data),
    ///     Err(e) => match e {
    ///         XRPLCoreException::XRPLBinaryCodecError(XRPLBinaryCodecException::UnexpectedParserSkipOverflow {
    ///             max: _,
    ///             found: _,
    ///         }) => assert!(false),
    ///         _ => assert!(false)
    ///     }
    /// }
    /// ```
    fn read_uint8(&mut self) -> XRPLCoreResult<u8>;

    /// Read 2 bytes from parser and return as unsigned int.
    ///
    /// # Examples
    ///
    /// ## Basic usage
    ///
    /// ```
    /// use xrpl::core::binarycodec::BinaryParser;
    /// use xrpl::core::Parser;
    /// use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
    /// use xrpl::core::exceptions::XRPLCoreException;
    ///
    /// let test_bytes: &[u8] = &[0, 17, 34, 51, 68, 85, 102];
    /// let mut binary_parser: BinaryParser = BinaryParser::from(test_bytes);
    ///
    /// match binary_parser.read_uint16() {
    ///     Ok(data) => assert_eq!(17, data),
    ///     Err(e) => match e {
    ///         XRPLCoreException::XRPLBinaryCodecError(XRPLBinaryCodecException::UnexpectedParserSkipOverflow {
    ///             max: _,
    ///             found: _,
    ///         }) => assert!(false),
    ///         _ => assert!(false)
    ///     }
    /// }
    /// ```
    fn read_uint16(&mut self) -> XRPLCoreResult<u16>;

    /// Read 4 bytes from parser and return as unsigned int.
    ///
    /// # Examples
    ///
    /// ## Basic usage
    ///
    /// ```
    /// use xrpl::core::binarycodec::BinaryParser;
    /// use xrpl::core::Parser;
    /// use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
    /// use xrpl::core::exceptions::XRPLCoreException;
    ///
    /// let test_bytes: &[u8] = &[0, 17, 34, 51, 68, 85, 102];
    /// let mut binary_parser: BinaryParser = BinaryParser::from(test_bytes);
    ///
    /// match binary_parser.read_uint32() {
    ///     Ok(data) => assert_eq!(1122867, data),
    ///     Err(e) => match e {
    ///         XRPLCoreException::XRPLBinaryCodecError(XRPLBinaryCodecException::UnexpectedParserSkipOverflow {
    ///             max: _,
    ///             found: _,
    ///         }) => assert!(false),
    ///         _ => assert!(false)
    ///     }
    /// }
    /// ```
    fn read_uint32(&mut self) -> XRPLCoreResult<u32>;

    /// Returns whether the binary parser has finished
    /// parsing (e.g. there is nothing left in the buffer
    /// that needs to be processed).
    ///
    /// # Examples
    ///
    /// ## Basic usage
    ///
    /// ```
    /// use xrpl::core::binarycodec::BinaryParser;
    /// use xrpl::core::Parser;
    /// use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
    /// use xrpl::core::exceptions::XRPLCoreException;
    /// extern crate alloc;
    /// use alloc::vec;
    ///
    /// let empty: &[u8] = &[];
    /// let mut buffer: Vec<u8> = vec![];
    /// let test_bytes: &[u8] = &[0, 17, 34, 51, 68, 85, 102];
    /// let mut binary_parser: BinaryParser = BinaryParser::from(test_bytes);
    ///
    /// while !binary_parser.is_end(None) {
    ///     match binary_parser.read(1) {
    ///         Ok(data) => buffer.extend_from_slice(&data),
    ///         Err(e) => match e {
    ///             XRPLCoreException::XRPLBinaryCodecError(XRPLBinaryCodecException::UnexpectedParserSkipOverflow {
    ///                 max: _,
    ///                 found: _,
    ///             }) => assert!(false),
    ///             _ => assert!(false)
    ///         }
    ///     }
    /// }
    ///
    /// assert_eq!(test_bytes, &buffer[..]);
    /// // The BinaryParser is emptied as it is read.
    /// assert_eq!(binary_parser, empty[..]);
    ///
    /// ```
    fn is_end(&self, custom_end: Option<usize>) -> bool;

    /// Reads a variable length encoding prefix and returns
    /// the encoded length. The formula for decoding a length
    /// prefix is described in Length Prefixing.
    ///
    /// See Length Prefixing:
    /// `<https://xrpl.org/serialization.html#length-prefixing>`
    ///
    /// # Examples
    ///
    /// ## Basic usage
    ///
    /// ```
    /// use xrpl::core::binarycodec::BinaryParser;
    /// use xrpl::core::Parser;
    /// use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
    /// use xrpl::core::exceptions::XRPLCoreException;
    ///
    /// let test_bytes: &[u8] = &[6, 17, 34, 51, 68, 85, 102];
    /// let mut binary_parser: BinaryParser = BinaryParser::from(test_bytes);
    ///
    /// match binary_parser.read_length_prefix() {
    ///     Ok(data) => assert_eq!(6, data),
    ///     Err(e) => match e {
    ///         XRPLCoreException::XRPLBinaryCodecError(XRPLBinaryCodecException::UnexpectedLengthPrefixRange {
    ///             min: _, max: _
    ///         }) => assert!(false),
    ///         _ => assert!(false)
    ///     }
    /// }
    fn read_length_prefix(&mut self) -> XRPLCoreResult<usize>;

    /// Reads field ID from BinaryParser and returns as
    /// a FieldHeader object.
    fn read_field_header(&mut self) -> XRPLCoreResult<FieldHeader>;

    /// Read the field ordinal at the head of the
    /// BinaryParser and return a FieldInstance object
    /// representing information about the field
    /// containedin the following bytes.
    fn read_field(&mut self) -> XRPLCoreResult<FieldInstance>;

    /// Read next bytes from BinaryParser as the given type.
    fn read_type<T: TryFromParser>(&mut self) -> XRPLCoreResult<T, T::Error>;

    /// Read value of the type specified by field from
    /// the BinaryParser.
    fn read_field_value<T: TryFromParser>(
        &mut self,
        field: &FieldInstance,
    ) -> XRPLCoreResult<T, T::Error>
    where
        T::Error: From<XRPLCoreException>;
}

pub trait Serialization {
    /// Write given bytes to this BinarySerializer.
    ///
    /// # Examples
    ///
    /// ## Basic usage
    ///
    /// ```
    /// use xrpl::core::binarycodec::BinarySerializer;
    /// use xrpl::core::binarycodec::Serialization;
    ///
    /// let mut test_bytes: Vec<u8> = [0, 17, 34, 51, 68, 85, 102].to_vec();
    /// let mut serializer: BinarySerializer = BinarySerializer::new();
    ///
    /// serializer.append(&mut test_bytes.to_owned());
    /// assert_eq!(test_bytes, serializer);
    /// ```
    fn append(&mut self, bytes: &[u8]) -> &Self;

    /// Write a variable length encoded value to
    /// the BinarySerializer.
    ///
    /// # Examples
    ///
    /// ## Basic usage
    ///
    /// ```
    /// use xrpl::core::binarycodec::BinarySerializer;
    /// use xrpl::core::binarycodec::Serialization;
    ///
    /// let expected: Vec<u8> = [3, 0, 17, 34].to_vec();
    /// let mut test_bytes: Vec<u8> = [0, 17, 34].to_vec();
    /// let mut serializer: BinarySerializer = BinarySerializer::new();
    ///
    /// serializer.write_length_encoded(&mut test_bytes, true);
    /// assert_eq!(expected, serializer);
    /// ```
    fn write_length_encoded(&mut self, value: &[u8], encode_value: bool) -> &Self;

    /// Write field and value to the buffer.
    ///
    /// # Examples
    ///
    /// ## Basic usage
    ///
    /// ```
    /// use xrpl::core::binarycodec::BinarySerializer;
    /// use xrpl::core::binarycodec::Serialization;
    /// use xrpl::core::binarycodec::definitions::FieldInstance;
    /// use xrpl::core::binarycodec::definitions::FieldInfo;
    /// use xrpl::core::binarycodec::definitions::FieldHeader;
    ///
    /// let field_header: FieldHeader = FieldHeader {
    ///     type_code: -2,
    ///     field_code: 0,
    /// };
    ///
    /// let field_info: FieldInfo = FieldInfo {
    ///     nth: 0,
    ///     is_vl_encoded: false,
    ///     is_serialized: false,
    ///     is_signing_field: false,
    ///     r#type: "Unknown".to_string(),
    /// };
    ///
    /// let field_instance = FieldInstance::new(&field_info, "Generic", field_header);
    /// let expected: Vec<u8> = [224, 0, 17, 34].to_vec();
    /// let test_bytes: Vec<u8> = [0, 17, 34].to_vec();
    /// let mut serializer: BinarySerializer = BinarySerializer::new();
    ///
    /// serializer.write_field_and_value(field_instance, &test_bytes, false);
    /// assert_eq!(expected, serializer);
    /// ```
    fn write_field_and_value(
        &mut self,
        field: FieldInstance,
        value: &[u8],
        is_unl_modify_workaround: bool,
    ) -> &Self;
}

impl Serialization for BinarySerializer {
    fn append(&mut self, bytes: &[u8]) -> &Self {
        self.extend_from_slice(bytes);
        self
    }

    fn write_length_encoded(&mut self, value: &[u8], encode_value: bool) -> &Self {
        let mut byte_object: Vec<u8> = Vec::new();
        if encode_value {
            // write value to byte_object
            byte_object.extend_from_slice(value);
        }
        // TODO Handle unwrap better
        let length_prefix = _encode_variable_length_prefix(&byte_object.len()).unwrap();

        self.extend_from_slice(&length_prefix);
        self.extend_from_slice(&byte_object);

        self
    }

    fn write_field_and_value(
        &mut self,
        field: FieldInstance,
        value: &[u8],
        is_unl_modify_workaround: bool,
    ) -> &Self {
        self.extend_from_slice(&field.header.to_bytes());

        if field.is_vl_encoded {
            self.write_length_encoded(value, !is_unl_modify_workaround);
        } else {
            self.extend_from_slice(value);
        }

        self
    }
}

/// Peek the first byte of the BinaryParser.
impl Parser for BinaryParser {
    fn peek(&self) -> Option<[u8; 1]> {
        if !self.0.is_empty() {
            Some(self.0[0].to_be_bytes())
        } else {
            None
        }
    }

    fn skip_bytes(&mut self, n: usize) -> XRPLCoreResult<&Self> {
        if n > self.0.len() {
            Err(XRPLBinaryCodecException::UnexpectedParserSkipOverflow {
                max: self.0.len(),
                found: n,
            }
            .into())
        } else {
            self.0 = self.0[n..].to_vec();
            Ok(self)
        }
    }

    fn read(&mut self, n: usize) -> XRPLCoreResult<Vec<u8>> {
        let first_n_bytes = self.0[..n].to_owned();

        self.skip_bytes(n)?;
        Ok(first_n_bytes)
    }

    fn read_uint8(&mut self) -> XRPLCoreResult<u8> {
        let result = self.read(1)?;
        Ok(u8::from_be_bytes(result.try_into().or(Err(
            XRPLBinaryCodecException::InvalidReadFromBytesValue,
        ))?))
    }

    fn read_uint16(&mut self) -> XRPLCoreResult<u16> {
        let result = self.read(2)?;
        Ok(u16::from_be_bytes(result.try_into().or(Err(
            XRPLBinaryCodecException::InvalidReadFromBytesValue,
        ))?))
    }

    fn read_uint32(&mut self) -> XRPLCoreResult<u32> {
        let result = self.read(4)?;
        Ok(u32::from_be_bytes(result.try_into().or(Err(
            XRPLBinaryCodecException::InvalidReadFromBytesValue,
        ))?))
    }

    fn is_end(&self, custom_end: Option<usize>) -> bool {
        if let Some(end) = custom_end {
            self.0.len() <= end
        } else {
            self.0.is_empty()
        }
    }

    fn read_length_prefix(&mut self) -> XRPLCoreResult<usize> {
        let byte1: usize = self.read_uint8()? as usize;

        match byte1 {
            // If the field contains 0 to 192 bytes of data,
            // the first byte defines the length of the contents.
            x if x <= MAX_SINGLE_BYTE_LENGTH => Ok(byte1),
            // If the field contains 193 to 12480 bytes of data,
            // the first two bytes indicate the length of the
            // field with the following formula:
            // 193 + ((byte1 - 193) * 256) + byte2
            x if x <= MAX_SECOND_BYTE_VALUE => {
                let byte2: usize = self.read_uint8()? as usize;
                Ok((MAX_SINGLE_BYTE_LENGTH + 1)
                    + ((byte1 - (MAX_SINGLE_BYTE_LENGTH + 1)) * MAX_BYTE_VALUE)
                    + byte2)
            }
            // If the field contains 12481 to 918744 bytes of data,
            // the first three bytes indicate the length of the
            // field with the following formula:
            // 12481 + ((byte1 - 241) * 65536) + (byte2 * 256) + byte3
            x if x <= 254 => {
                let byte2: usize = self.read_uint8()? as usize;
                let byte3: usize = self.read_uint8()? as usize;

                Ok(MAX_DOUBLE_BYTE_LENGTH
                    + ((byte1 - (MAX_SECOND_BYTE_VALUE + 1)) * MAX_DOUBLE_BYTE_VALUE)
                    + (byte2 * MAX_BYTE_VALUE)
                    + byte3)
            }
            _ => {
                Err(XRPLBinaryCodecException::UnexpectedLengthPrefixRange { min: 1, max: 3 }.into())
            }
        }
    }

    fn read_field_header(&mut self) -> XRPLCoreResult<FieldHeader> {
        let mut type_code: i16 = self.read_uint8()? as i16;
        let mut field_code: i16 = type_code & 15;

        type_code >>= 4;

        if type_code == 0 {
            type_code = self.read_uint8()? as i16;

            if type_code == 0 || type_code < 16 {
                return Err(
                    XRPLBinaryCodecException::UnexpectedTypeCodeRange { min: 1, max: 16 }.into(),
                );
            };
        };

        if field_code == 0 {
            field_code = self.read_uint8()? as i16;

            if field_code == 0 || field_code < 16 {
                return Err(
                    XRPLBinaryCodecException::UnexpectedFieldCodeRange { min: 1, max: 16 }.into(),
                );
            };
        };

        Ok(FieldHeader {
            type_code,
            field_code,
        })
    }

    fn read_field(&mut self) -> XRPLCoreResult<FieldInstance> {
        let field_header = self.read_field_header()?;
        let field_name = get_field_name_from_header(&field_header);

        if let Some(name) = field_name {
            if let Some(instance) = get_field_instance(name) {
                return Ok(instance);
            };
        };

        Err(XRPLBinaryCodecException::UnknownFieldName.into())
    }

    fn read_type<T: TryFromParser>(&mut self) -> XRPLCoreResult<T, T::Error> {
        T::from_parser(self, None)
    }

    fn read_field_value<T: TryFromParser>(
        &mut self,
        field: &FieldInstance,
    ) -> XRPLCoreResult<T, T::Error>
    where
        T::Error: From<XRPLCoreException>,
    {
        if field.is_vl_encoded {
            let length = self.read_length_prefix()?;
            T::from_parser(self, Some(length))
        } else {
            T::from_parser(self, None)
        }
    }
}

impl From<&[u8]> for BinaryParser {
    fn from(hex_bytes: &[u8]) -> Self {
        BinaryParser(hex_bytes.to_vec())
    }
}

impl From<Vec<u8>> for BinaryParser {
    fn from(hex_bytes: Vec<u8>) -> Self {
        BinaryParser(hex_bytes)
    }
}

impl TryFrom<&str> for BinaryParser {
    type Error = XRPLCoreException;

    fn try_from(hex_bytes: &str) -> XRPLCoreResult<Self, Self::Error> {
        Ok(BinaryParser(hex::decode(hex_bytes)?))
    }
}

impl PartialEq<[u8]> for BinaryParser {
    fn eq(&self, bytes: &[u8]) -> bool {
        self.0 == bytes
    }
}

impl PartialEq<Vec<u8>> for BinaryParser {
    fn eq(&self, bytes: &Vec<u8>) -> bool {
        &self.0 == bytes
    }
}

impl ExactSizeIterator for BinaryParser {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Iterator for BinaryParser {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_end(None) {
            None
        } else {
            Some(self.read_uint8().expect("BinaryParser::next"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::alloc::string::ToString;
    use alloc::string::String;

    const TEST_HEX: &str = "00112233445566";

    #[test]
    fn test_binaryparser_from() {
        let test_bytes: Vec<u8> = hex::decode(TEST_HEX).expect("");
        let ref_bytes: &[u8] = test_bytes.as_ref();
        let slice_parser = BinaryParser::from(ref_bytes);
        let vec_parser = BinaryParser::from(test_bytes.to_owned());

        assert_eq!(slice_parser, test_bytes[..]);
        assert_eq!(vec_parser, test_bytes[..]);
    }

    #[test]
    fn test_binaryparser_try_from() {
        let test_bytes: Vec<u8> = hex::decode(TEST_HEX).expect("");
        let string_parser = BinaryParser::try_from(TEST_HEX).unwrap();

        assert_eq!(string_parser, test_bytes[..]);
    }

    #[test]
    fn test_peek() {
        let test_bytes: Vec<u8> = hex::decode(TEST_HEX).expect("");
        let binary_parser = BinaryParser::from(test_bytes.as_ref());

        assert_eq!(binary_parser.peek(), Some([test_bytes[0]; 1]));
    }

    #[test]
    fn test_skip_bytes() {
        let test_bytes: Vec<u8> = hex::decode(TEST_HEX).expect("");
        let mut binary_parser = BinaryParser::from(test_bytes.as_ref());

        assert!(binary_parser.skip_bytes(4).is_ok());
        assert_eq!(binary_parser, test_bytes[4..]);
    }

    #[test]
    fn test_read() {
        let test_bytes: Vec<u8> = hex::decode(TEST_HEX).expect("");
        let mut binary_parser = BinaryParser::from(test_bytes.as_ref());
        let result = binary_parser.read(5);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_bytes[..5]);
    }

    #[test]
    fn test_read_uint8() {
        let test_hex: &str = "01000200000003";
        let test_bytes: Vec<u8> = hex::decode(test_hex).expect("");
        let mut binary_parser = BinaryParser::from(test_bytes.as_ref());
        let result = binary_parser.read_uint8();

        assert!(result.is_ok());
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn test_read_uint16() {
        let test_hex: &str = "000200000003";
        let test_bytes: Vec<u8> = hex::decode(test_hex).expect("");
        let mut binary_parser = BinaryParser::from(test_bytes.as_ref());
        let result = binary_parser.read_uint16();

        assert!(result.is_ok());
        assert_eq!(result, Ok(2));
    }

    #[test]
    fn test_read_uint32() {
        let test_hex: &str = "00000003";
        let test_bytes: Vec<u8> = hex::decode(test_hex).expect("");
        let mut binary_parser = BinaryParser::from(test_bytes.as_ref());
        let result = binary_parser.read_uint32();

        assert!(result.is_ok());
        assert_eq!(result, Ok(3));
    }

    #[test]
    fn test_read_length_prefix() {
        let test_bytes: Vec<u8> = hex::decode(TEST_HEX).expect("");
        let mut binary_parser = BinaryParser::from(test_bytes.as_ref());
        let result = binary_parser.read_length_prefix();

        assert!(result.is_ok());
        assert_eq!(result, Ok(0));
    }

    // TODO Finish tests
    #[test]
    fn test_read_field_header() {}

    #[test]
    fn test_read_field_value() {}

    #[test]
    fn test_read_field_and_value() {}

    #[test]
    fn test_read_type() {}

    #[test]
    fn accept_peek_skip_read() {
        let test_bytes: Vec<u8> = hex::decode(TEST_HEX).expect("");
        let mut binary_parser = BinaryParser::from(test_bytes.as_ref());

        assert_eq!(binary_parser.peek(), Some([test_bytes[0]; 1]));
        assert!(binary_parser.skip_bytes(3).is_ok());
        assert_eq!(binary_parser, test_bytes[3..]);

        let result = binary_parser.read(2);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_bytes[3..5]);
    }

    #[test]
    fn test_binaryserializer_write_field_and_value() {
        let field_header = FieldHeader {
            type_code: -2,
            field_code: 0,
        };

        let field_info = FieldInfo {
            nth: 0,
            is_vl_encoded: false,
            is_serialized: false,
            is_signing_field: false,
            r#type: "Unknown".to_string(),
        };

        let field_instance = FieldInstance::new(&field_info, "Generic", field_header);
        let expected: Vec<u8> = [224, 0, 17, 34].to_vec();
        let test_bytes: Vec<u8> = [0, 17, 34].to_vec();
        let mut serializer: BinarySerializer = BinarySerializer::new();

        serializer.write_field_and_value(field_instance, &test_bytes, false);
        assert_eq!(expected, serializer);
    }

    /// This is currently a sanity check for private
    /// [`_encode_variable_length_prefix`], which is called by
    /// BinarySerializer.write_length_encoded.
    #[test]
    fn test_encode_variable_length_prefix() {
        for case in [100_usize, 1000, 20_000] {
            let blob = (0..case).map(|_| "A2").collect::<String>();
            let mut binary_serializer: BinarySerializer = BinarySerializer::new();

            binary_serializer.write_length_encoded(&hex::decode(blob).expect(""), true);

            let mut binary_parser: BinaryParser = BinaryParser::from(binary_serializer.as_ref());
            let decoded_length = binary_parser.read_length_prefix();

            assert!(decoded_length.is_ok());
            assert_eq!(decoded_length, Ok(case));
        }
    }
}
