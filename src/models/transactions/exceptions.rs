use crate::models::transactions::{AccountSetFlag, PaymentFlag};
use strum_macros::Display;
use thiserror_no_std::Error;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum XrplTransactionException<'a> {
    XrplAccountSetError(XrplAccountSetException<'a>),
    XrplCheckCashError(XrplCheckCashException<'a>),
    XrplDepositPreauthError(XrplDepositPreauthException<'a>),
    XrplEscrowCreateError(XrplEscrowCreateException<'a>),
    XrplEscrowFinishError(XrplEscrowFinishException<'a>),
    XrplNFTokenAcceptOfferError(XrplNFTokenAcceptOfferException<'a>),
    XrplNFTokenCancelOfferError(XrplNFTokenCancelOfferException<'a>),
    XrplNFTokenCreateOfferError(XrplNFTokenCreateOfferException<'a>),
    XrplNFTokenMintError(XrplNFTokenMintException<'a>),
    XrplPaymentError(XrplPaymentException<'a>),
    XrplSignerListSetError(XrplSignerListSetException<'a>),
    XrplUNLModifyError(XrplUNLModifyException<'a>),
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplTransactionException<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplAccountSetException<'a> {
    /// A fields value exceeds its maximum value.
    #[error("The value of the field `{field:?}` is defined above its maximum (max {max:?}, found {found:?}). For more information see: {resource:?}")]
    ValueTooHigh {
        field: &'a str,
        max: u32,
        found: u32,
        resource: &'a str,
    },
    /// A fields value exceeds its minimum value.
    #[error("The value of the field `{field:?}` is defined below its minimum (min {min:?}, found {found:?}). For more information see: {resource:?}")]
    ValueTooLow {
        field: &'a str,
        min: u32,
        found: u32,
        resource: &'a str,
    },
    /// A fields value exceeds its maximum character length.
    #[error("The value of the field `{field:?}` exceeds its maximum length of characters (max {max:?}, found {found:?}). For more information see: {resource:?}")]
    ValueTooLong {
        field: &'a str,
        max: usize,
        found: usize,
        resource: &'a str,
    },
    /// A fields value doesn't match its required format.
    #[error("The value of the field `{field:?}` does not have the correct format (expected {format:?}, found {found:?}). For more information see: {resource:?}")]
    InvalidValueFormat {
        field: &'a str,
        format: &'a str,
        found: &'a str,
        resource: &'a str,
    },
    /// A field can only be defined if a transaction flag is set.
    #[error("For the field `{field:?}` to be defined it is required to set the flag `{flag:?}`. For more information see: {resource:?}")]
    FieldRequiresFlag {
        field: &'a str,
        flag: AccountSetFlag,
        resource: &'a str,
    },
    /// An account set flag can only be set if a field is defined.
    #[error("For the flag `{flag:?}` to be set it is required to define the field `{field:?}`. For more information see: {resource:?}")]
    FlagRequiresField {
        flag: AccountSetFlag,
        field: &'a str,
        resource: &'a str,
    },
    /// Am account set flag can not be set and unset at the same time.
    #[error("A flag cannot be set and unset at the same time (found {found:?}). For more information see: {resource:?}")]
    SetAndUnsetSameFlag {
        found: AccountSetFlag,
        resource: &'a str,
    },
    /// A field was defined and an account set flag that is required for that field was unset.
    #[error("The field `{field:?}` cannot be defined if its required flag `{flag:?}` is being unset. For more information see: {resource:?}")]
    SetFieldWhenUnsetRequiredFlag {
        field: &'a str,
        flag: AccountSetFlag,
        resource: &'a str,
    },
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplAccountSetException<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplCheckCashException<'a> {
    /// A field cannot be defined with other fields.
    #[error("The field `{field1:?}` can not be defined with `{field2:?}`. Define exactly one of them. For more information see: {resource:?}")]
    DefineExactlyOneOf {
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplDepositPreauthException<'a> {
    /// A field cannot be defined with other fields.
    #[error("The field `{field1:?}` can not be defined with `{field2:?}`. Define exactly one of them. For more information see: {resource:?}")]
    DefineExactlyOneOf {
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplCheckCashException<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplEscrowCreateException<'a> {
    /// A fields value cannot be below another fields value.
    #[error("The value of the field `{field1:?}` is not allowed to be below the value of the field `{field2:?}` (max {field2_val:?}, found {field1_val:?}). For more information see: {resource:?}")]
    ValueBelowValue {
        field1: &'a str,
        field2: &'a str,
        field1_val: u32,
        field2_val: u32,
        resource: &'a str,
    },
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplEscrowCreateException<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplEscrowFinishException<'a> {
    /// For a field to be defined it also needs another field to be defined.
    #[error("For the field `{field1:?}` to be defined it is required to also define the field `{field2:?}`. For more information see: {resource:?}")]
    FieldRequiresField {
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplEscrowFinishException<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplNFTokenAcceptOfferException<'a> {
    /// Define at least one of the fields.
    #[error("Define at least one of the fields `{field1:?}` and `{field2:?}`. For more information see: {resource:?}")]
    DefineOneOf {
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    /// The value can not be zero.
    #[error("The value of the field `{field:?}` is not allowed to be zero. For more information see: {resource:?}")]
    ValueZero { field: &'a str, resource: &'a str },
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplNFTokenAcceptOfferException<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplNFTokenCancelOfferException<'a> {
    /// A collection was defined to be empty.
    #[error("The value of the field `{field:?}` is not allowed to be empty (type `{r#type:?}`). If the field is optional, define it to be `None`. For more information see: {resource:?}")]
    CollectionEmpty {
        field: &'a str,
        r#type: &'a str,
        resource: &'a str,
    },
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplNFTokenCancelOfferException<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplNFTokenCreateOfferException<'a> {
    /// The value can not be zero.
    #[error("The value of the field `{field:?}` is not allowed to be zero. For more information see: {resource:?}")]
    ValueZero { field: &'a str, resource: &'a str },
    /// A fields value is not allowed to be the same as another fields value.
    #[error("The value of the field `{field1:?}` is not allowed to be the same as the value of the field `{field2:?}`. For more information see: {resource:?}")]
    ValueEqualsValue {
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    /// An optional value must be defined in a certain context.
    #[error("The optional field `{field:?}` is required to be defined for {context:?}. For more information see: {resource:?}")]
    OptionRequired {
        field: &'a str,
        context: &'a str,
        resource: &'a str,
    },
    /// An optional value is not allowed to be defined in a certain context.
    #[error("The optional field `{field:?}` is not allowed to be defined for {context:?}. For more information see: {resource:?}")]
    IllegalOption {
        field: &'a str,
        context: &'a str,
        resource: &'a str,
    },
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplNFTokenCreateOfferException<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplNFTokenMintException<'a> {
    /// A fields value is not allowed to be the same as another fields value.
    #[error("The value of the field `{field1:?}` is not allowed to be the same as the value of the field `{field2:?}`. For more information see: {resource:?}")]
    ValueEqualsValue {
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    /// A fields value exceeds its maximum value.
    #[error("The field `{field:?}` exceeds its maximum value (max {max:?}, found {found:?}). For more information see: {resource:?}")]
    ValueTooHigh {
        field: &'a str,
        max: u32,
        found: u32,
        resource: &'a str,
    },
    /// A fields value exceeds its maximum character length.
    #[error("The value of the field `{field:?}` exceeds its maximum length of characters (max {max:?}, found {found:?}). For more information see: {resource:?}")]
    ValueTooLong {
        field: &'a str,
        max: usize,
        found: usize,
        resource: &'a str,
    },
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplNFTokenMintException<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplPaymentException<'a> {
    /// An optional value must be defined in a certain context.
    #[error("The optional field `{field:?}` is required to be defined for {context:?}. For more information see: {resource:?}")]
    OptionRequired {
        field: &'a str,
        context: &'a str,
        resource: &'a str,
    },
    /// An optional value is not allowed to be defined in a certain context.
    #[error("The optional field `{field:?}` is not allowed to be defined for {context:?}.For more information see: {resource:?}")]
    IllegalOption {
        field: &'a str,
        context: &'a str,
        resource: &'a str,
    },
    /// A fields value is not allowed to be the same as another fields value, in a certain context.
    #[error("The value of the field `{field1:?}` is not allowed to be the same as the value of the field `{field2:?}`, for {context:?}. For more information see: {resource:?}")]
    ValueEqualsValueInContext {
        field1: &'a str,
        field2: &'a str,
        context: &'a str,
        resource: &'a str,
    },
    /// An account set flag can only be set if a field is defined.
    #[error("For the flag `{flag:?}` to be set it is required to define the field `{field:?}`. For more information see: {resource:?}")]
    FlagRequiresField {
        flag: PaymentFlag,
        field: &'a str,
        resource: &'a str,
    },
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplPaymentException<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplSignerListSetException<'a> {
    /// A field was defined that another field definition would delete.
    #[error("The value of the field `{field1:?}` can not be defined with the field `{field2:?}` because it would cause the deletion of `{field1:?}`. For more information see: {resource:?}")]
    ValueCausesValueDeletion {
        field1: &'a str,
        field2: &'a str,
        resource: &'a str,
    },
    /// A field is expected to have a certain value to be deleted.
    #[error("The field `{field:?}` has the wrong value to be deleted (expected {expected:?}, found {found:?}). For more information see: {resource:?}")]
    InvalidValueForValueDeletion {
        field: &'a str,
        expected: u32,
        found: u32,
        resource: &'a str,
    },
    /// A collection has too few items in it.
    #[error("The value of the field `{field:?}` has too few items in it (min {min:?}, found {found:?}). For more information see: {resource:?}")]
    CollectionTooFewItems {
        field: &'a str,
        min: usize,
        found: usize,
        resource: &'a str,
    },
    /// A collection has too many items in it.
    #[error("The value of the field `{field:?}` has too many items in it (max {max:?}, found {found:?}). For more information see: {resource:?}")]
    CollectionTooManyItems {
        field: &'a str,
        max: usize,
        found: usize,
        resource: &'a str,
    },
    /// A collection is not allowed to have duplicates in it.
    #[error("The value of the field `{field:?}` has a duplicate in it (found {found:?}). For more information see: {resource:?}")]
    CollectionItemDuplicate {
        field: &'a str,
        found: &'a str,
        resource: &'a str,
    },
    /// A collection contains an invalid value.
    #[error("The field `{field:?}` contains an invalid value (found {found:?}). For more information see: {resource:?}")]
    CollectionInvalidItem {
        field: &'a str,
        found: &'a str,
        resource: &'a str,
    },
    #[error("The field `signer_quorum` must be below or equal to the sum of `signer_weight` in `signer_entries`. For more information see: {resource:?}")]
    SignerQuorumExceedsSignerWeight {
        max: u32,
        found: u32,
        resource: &'a str,
    },
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplSignerListSetException<'a> {}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum XrplUNLModifyException<'a> {
    /// A field is expected to have a certain value.
    #[error("The field `{field:?}` has an invalid value (expected {expected:?}, found {found:?}). For more information see: {resource:?}")]
    InvalidValue {
        field: &'a str,
        expected: &'a str,
        found: u32,
        resource: &'a str,
    },
}

#[cfg(feature = "std")]
impl<'a> alloc::error::Error for XrplUNLModifyException<'a> {}