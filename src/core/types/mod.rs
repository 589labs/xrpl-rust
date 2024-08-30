//! Top-level exports for types used in binary_codec.

pub mod account_id;
pub mod amount;
pub mod blob;
pub mod currency;
pub mod exceptions;
pub mod hash;
pub mod paths;
pub(crate) mod test_cases;
pub mod utils;
pub mod vector256;

use core::convert::TryFrom;
use core::convert::TryInto;
use core::fmt::Debug;
use core::fmt::Display;
use core::iter::FromIterator;

pub use self::account_id::AccountId;
pub use self::amount::Amount;
pub use self::blob::Blob;
pub use self::currency::Currency;
pub use self::hash::Hash;
pub use self::hash::Hash128;
pub use self::hash::Hash160;
pub use self::hash::Hash256;
pub use self::paths::Path;
pub use self::paths::PathSet;
pub use self::paths::PathStep;
pub use self::vector256::Vector256;

use crate::core::binarycodec::binary_wrappers::Serialization;
use crate::core::definitions::get_field_instance;
use crate::core::definitions::get_transaction_result_code;
use crate::core::definitions::get_transaction_type_code;
use crate::core::definitions::FieldInstance;
use crate::core::BinaryParser;
use crate::Err;
use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use amount::IssuedCurrency;
use anyhow::Result;
use serde_json::Map;
use serde_json::Value;

use super::addresscodec::is_valid_xaddress;
use super::addresscodec::xaddress_to_classic_address;
use super::BinarySerializer;

const ACCOUNT: &str = "Account";
const SOURCE_TAG: &str = "SourceTag";
const DESTINATION: &str = "Destination";
const DESTINATION_TAG: &str = "DestinationTag";
const UNL_MODIFY_TX_TYPE: &str = "0066";
const ST_OBJECT: &str = "STObject";
const OBJECT_END_MARKER_BYTES: [u8; 1] = [0xE1];

#[derive(Debug)]
pub enum XRPLTypes {
    AccountID(AccountId),
    Amount(Amount),
    Blob(Blob),
    Currency(Currency),
    Hash128(Hash128),
    Hash160(Hash160),
    Hash256(Hash256),
    Path(Path),
    PathSet(PathSet),
    PathStep(PathStep),
    Vector256(Vector256),
    STArray(STArray),
    STObject(STObject),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Unknown,
}

impl XRPLTypes {
    pub fn from_value(name: &str, value: Value) -> Result<XRPLTypes> {
        if let Some(value) = value.as_str() {
            match name {
                "AccountID" => Ok(XRPLTypes::AccountID(Self::type_from_str(value)?)),
                "Amount" => Ok(XRPLTypes::Amount(Self::type_from_str(value)?)),
                "Blob" => Ok(XRPLTypes::Blob(Self::type_from_str(value)?)),
                "Currency" => Ok(XRPLTypes::Currency(Self::type_from_str(value)?)),
                "Hash128" => Ok(XRPLTypes::Hash128(Self::type_from_str(value)?)),
                "Hash160" => Ok(XRPLTypes::Hash160(Self::type_from_str(value)?)),
                "Hash256" => Ok(XRPLTypes::Hash256(Self::type_from_str(value)?)),
                _ => Err!(exceptions::XRPLTypeException::UnknownXRPLType),
            }
        } else if let Some(value) = value.as_u64() {
            match name {
                "UInt8" => Ok(XRPLTypes::UInt8(value as u8)),
                "UInt16" => Ok(XRPLTypes::UInt16(value as u16)),
                "UInt32" => Ok(XRPLTypes::UInt32(value as u32)),
                "UInt64" => Ok(XRPLTypes::UInt64(value as u64)),
                _ => Err!(exceptions::XRPLTypeException::UnknownXRPLType),
            }
        } else if let Some(value) = value.as_object() {
            match name {
                "Amount" => Ok(XRPLTypes::Amount(Self::amount_from_map(value.to_owned())?)),
                "STObject" => Ok(XRPLTypes::STObject(STObject::try_from_value(
                    Value::Object(value.to_owned()),
                    false,
                )?)),
                _ => Err!(exceptions::XRPLTypeException::UnknownXRPLType),
            }
        } else if let Some(value) = value.as_array() {
            todo!()
            // match name {
            // "STArray" => XRPLTypes::STArray(STArray(SerializedType::from(
            //     value
            //         .iter()
            //         .map(|v| v.to_string())
            //         .collect::<Vec<String>>()
            //         .join(""),
            // ))),
            // _ => XRPLTypes::Unknown,
            // }
        } else {
            Err!(exceptions::XRPLTypeException::UnknownXRPLType)
        }
    }

    fn type_from_str<'a, T>(value: &'a str) -> Result<T>
    where
        T: TryFrom<&'a str>,
        <T as TryFrom<&'a str>>::Error: Display,
    {
        match value.try_into() {
            Ok(value) => Ok(value),
            Err(error) => Err!(error),
        }
    }

    fn amount_from_map<T>(value: Map<String, Value>) -> Result<T>
    where
        T: TryFrom<IssuedCurrency>,
        <T as TryFrom<IssuedCurrency>>::Error: Display,
    {
        match IssuedCurrency::try_from(Value::Object(value)) {
            Ok(value) => match value.try_into() {
                Ok(value) => Ok(value),
                Err(error) => Err!(error),
            },
            Err(error) => Err!(error),
        }
    }
}

impl Into<SerializedType> for XRPLTypes {
    fn into(self) -> SerializedType {
        match self {
            XRPLTypes::AccountID(account_id) => SerializedType::from(account_id),
            XRPLTypes::Amount(amount) => SerializedType::from(amount),
            XRPLTypes::Blob(blob) => SerializedType::from(blob),
            XRPLTypes::Currency(currency) => SerializedType::from(currency),
            XRPLTypes::Hash128(hash128) => SerializedType::from(hash128),
            XRPLTypes::Hash160(hash160) => SerializedType::from(hash160),
            XRPLTypes::Hash256(hash256) => SerializedType::from(hash256),
            XRPLTypes::Path(path) => SerializedType::from(path),
            XRPLTypes::PathSet(path_set) => SerializedType::from(path_set),
            XRPLTypes::PathStep(path_step) => SerializedType::from(path_step),
            XRPLTypes::Vector256(vector256) => SerializedType::from(vector256),
            XRPLTypes::STArray(st_array) => st_array.0,
            XRPLTypes::STObject(st_object) => st_object.0,
            XRPLTypes::UInt8(value) => SerializedType(value.to_be_bytes().to_vec()),
            XRPLTypes::UInt16(value) => SerializedType(value.to_be_bytes().to_vec()),
            XRPLTypes::UInt32(value) => SerializedType(value.to_be_bytes().to_vec()),
            XRPLTypes::UInt64(value) => SerializedType(value.to_be_bytes().to_vec()),
            XRPLTypes::Unknown => SerializedType(vec![]),
        }
    }
}

/// Contains a serialized buffer of a Serializer type.
#[derive(Debug, Clone)]
pub struct SerializedType(Vec<u8>);

/// Class for serializing and deserializing Lists of objects.
///
/// See Array Fields:
/// `<https://xrpl.org/serialization.html#array-fields>`
#[derive(Debug)]
pub struct STArray(SerializedType);

/// Class for serializing/deserializing Indexmaps of objects.
///
/// See Object Fields:
/// `<https://xrpl.org/serialization.html#object-fields>`
#[derive(Debug)]
pub struct STObject(SerializedType);

impl STObject {
    /// Create a SerializedMap from a serde_json::Value.
    ///
    /// ```
    /// use xrpl::core::types::STObject;
    ///
    /// let expected_json = r#"{
    ///     "Account": "raD5qJMAShLeHZXf9wjUmo6vRK4arj9cF3",
    ///     "Fee": "10",
    ///     "Flags": 0,
    ///     "Sequence": 103929,
    ///     "SigningPubKey": "028472865AF4CB32AA285834B57576B7290AA8C31B459047DB27E16F418D6A7166",
    ///     "TakerGets": {
    ///         "value": "1694.768",
    ///         "currency": "ILS",
    ///         "issuer": "rNPRNzBB92BVpAhhZr4iXDTveCgV5Pofm9"
    ///     },
    ///     "TakerPays": "98957503520",
    ///     "TransactionType": "OfferCreate",
    ///     "TxnSignature": "304502202ABE08D5E78D1E74A4C18F2714F64E87B8BD57444AFA5733109EB3C077077520022100DB335EE97386E4C0591CAC024D50E9230D8F171EEB901B5E5E4BD6D1E0AEF98C"
    /// }"#;
    ///
    /// let buffer = "120007220000000024000195F964400000170A53AC2065D5460561E\
    ///     C9DE000000000000000000000000000494C53000000000092D70596\
    ///     8936C419CE614BF264B5EEB1CEA47FF468400000000000000A73210\
    ///     28472865AF4CB32AA285834B57576B7290AA8C31B459047DB27E16F\
    ///     418D6A71667447304502202ABE08D5E78D1E74A4C18F2714F64E87B\
    ///     8BD57444AFA5733109EB3C077077520022100DB335EE97386E4C059\
    ///     1CAC024D50E9230D8F171EEB901B5E5E4BD6D1E0AEF98C811439408\
    ///     A69F0895E62149CFCC006FB89FA7D1E6E5D";
    /// let value = serde_json::from_str(expected_json).unwrap();
    /// let serialized_map = STObject::try_from_value(value, false).unwrap();
    /// let hex = hex::encode_upper(serialized_map.as_ref());
    /// assert_eq!(hex, buffer);
    /// ```
    pub fn try_from_value(value: Value, signing_only: bool) -> Result<Self> {
        let object = match value {
            Value::Object(map) => map,
            _ => return Err!(exceptions::XRPLSerializeMapException::ExpectedObject),
        };
        let mut serializer = BinarySerializer::new();
        let mut value_xaddress_handled = Map::new();
        for (field, value) in &object {
            if let Some(value) = value.as_str() {
                if is_valid_xaddress(value) {
                    let handled_xaddress = handle_xaddress(field.into(), value.into())?;
                    if let Some(handled_tag) = handled_xaddress.get(SOURCE_TAG) {
                        if let Some(object_tag) = object.get(SOURCE_TAG) {
                            if handled_tag != object_tag {
                                return Err!(
                                    exceptions::XRPLSerializeMapException::AccountMismatchingTags
                                );
                            }
                        }
                    }
                    if let Some(handled_tag) = handled_xaddress.get(DESTINATION_TAG) {
                        if let Some(object_tag) = object.get(DESTINATION_TAG) {
                            if handled_tag != object_tag {
                                return Err!(
                                    exceptions::XRPLSerializeMapException::DestinationMismatchingTags
                                );
                            }
                        }
                    }
                    value_xaddress_handled.extend(handled_xaddress);
                } else if field == "TransactionType" {
                    let transaction_type_code = match get_transaction_type_code(value) {
                        Some(code) => code,
                        None => {
                            return Err!(
                                exceptions::XRPLSerializeMapException::UnknownTransactionType(
                                    value
                                )
                            )
                        }
                    };
                    value_xaddress_handled.insert(
                        field.to_owned(),
                        Value::Number(transaction_type_code.to_owned().into()),
                    );
                } else if field == "TransactionResult" {
                    let transaction_result_code = match get_transaction_result_code(value) {
                        Some(code) => code,
                        None => {
                            return Err!(
                                exceptions::XRPLSerializeMapException::UnknownTransactionResult(
                                    value
                                )
                            )
                        }
                    };
                    value_xaddress_handled.insert(
                        field.to_owned(),
                        Value::Number(transaction_result_code.to_owned().into()),
                    );
                } else if field == "LedgerEntryType" {
                    let ledger_entry_type_code = match get_transaction_type_code(value) {
                        Some(code) => code,
                        None => {
                            return Err!(
                                exceptions::XRPLSerializeMapException::UnknownLedgerEntryType(
                                    value
                                )
                            )
                        }
                    };
                    value_xaddress_handled.insert(
                        field.to_owned(),
                        Value::Number(ledger_entry_type_code.to_owned().into()),
                    );
                } else {
                    value_xaddress_handled
                        .insert(field.to_owned(), Value::String(value.to_owned()));
                }
            } else {
                value_xaddress_handled.insert(field.to_owned(), value.clone());
            }
        }

        let mut sorted_keys: Vec<FieldInstance> = Vec::new();
        for (field, _) in &value_xaddress_handled {
            let field_instance = get_field_instance(&field);
            if let Some(field_instance) = field_instance {
                if value_xaddress_handled.contains_key(&field_instance.name)
                    && field_instance.is_serialized
                {
                    sorted_keys.push(field_instance);
                }
            }
        }
        sorted_keys.sort_by_key(|k| k.ordinal);
        if signing_only {
            sorted_keys.retain(|k| k.is_signing);
        }
        let mut is_unl_modify = false;

        for field_instance in &sorted_keys {
            let associated_value = match value_xaddress_handled.get(&field_instance.name) {
                Some(value) => value,
                None => Err(anyhow::anyhow!(
                    "Error prossessing field: {}",
                    field_instance.name
                ))?,
            };
            let associated_value = XRPLTypes::from_value(
                &field_instance.associated_type,
                associated_value.to_owned(),
            )?;
            let associated_value: SerializedType = associated_value.into();
            // dbg!(&field_instance, &associated_value);
            if field_instance.name == "TransactionType"
                && associated_value.to_string() == UNL_MODIFY_TX_TYPE
            {
                is_unl_modify = true;
            }
            let _is_unl_modify_workaround = field_instance.name == "Account" && is_unl_modify;

            serializer.write_field_and_value(field_instance.to_owned(), associated_value.as_ref());
            if field_instance.associated_type == ST_OBJECT {
                serializer.append(OBJECT_END_MARKER_BYTES.to_vec().as_mut());
            }
        }

        Ok(STObject(serializer.into()))
    }
}

impl XRPLType for STObject {
    type Error = anyhow::Error;

    fn new(buffer: Option<&[u8]>) -> Result<Self, Self::Error> {
        if let Some(data) = buffer {
            Ok(STObject(SerializedType(data.to_vec())))
        } else {
            Ok(STObject(SerializedType(vec![])))
        }
    }
}

impl AsRef<[u8]> for STObject {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

fn handle_xaddress(field: Cow<str>, xaddress: Cow<str>) -> Result<Map<String, Value>> {
    let (classic_address, tag, _is_test_net) = match xaddress_to_classic_address(&xaddress) {
        Ok((classic_address, tag, is_test_net)) => (classic_address, tag, is_test_net),
        Err(e) => return Err!(e),
    };
    if let Some(tag) = tag {
        if &field == DESTINATION {
            let tag_name = DESTINATION_TAG;
            Ok(Map::from_iter(vec![
                (field.to_string(), Value::String(classic_address)),
                (tag_name.to_string(), Value::Number(tag.into())),
            ]))
        } else if &field == ACCOUNT {
            let tag_name = SOURCE_TAG;
            Ok(Map::from_iter(vec![
                (field.to_string(), Value::String(classic_address)),
                (tag_name.to_string(), Value::Number(tag.into())),
            ]))
        } else {
            Err!(exceptions::XRPLSerializeMapException::DisallowedTag { field: &field })
        }
    } else {
        Ok(Map::from_iter(vec![(
            field.to_string(),
            Value::String(classic_address),
        )]))
    }
}

/// An XRPL Type will implement this trait.
///
/// # Examples
///
/// ## Basic usage
///
/// ```
/// use xrpl::core::types::XRPLType;
/// use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
///
/// pub struct Example(Vec<u8>);
///
/// impl XRPLType for Example {
///     type Error = XRPLBinaryCodecException;
///
///     fn new(buffer: Option<&[u8]>) -> Result<Self, Self::Error> {
///         if let Some(data) = buffer {
///             Ok(Example(data.to_vec()))
///         } else {
///             Ok(Example(vec![]))
///         }
///     }
/// }
/// ```
pub trait XRPLType {
    /// Error type for implementing type.
    type Error;

    /// Create a new instance of a type.
    fn new(buffer: Option<&[u8]>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// Converter for transforming a BinaryParser into a type.
///
/// # Examples
///
/// ## Basic usage
///
/// ```
/// use xrpl::core::types::TryFromParser;
/// use xrpl::core::binarycodec::BinaryParser;
/// use xrpl::core::Parser;
/// use xrpl::core::binarycodec::exceptions::XRPLBinaryCodecException;
///
/// pub struct Example(Vec<u8>);
///
/// impl TryFromParser for Example {
///     type Error = XRPLBinaryCodecException;
///
///     fn from_parser(
///         parser: &mut BinaryParser,
///         _length: Option<usize>,
///     ) -> Result<Example, Self::Error> {
///         Ok(Example(parser.read(42)?))
///     }
/// }
/// ```
pub trait TryFromParser {
    /// Error type for implementing type.
    type Error;

    /// Construct a type from a BinaryParser.
    fn from_parser(parser: &mut BinaryParser, length: Option<usize>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

impl ToString for SerializedType {
    /// Get the hex representation of the SerializedType bytes.
    fn to_string(&self) -> String {
        hex::encode_upper(self.0.as_slice())
    }
}

impl From<Vec<u8>> for SerializedType {
    /// Create a SerializedType from a Vec<u8>.
    fn from(buffer: Vec<u8>) -> Self {
        SerializedType(buffer)
    }
}

impl AsRef<[u8]> for SerializedType {
    /// Get a reference of the byte representation.
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl<T> From<T> for SerializedType
where
    T: XRPLType + AsRef<[u8]>,
{
    /// Create a serialized type from an XRPLType.
    fn from(instance: T) -> Self {
        SerializedType(instance.as_ref().to_vec())
    }
}
