//! Codec for serializing and deserializing Amount fields.
//!
//! See Amount Fields:
//! `<https://xrpl.org/serialization.html#amount-fields>`

use crate::core::binarycodec::exceptions::XRPLBinaryCodecException;
use crate::core::binarycodec::BinaryParser;
use crate::core::binarycodec::Parser;
use crate::core::types::account_id::AccountId;
use crate::core::types::currency::Currency;
use crate::core::types::*;
use crate::utils::exceptions::XRPRangeException;
use crate::utils::xrpl_conversion::*;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;
use core::convert::TryInto;
use core::str::FromStr;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use serde::ser::Error;
use serde::ser::SerializeMap;
use serde::Serializer;
use serde::{Deserialize, Serialize};

const _MIN_MANTISSA: u64 = u64::pow(10, 15);
const _MAX_MANTISSA: u64 = u64::pow(10, 16) - 1;

const _NOT_XRP_BIT_MASK: u8 = 0x80;
const _POS_SIGN_BIT_MASK: u64 = 0x4000000000000000;
const _ZERO_CURRENCY_AMOUNT_HEX: u64 = 0x8000000000000000;
const _NATIVE_AMOUNT_BYTE_LENGTH: u8 = 8;
const _CURRENCY_AMOUNT_BYTE_LENGTH: u8 = 48;

/// An Issued Currency object.
struct IssuedCurrency {
    pub value: Decimal,
    pub currency: Currency,
    pub issuer: AccountId,
}

/// Codec for serializing and deserializing Amount fields.
///
/// See Amount Fields:
/// `<https://xrpl.org/serialization.html#amount-fields>`
#[derive(Debug, Deserialize, Clone)]
#[serde(try_from = "&str")]
pub struct Amount(Vec<u8>);

/// Returns True if the given string contains a
/// decimal point character.
fn _contains_decimal(string: &str) -> bool {
    string.contains('.')
}

/// Serializes the value field of an issued currency amount
/// to its bytes representation.
fn _serialize_issued_currency_value(decimal: Decimal) -> Result<[u8; 16], XRPRangeException> {
    verify_valid_ic_value(&decimal.to_string())?;

    let mut mantissa = decimal.mantissa();
    let mut exp: u64 = decimal.scale() as u64;

    if decimal.is_zero() {
        return Ok((_ZERO_CURRENCY_AMOUNT_HEX as i128).to_be_bytes());
    };

    while mantissa < _MIN_MANTISSA as i128 && exp as i32 > MIN_IOU_EXPONENT as i32 {
        mantissa *= 10;
        exp -= 1;
    }

    while mantissa > _MAX_MANTISSA as i128 {
        if exp >= MAX_IOU_EXPONENT as u64 {
            return Err(XRPRangeException::UnexpectedICAmountOverflow {
                max: MAX_IOU_EXPONENT as usize,
                found: exp as usize,
            });
        } else {
            mantissa /= 10;
            exp += 1;
        }
    }

    if (exp as i32) < MIN_IOU_EXPONENT as i32 || mantissa < _MIN_MANTISSA as i128 {
        // Round to zero
        Ok((_ZERO_CURRENCY_AMOUNT_HEX as i128).to_be_bytes())
    } else if exp > MAX_IOU_EXPONENT as u64 || mantissa > _MAX_MANTISSA as i128 {
        Err(XRPRangeException::UnexpectedICAmountOverflow {
            max: MAX_IOU_EXPONENT as usize,
            found: exp as usize,
        })
    } else {
        // "Not XRP" bit set
        let mut serial: i128 = _ZERO_CURRENCY_AMOUNT_HEX as i128;

        // "Is positive" bit set
        if decimal.is_sign_positive() {
            serial |= _POS_SIGN_BIT_MASK as i128;
        };

        // next 8 bits are exponents
        serial |= ((exp + 97) << 54) as i128;
        // last 54 bits are mantissa
        serial |= mantissa;

        Ok(serial.to_be_bytes())
    }
}

/// Serializes an XRP amount.
fn _serialize_xrp_amount(value: &str) -> Result<[u8; 8], XRPRangeException> {
    verify_valid_xrp_value(value)?;

    let decimal = Decimal::from_str(value)?.normalize();

    if let Some(result) = decimal.to_u64() {
        let value_with_pos_bit = result | _POS_SIGN_BIT_MASK;
        Ok(value_with_pos_bit.to_be_bytes())
    } else {
        // Safety, should never occur
        Err(XRPRangeException::InvalidXRPAmount)
    }
}

/// Serializes an issued currency amount.
fn _serialize_issued_currency_amount(
    issused_currency: IssuedCurrency,
) -> Result<[u8; 8], XRPRangeException> {
    let mut bytes = vec![];
    let amount_bytes = _serialize_issued_currency_value(issused_currency.value)?;
    let currency_bytes: &[u8] = issused_currency.currency.as_ref();
    let issuer_bytes: &[u8] = issused_currency.issuer.as_ref();

    bytes.extend_from_slice(&amount_bytes);
    bytes.extend_from_slice(currency_bytes);
    bytes.extend_from_slice(issuer_bytes);

    Ok(bytes.try_into().expect("_serialize_issued_currency_amount"))
}

impl Amount {
    /// Format native asset value for serialization.
    fn _format_native_serialization(&self) -> String {
        let sign: &str = if self.is_positive() { "" } else { "-" };
        let mut sized: [u8; 8] = Default::default();
        sized.copy_from_slice(&self.get_buffer()[..8]);
        let number = u64::from_be_bytes(sized);

        format!("{}{}", sign, number)
    }

    /// Returns True if this amount is a native XRP amount.
    pub fn is_native(&self) -> bool {
        self.0[0] == 0
    }

    /// Returns true if 2nd bit in 1st byte is set to 1
    /// (positive amount).
    pub fn is_positive(&self) -> bool {
        self.0[1] > 0
    }
}

impl IssuedCurrency {
    /// Format issued currency value for serialization.
    fn _format_ic_serialization(
        parser: &mut BinaryParser,
    ) -> Result<Decimal, XRPLBinaryCodecException> {
        let ic = IssuedCurrency::from_parser(parser, None)?;
        let exp = ic.value.scale();
        let mantissa = ic.value.mantissa();
        let decimal = Decimal::from_str(&format!("{}", mantissa))?;
        let multiplier = Decimal::from_str(&format!("1e{}", exp))?;
        let value = decimal
            .checked_mul(multiplier)
            .ok_or(rust_decimal::Error::ExceedsMaximumPossibleValue)?
            .normalize();

        verify_valid_ic_value(&value.to_string())?;
        Ok(value)
    }
}

impl XRPLType for Amount {
    type Error = hex::FromHexError;

    fn new(buffer: Option<&[u8]>) -> Result<Self, Self::Error> {
        Ok(Amount(buffer.or_else(|| Some(&[])).unwrap().to_vec()))
    }
}

impl Buffered for Amount {
    fn get_buffer(&self) -> &[u8] {
        &self.0
    }
}

impl FromParser for Amount {
    type Error = XRPLBinaryCodecException;

    fn from_parser(
        parser: &mut BinaryParser,
        _length: Option<usize>,
    ) -> Result<Amount, Self::Error> {
        let parser_first_byte = parser.peek();
        let num_bytes = match parser_first_byte {
            None => _CURRENCY_AMOUNT_BYTE_LENGTH,
            Some(_) => _NATIVE_AMOUNT_BYTE_LENGTH,
        };

        Ok(Amount(parser.read(num_bytes as usize)?))
    }
}

impl FromParser for IssuedCurrency {
    type Error = XRPLBinaryCodecException;

    fn from_parser(
        parser: &mut BinaryParser,
        _length: Option<usize>,
    ) -> Result<IssuedCurrency, Self::Error> {
        Ok(IssuedCurrency {
            value: IssuedCurrency::_format_ic_serialization(parser)?,
            currency: Currency::from_parser(parser, None)?,
            issuer: AccountId::from_parser(parser, None)?,
        })
    }
}

impl Serialize for Amount {
    /// Construct a JSON object representing this Amount.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.is_native() {
            serializer.serialize_str(&self._format_native_serialization())
        } else {
            let mut parser = BinaryParser::from(self.get_buffer());

            if let Ok(ic) = IssuedCurrency::from_parser(&mut parser, None) {
                let mut builder = serializer.serialize_map(Some(3))?;

                builder.serialize_entry("value", &ic.value)?;
                builder.serialize_entry("currency", &ic.currency)?;
                builder.serialize_entry("issuer", &ic.issuer)?;
                builder.end()
            } else {
                Err(S::Error::custom(
                    XRPLBinaryCodecException::InvalidReadFromBytesValue,
                ))
            }
        }
    }
}

impl TryFrom<&str> for Amount {
    type Error = XRPRangeException;

    /// Construct a Hash object from a hex string.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let serialized = _serialize_xrp_amount(value)?;
        Ok(Amount::new(Some(&serialized))?)
    }
}

impl TryFrom<IssuedCurrency> for Amount {
    type Error = XRPRangeException;

    /// Construct a Hash object from a hex string.
    fn try_from(value: IssuedCurrency) -> Result<Self, Self::Error> {
        let serialized = _serialize_issued_currency_amount(value)?;
        Ok(Amount::new(Some(&serialized))?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains_decimal() {}
}
