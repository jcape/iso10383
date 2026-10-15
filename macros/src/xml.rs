//! Code Generation based on the XML parser.

mod recordset;

use crate::xml::recordset::RecordSet;
use core::fmt::Debug;
use iso10383_parser::MicList;
use proc_macro2::{Span, TokenStream};
use quick_xml::de;
use std::{env, fs::File, io::BufReader, path::PathBuf};
use syn::{
    Error, Expr, ExprLit, Lit, Meta, MetaNameValue, Result, Token, parse::Parser as _,
    punctuated::Punctuated,
};

/// The macro configuration for the xml parser.
#[derive(Debug)]
struct Config {
    /// The path to the xml file.
    xml: PathBuf,
    /// The code location of the macro invocation.
    span: Span,
    /// The zerocopy feature name.
    zerocopy: Option<String>,
    /// The details (static data) feature name.
    details: Option<String>,
}

impl Config {
    /// Parse the literal provided as a path to an XML file.
    fn parse_xml(&mut self, lit: &Lit) -> Result<()> {
        if !self.xml.as_os_str().is_empty() {
            return Err(Error::new_spanned(lit, "Multiple `xml` parameters."));
        }

        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .map_err(|_error| Error::new(Span::call_site(), "CARGO_MANIFEST_DIR is unset?"))?;

        match lit {
            Lit::Str(lit_str) => {
                self.xml.push(&manifest_dir);
                self.xml.push("src");
                self.xml.push(lit_str.value());

                self.span = lit_str.span();
            }
            val => {
                return Err(Error::new_spanned(
                    val,
                    "`xml` must be a path to an XML file relative to the `src` dir of the calling crate",
                ));
            }
        }

        Ok(())
    }

    /// Parse the literal provided as a string or boolean.
    fn parse_zerocopy(&mut self, lit: &Lit) -> Result<()> {
        if self.zerocopy.is_some() {
            return Err(Error::new_spanned(lit, "Multiple `zerocopy` parameters."));
        }

        self.zerocopy = Self::parse_feature(lit, "zerocopy")?;

        Ok(())
    }

    /// Parse the literal provided as a string or boolean.
    fn parse_details(&mut self, lit: &Lit) -> Result<()> {
        if self.details.is_some() {
            return Err(Error::new_spanned(lit, "Multiple `details` parameters."));
        }

        self.details = Self::parse_feature(lit, "details")?;

        Ok(())
    }

    /// Parse a feature literal.
    ///
    /// If a boolean literal is passed, the given name will be used as the feature. If a string
    /// literal is given, that string will be used instead.
    fn parse_feature(lit: &Lit, name: &str) -> Result<Option<String>> {
        match lit {
            Lit::Bool(lit_bool) => Ok(lit_bool.value().then(|| name.to_owned())),
            Lit::Str(lit_str) => Ok(Some(lit_str.value())),
            val => {
                let message = format!("`{name}` must be bool or the literal feature name to use");

                Err(Error::new_spanned(val, message))
            }
        }
    }

    /// Parse a namevalue token pair.
    fn parse_namevalue(&mut self, tokens: &MetaNameValue) -> Result<()> {
        let ident = tokens
            .path
            .get_ident()
            .ok_or_else(|| syn::Error::new_spanned(tokens, "Must have specified ident"))?
            .to_string()
            .to_lowercase();
        let lit = match &tokens.value {
            Expr::Lit(ExprLit { lit, .. }) => lit,
            expr => return Err(Error::new_spanned(expr, "Must be a literal")),
        };

        match ident.as_str() {
            "xml" => self.parse_xml(lit),
            "zerocopy" => self.parse_zerocopy(lit),
            "details" => self.parse_details(lit),
            other => {
                let message = format!(
                    "{other} is not a valid paramter. The only valid parameters are `xml`, which should refer to a file relative to the calling crate's `src` directory, and `zerocopy`/`details`, which should be set to `true` or the relevant feature name."
                );
                Err(Error::new_spanned(ident, message))
            }
        }
    }

    /// Construct a new config from the given comma-separated tokens.
    #[expect(clippy::single_call_fn, reason = "Clean code.")]
    fn build(args: &Punctuated<Meta, Token![,]>) -> Result<Config> {
        let mut retval = Self {
            xml: PathBuf::default(),
            span: Span::call_site(),
            zerocopy: None,
            details: None,
        };

        for arg in args {
            match arg {
                Meta::Path(tokens) => {
                    return Err(Error::new_spanned(
                        tokens,
                        concat!(
                            "The only valid parameters are `xml`, which should refer to a file ",
                            "relative to the calling crate's `src` directory, `zerocopy`, and ",
                            "`details`, which should be set to `true` or the relevant feature name."
                        ),
                    ));
                }
                Meta::List(tokens) => {
                    return Err(Error::new_spanned(
                        tokens,
                        concat!(
                            "The only valid parameter is `xml`, which should refer to a file ",
                            "relative to the calling crate's `src` directory, `zerocopy`, and ",
                            "`details`, which should be set to `true` or the relevant feature name."
                        ),
                    ));
                }
                Meta::NameValue(tokens) => retval.parse_namevalue(tokens)?,
            }
        }

        Ok(retval)
    }
}

/// A function to generate the error enum.
#[expect(clippy::single_call_fn, reason = "Clean code")]
fn generate_error() -> TokenStream {
    quote::quote! {
        /// Errors which can be generated by the codes
        #[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[non_exhaustive]
        pub enum Error {
            /// The MIC was not found in the code enumeration.
            UnknownMic,
        }

        impl ::core::fmt::Display for Error {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    Self::UnknownMic => f.write_str("Unknown MIC"),
                }
            }
        }
    }
}

/// A function to generate the code enum.
#[expect(clippy::single_call_fn)]
fn generate_code(
    records: &RecordSet,
    zerocopy: Option<String>,
    details_tokens: &TokenStream,
) -> TokenStream {
    let doc = records.doc();
    let ident = records.code_ident();
    let code_bytes = records.code_bytes();
    let const_ident = records.const_ident();
    let deprecated = records.deprecated();

    let zc_tokens = if let Some(zc_feature) = zerocopy {
        quote::quote! {
            #[cfg_attr(
                feature = #zc_feature,
                derive(
                    ::zerocopy::Immutable,
                    ::zerocopy::IntoBytes,
                    ::zerocopy::KnownLayout,
                    ::zerocopy::TryFromBytes
                )
            )]
        }
    } else {
        quote::quote! {}
    };

    quote::quote! {
        /// An enumeration containing all currently known MICs.
        ///
        /// ```rust
        /// use iso10383_static::Code;
        /// use iso10383_types::mic;
        ///
        /// let mcode = mic::from_str("XNAS").expect("mic");
        /// let code = Code::from_mic(mcode).expect("code");
        /// assert_eq!(Code::Xnas, code);
        /// assert_eq!(mcode, code.as_mic());
        /// ```
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        #zc_tokens
        #[repr(u32)]
        #[non_exhaustive]
        pub enum Code {
            #(
                #[doc = #doc]
                #deprecated
                #ident = u32::from_be_bytes(*#code_bytes),
            )*
        }

        impl Code {
            /// Get an enumerated value for the given MIC value, if it exists.
            ///
            /// # Errors
            ///
            /// - [`Error::UnknownMic`] when the MIC value is unknown
            #[inline(never)]
            #[expect(deprecated, reason = "Deprecated Code enum values will be matched")]
            pub const fn from_mic(value: &::iso10383_types::mic) -> Result<Self, Error> {
                match value.as_bytes() {
                    #(
                        #code_bytes => Ok(Self::#ident),
                    )*

                    _ => Err(Error::UnknownMic),
                }
            }

            /// Retrieve the corresponding MIC.
            #[inline(never)]
            #[must_use]
            #[expect(deprecated, reason = "Deprecated Code enum values will be matched")]
            pub fn as_mic(&self) -> &'static ::iso10383_types::mic {
                match self {
                    #(
                        Self::#ident => {
                            match ::iso10383_types::mic::from_bytes(#code_bytes) {
                                Ok(value) => value,
                                Err(err) => panic!("Invalid MIC in generated code"),
                            }
                        },
                    )*
                }
            }

            /// Retrieve a struct containing constant details for this enum value.
            #[inline(never)]
            #[must_use]
            #[expect(deprecated, reason = "Deprecated Code enum values will be matched")]
            #details_tokens
            pub fn details(&self) -> &'static Record {
                match self {
                    #(
                        Self::#ident => &crate::#const_ident,
                    )*
                }
            }
        }

        impl Ord for Code {
            #[inline]
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                (&(*self as u32)).cmp(&(*other as u32))
            }
        }

        impl PartialOrd for Code {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
    }
}

/// Generate the constants.
#[expect(clippy::single_call_fn, reason = "Clean code")]
fn generate_const_data(records: &RecordSet, details_tokens: &TokenStream) -> TokenStream {
    let const_ident = records.const_ident();
    let code = records.code_ident();
    let operating_code = records.oper_ident();
    let kind = records.kind();
    let name = records.name();
    let legal_name = records.legal_name();
    let legal_id = records.legal_id();
    let category = records.category();
    let acronym = records.acronym();
    let country = records.alpha2();
    let city = records.city();
    let website = records.website();
    let status = records.status();
    let creation_date = records.creation();
    let last_update_date = records.last_update();
    let last_validation_date = records.last_validation();
    let expiry_date = records.expiry();
    let comments = records.comments();
    let count = const_ident.len();
    let deprecated = records.deprecated();

    quote::quote! {
        /// A constant array of references to all MIC records.
        #[expect(deprecated, reason = "Deprecated records will be included")]
        #details_tokens
        pub const ALL_MICS: [&Record; #count] = [
            #(
                &#const_ident,
            )*
        ];

        #(
            #[doc = #name]
            #details_tokens
            #deprecated
            #[expect(
                clippy::allow_attributes,
                reason = "Some records may use deprecated Code enum values, most will not"
            )]
            pub const #const_ident: Record = Record {
                #[allow(deprecated, reason = "Deprecated records use deprecated Code enum values")]
                code: Code::#code,
                #[allow(deprecated, reason = "Deprecated records use deprecated Code enum values")]
                operating_code: Code::#operating_code,
                kind: ::iso10383_types::Kind::#kind,
                name: #name,
                legal_entity_name: #legal_name,
                legal_entity_id: #legal_id,
                category: ::iso10383_types::Category::#category,
                acronym: #acronym,
                country: ::iso3166_static::Alpha2::#country,
                city: #city,
                website: #website,
                status: ::iso10383_types::Status::#status,
                creation_date: #creation_date,
                last_update_date: #last_update_date,
                last_validation_date: #last_validation_date,
                expiry_date: #expiry_date,
                comments: #comments,
            };

        )*
    }
}

/// Generate the static record data.
#[expect(clippy::single_call_fn)]
fn generate_data(records: &RecordSet, details_tokens: &TokenStream) -> TokenStream {
    let mut retval = quote::quote! {
        /// All data about a MIC.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(::serde::Deserialize, ::serde::Serialize))]
        pub struct Record {
            /// The market identifier code.
            pub code: Code,
            /// The code of the operating entity.
            pub operating_code: Code,
            /// The type of code.
            pub kind: ::iso10383_types::Kind,
            /// The name of the operating entity.
            pub name: &'static str,
            /// The name of the legal entity.
            pub legal_entity_name: Option<&'static str>,
            /// The legal entity ID.
            pub legal_entity_id: Option<&'static ::iso17442_types::lei>,
            /// The market category this code is operating in.
            pub category: ::iso10383_types::Category,
            /// The known acronym of the code.
            pub acronym: Option<&'static str>,
            /// The Alpha2 country code the organization operating this code is registered in.
            pub country: ::iso3166_static::Alpha2,
            /// The city the organization operating this code is registered in.
            pub city: &'static str,
            /// The website of the organization operating this code.
            pub website: Option<&'static str>,
            /// The status of this code.
            pub status: ::iso10383_types::Status,
            /// The date this record was created at.
            pub creation_date: ::jiff::civil::Date,
            /// The date this record was updated at.
            pub last_update_date: ::jiff::civil::Date,
            /// The date this record was last validated.
            pub last_validation_date: Option<::jiff::civil::Date>,
            /// The date this record was marked expired.
            pub expiry_date: Option<::jiff::civil::Date>,
            /// Any comments about this record.
            pub comments: Option<&'static str>,
        }
    };

    retval.extend(generate_const_data(records, details_tokens));

    retval
}

/// A function to start generating data from XML using the input tokens as configuration.
#[expect(clippy::single_call_fn)]
pub(crate) fn generate(input: TokenStream) -> Result<TokenStream> {
    let Config {
        xml,
        span,
        zerocopy,
        details,
    } = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(input)
        .and_then(|args| Config::build(&args))?;

    let file = File::open(&xml).map_err(|error| {
        let message = format!("Could not open `xml` file: {error}");
        Error::new(span, message)
    })?;

    let reader = BufReader::new(file);

    let output = de::from_reader::<_, MicList>(reader).map_err(|error| {
        let message = format!("Could not parse XML file: {error}");
        Error::new(span, message)
    })?;

    let mut raw_records = output.mics().to_vec();
    raw_records.sort_by_cached_key(|record| record.mic);

    let mut records = RecordSet::with_capacity(raw_records.len());
    for record in raw_records {
        records.push_record(&record, span)?;
    }

    let details_tokens = if let Some(feature_name) = details {
        quote::quote! {
            #[cfg(feature = #feature_name)]
        }
    } else {
        quote::quote! {}
    };

    let mut retval = generate_error();
    retval.extend(generate_code(&records, zerocopy, &details_tokens));
    retval.extend(generate_data(&records, &details_tokens));

    Ok(retval)
}
