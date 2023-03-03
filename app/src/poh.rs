// This file is generated by rust-protobuf 2.18.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `proto/poh.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_2;

#[derive(PartialEq,Clone,Default)]
pub struct InitializeRequest {
    // message fields
    pub pub_key: ::std::string::String,
    pub cert: ::std::string::String,
    pub msg: ::std::string::String,
    pub msg_sig: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a InitializeRequest {
    fn default() -> &'a InitializeRequest {
        <InitializeRequest as ::protobuf::Message>::default_instance()
    }
}

impl InitializeRequest {
    pub fn new() -> InitializeRequest {
        ::std::default::Default::default()
    }

    // string pub_key = 1;


    pub fn get_pub_key(&self) -> &str {
        &self.pub_key
    }
    pub fn clear_pub_key(&mut self) {
        self.pub_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_pub_key(&mut self, v: ::std::string::String) {
        self.pub_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pub_key(&mut self) -> &mut ::std::string::String {
        &mut self.pub_key
    }

    // Take field
    pub fn take_pub_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.pub_key, ::std::string::String::new())
    }

    // string cert = 2;


    pub fn get_cert(&self) -> &str {
        &self.cert
    }
    pub fn clear_cert(&mut self) {
        self.cert.clear();
    }

    // Param is passed by value, moved
    pub fn set_cert(&mut self, v: ::std::string::String) {
        self.cert = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cert(&mut self) -> &mut ::std::string::String {
        &mut self.cert
    }

    // Take field
    pub fn take_cert(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cert, ::std::string::String::new())
    }

    // string msg = 3;


    pub fn get_msg(&self) -> &str {
        &self.msg
    }
    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    // string msg_sig = 4;


    pub fn get_msg_sig(&self) -> &str {
        &self.msg_sig
    }
    pub fn clear_msg_sig(&mut self) {
        self.msg_sig.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg_sig(&mut self, v: ::std::string::String) {
        self.msg_sig = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg_sig(&mut self) -> &mut ::std::string::String {
        &mut self.msg_sig
    }

    // Take field
    pub fn take_msg_sig(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg_sig, ::std::string::String::new())
    }
}

impl ::protobuf::Message for InitializeRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.pub_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cert)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg_sig)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.pub_key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.pub_key);
        }
        if !self.cert.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.cert);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.msg);
        }
        if !self.msg_sig.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.msg_sig);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.pub_key.is_empty() {
            os.write_string(1, &self.pub_key)?;
        }
        if !self.cert.is_empty() {
            os.write_string(2, &self.cert)?;
        }
        if !self.msg.is_empty() {
            os.write_string(3, &self.msg)?;
        }
        if !self.msg_sig.is_empty() {
            os.write_string(4, &self.msg_sig)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> InitializeRequest {
        InitializeRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "pub_key",
                |m: &InitializeRequest| { &m.pub_key },
                |m: &mut InitializeRequest| { &mut m.pub_key },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "cert",
                |m: &InitializeRequest| { &m.cert },
                |m: &mut InitializeRequest| { &mut m.cert },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "msg",
                |m: &InitializeRequest| { &m.msg },
                |m: &mut InitializeRequest| { &mut m.msg },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "msg_sig",
                |m: &InitializeRequest| { &m.msg_sig },
                |m: &mut InitializeRequest| { &mut m.msg_sig },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<InitializeRequest>(
                "InitializeRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static InitializeRequest {
        static instance: ::protobuf::rt::LazyV2<InitializeRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(InitializeRequest::new)
    }
}

impl ::protobuf::Clear for InitializeRequest {
    fn clear(&mut self) {
        self.pub_key.clear();
        self.cert.clear();
        self.msg.clear();
        self.msg_sig.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InitializeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InitializeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InitializeResponse {
    // message fields
    pub msg: ::std::string::String,
    pub valid: bool,
    pub session_key: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a InitializeResponse {
    fn default() -> &'a InitializeResponse {
        <InitializeResponse as ::protobuf::Message>::default_instance()
    }
}

impl InitializeResponse {
    pub fn new() -> InitializeResponse {
        ::std::default::Default::default()
    }

    // string msg = 1;


    pub fn get_msg(&self) -> &str {
        &self.msg
    }
    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg(&mut self) -> &mut ::std::string::String {
        &mut self.msg
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.msg, ::std::string::String::new())
    }

    // bool valid = 2;


    pub fn get_valid(&self) -> bool {
        self.valid
    }
    pub fn clear_valid(&mut self) {
        self.valid = false;
    }

    // Param is passed by value, moved
    pub fn set_valid(&mut self, v: bool) {
        self.valid = v;
    }

    // string session_key = 3;


    pub fn get_session_key(&self) -> &str {
        &self.session_key
    }
    pub fn clear_session_key(&mut self) {
        self.session_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_session_key(&mut self, v: ::std::string::String) {
        self.session_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_session_key(&mut self) -> &mut ::std::string::String {
        &mut self.session_key
    }

    // Take field
    pub fn take_session_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.session_key, ::std::string::String::new())
    }
}

impl ::protobuf::Message for InitializeResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.msg)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.valid = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.session_key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.msg);
        }
        if self.valid != false {
            my_size += 2;
        }
        if !self.session_key.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.session_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.msg.is_empty() {
            os.write_string(1, &self.msg)?;
        }
        if self.valid != false {
            os.write_bool(2, self.valid)?;
        }
        if !self.session_key.is_empty() {
            os.write_string(3, &self.session_key)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> InitializeResponse {
        InitializeResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "msg",
                |m: &InitializeResponse| { &m.msg },
                |m: &mut InitializeResponse| { &mut m.msg },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                "valid",
                |m: &InitializeResponse| { &m.valid },
                |m: &mut InitializeResponse| { &mut m.valid },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "session_key",
                |m: &InitializeResponse| { &m.session_key },
                |m: &mut InitializeResponse| { &mut m.session_key },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<InitializeResponse>(
                "InitializeResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static InitializeResponse {
        static instance: ::protobuf::rt::LazyV2<InitializeResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(InitializeResponse::new)
    }
}

impl ::protobuf::Clear for InitializeResponse {
    fn clear(&mut self) {
        self.msg.clear();
        self.valid = false;
        self.session_key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InitializeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InitializeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PoHRootCertificates {
    // message fields
    pub rc: ::protobuf::RepeatedField<PoHRootCertificate>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PoHRootCertificates {
    fn default() -> &'a PoHRootCertificates {
        <PoHRootCertificates as ::protobuf::Message>::default_instance()
    }
}

impl PoHRootCertificates {
    pub fn new() -> PoHRootCertificates {
        ::std::default::Default::default()
    }

    // repeated .poh.PoHRootCertificate rc = 1;


    pub fn get_rc(&self) -> &[PoHRootCertificate] {
        &self.rc
    }
    pub fn clear_rc(&mut self) {
        self.rc.clear();
    }

    // Param is passed by value, moved
    pub fn set_rc(&mut self, v: ::protobuf::RepeatedField<PoHRootCertificate>) {
        self.rc = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rc(&mut self) -> &mut ::protobuf::RepeatedField<PoHRootCertificate> {
        &mut self.rc
    }

    // Take field
    pub fn take_rc(&mut self) -> ::protobuf::RepeatedField<PoHRootCertificate> {
        ::std::mem::replace(&mut self.rc, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for PoHRootCertificates {
    fn is_initialized(&self) -> bool {
        for v in &self.rc {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rc)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.rc {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.rc {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PoHRootCertificates {
        PoHRootCertificates::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PoHRootCertificate>>(
                "rc",
                |m: &PoHRootCertificates| { &m.rc },
                |m: &mut PoHRootCertificates| { &mut m.rc },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PoHRootCertificates>(
                "PoHRootCertificates",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PoHRootCertificates {
        static instance: ::protobuf::rt::LazyV2<PoHRootCertificates> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PoHRootCertificates::new)
    }
}

impl ::protobuf::Clear for PoHRootCertificates {
    fn clear(&mut self) {
        self.rc.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoHRootCertificates {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoHRootCertificates {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PoHRootCertificate {
    // message fields
    pub public_key: ::std::string::String,
    pub era_id: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a PoHRootCertificate {
    fn default() -> &'a PoHRootCertificate {
        <PoHRootCertificate as ::protobuf::Message>::default_instance()
    }
}

impl PoHRootCertificate {
    pub fn new() -> PoHRootCertificate {
        ::std::default::Default::default()
    }

    // string public_key = 1;


    pub fn get_public_key(&self) -> &str {
        &self.public_key
    }
    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::string::String) {
        self.public_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::string::String {
        &mut self.public_key
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.public_key, ::std::string::String::new())
    }

    // int32 era_id = 2;


    pub fn get_era_id(&self) -> i32 {
        self.era_id
    }
    pub fn clear_era_id(&mut self) {
        self.era_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_era_id(&mut self, v: i32) {
        self.era_id = v;
    }
}

impl ::protobuf::Message for PoHRootCertificate {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.public_key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.era_id = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.public_key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.public_key);
        }
        if self.era_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.era_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.public_key.is_empty() {
            os.write_string(1, &self.public_key)?;
        }
        if self.era_id != 0 {
            os.write_int32(2, self.era_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> PoHRootCertificate {
        PoHRootCertificate::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "public_key",
                |m: &PoHRootCertificate| { &m.public_key },
                |m: &mut PoHRootCertificate| { &mut m.public_key },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "era_id",
                |m: &PoHRootCertificate| { &m.era_id },
                |m: &mut PoHRootCertificate| { &mut m.era_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<PoHRootCertificate>(
                "PoHRootCertificate",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static PoHRootCertificate {
        static instance: ::protobuf::rt::LazyV2<PoHRootCertificate> = ::protobuf::rt::LazyV2::INIT;
        instance.get(PoHRootCertificate::new)
    }
}

impl ::protobuf::Clear for PoHRootCertificate {
    fn clear(&mut self) {
        self.public_key.clear();
        self.era_id = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PoHRootCertificate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoHRootCertificate {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fproto/poh.proto\x12\x03poh\"k\n\x11InitializeRequest\x12\x17\n\x07\
    pub_key\x18\x01\x20\x01(\tR\x06pubKey\x12\x12\n\x04cert\x18\x02\x20\x01(\
    \tR\x04cert\x12\x10\n\x03msg\x18\x03\x20\x01(\tR\x03msg\x12\x17\n\x07msg\
    _sig\x18\x04\x20\x01(\tR\x06msgSig\"]\n\x12InitializeResponse\x12\x10\n\
    \x03msg\x18\x01\x20\x01(\tR\x03msg\x12\x14\n\x05valid\x18\x02\x20\x01(\
    \x08R\x05valid\x12\x1f\n\x0bsession_key\x18\x03\x20\x01(\tR\nsessionKey\
    \">\n\x13PoHRootCertificates\x12'\n\x02rc\x18\x01\x20\x03(\x0b2\x17.poh.\
    PoHRootCertificateR\x02rc\"J\n\x12PoHRootCertificate\x12\x1d\n\npublic_k\
    ey\x18\x01\x20\x01(\tR\tpublicKey\x12\x15\n\x06era_id\x18\x02\x20\x01(\
    \x05R\x05eraId2D\n\x03PoH\x12=\n\ninitialize\x12\x16.poh.InitializeReque\
    st\x1a\x17.poh.InitializeResponseb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
