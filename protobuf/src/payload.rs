// This file is generated by rust-protobuf 2.18.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `payload.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_1;

#[derive(PartialEq,Clone,Default)]
pub struct Payload {
    // message fields
    action: ::std::option::Option<Payload_Action>,
    name: ::protobuf::SingularField<::std::string::String>,
    number: ::std::option::Option<u32>,
    amount: ::std::option::Option<i32>,
    new_number: ::std::option::Option<u32>,
    timestamp: ::std::option::Option<i64>,
    public_key: ::protobuf::SingularField<::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Payload {
    fn default() -> &'a Payload {
        <Payload as ::protobuf::Message>::default_instance()
    }
}

impl Payload {
    pub fn new() -> Payload {
        ::std::default::Default::default()
    }

    // required .Payload.Action action = 1;


    pub fn get_action(&self) -> Payload_Action {
        self.action.unwrap_or(Payload_Action::DEPOSIT)
    }
    pub fn clear_action(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: Payload_Action) {
        self.action = ::std::option::Option::Some(v);
    }

    // required string name = 2;


    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    // optional uint32 number = 3;


    pub fn get_number(&self) -> u32 {
        self.number.unwrap_or(0)
    }
    pub fn clear_number(&mut self) {
        self.number = ::std::option::Option::None;
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: u32) {
        self.number = ::std::option::Option::Some(v);
    }

    // optional sint32 amount = 5;


    pub fn get_amount(&self) -> i32 {
        self.amount.unwrap_or(0)
    }
    pub fn clear_amount(&mut self) {
        self.amount = ::std::option::Option::None;
    }

    pub fn has_amount(&self) -> bool {
        self.amount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i32) {
        self.amount = ::std::option::Option::Some(v);
    }

    // optional uint32 new_number = 6;


    pub fn get_new_number(&self) -> u32 {
        self.new_number.unwrap_or(0)
    }
    pub fn clear_new_number(&mut self) {
        self.new_number = ::std::option::Option::None;
    }

    pub fn has_new_number(&self) -> bool {
        self.new_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_number(&mut self, v: u32) {
        self.new_number = ::std::option::Option::Some(v);
    }

    // optional sint64 timestamp = 7;


    pub fn get_timestamp(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }
    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    // optional string public_key = 8;


    pub fn get_public_key(&self) -> &str {
        match self.public_key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    pub fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::string::String) {
        self.public_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::string::String {
        if self.public_key.is_none() {
            self.public_key.set_default();
        }
        self.public_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::string::String {
        self.public_key.take().unwrap_or_else(|| ::std::string::String::new())
    }
}

impl ::protobuf::Message for Payload {
    fn is_initialized(&self) -> bool {
        if self.action.is_none() {
            return false;
        }
        if self.name.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_proto2_enum_with_unknown_fields_into(wire_type, is, &mut self.action, 1, &mut self.unknown_fields)?
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.number = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint32()?;
                    self.amount = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.new_number = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_sint64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.public_key)?;
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
        if let Some(v) = self.action {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.number {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.amount {
            my_size += ::protobuf::rt::value_varint_zigzag_size(5, v);
        }
        if let Some(v) = self.new_number {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_varint_zigzag_size(7, v);
        }
        if let Some(ref v) = self.public_key.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.action {
            os.write_enum(1, ::protobuf::ProtobufEnum::value(&v))?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.number {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.amount {
            os.write_sint32(5, v)?;
        }
        if let Some(v) = self.new_number {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.timestamp {
            os.write_sint64(7, v)?;
        }
        if let Some(ref v) = self.public_key.as_ref() {
            os.write_string(8, &v)?;
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

    fn new() -> Payload {
        Payload::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Payload_Action>>(
                "action",
                |m: &Payload| { &m.action },
                |m: &mut Payload| { &mut m.action },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Payload| { &m.name },
                |m: &mut Payload| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "number",
                |m: &Payload| { &m.number },
                |m: &mut Payload| { &mut m.number },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint32>(
                "amount",
                |m: &Payload| { &m.amount },
                |m: &mut Payload| { &mut m.amount },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "new_number",
                |m: &Payload| { &m.new_number },
                |m: &mut Payload| { &mut m.new_number },
            ));
            fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeSint64>(
                "timestamp",
                |m: &Payload| { &m.timestamp },
                |m: &mut Payload| { &mut m.timestamp },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "public_key",
                |m: &Payload| { &m.public_key },
                |m: &mut Payload| { &mut m.public_key },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Payload>(
                "Payload",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Payload {
        static instance: ::protobuf::rt::LazyV2<Payload> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Payload::new)
    }
}

impl ::protobuf::Clear for Payload {
    fn clear(&mut self) {
        self.action = ::std::option::Option::None;
        self.name.clear();
        self.number = ::std::option::Option::None;
        self.amount = ::std::option::Option::None;
        self.new_number = ::std::option::Option::None;
        self.timestamp = ::std::option::Option::None;
        self.public_key.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Payload {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Payload {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Payload_Action {
    DEPOSIT = 0,
    WITHDRAW = 1,
    UPDATE_NUMBER = 2,
    ADD_ACCOUNT = 3,
    ADD_MERCHANT = 4,
}

impl ::protobuf::ProtobufEnum for Payload_Action {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Payload_Action> {
        match value {
            0 => ::std::option::Option::Some(Payload_Action::DEPOSIT),
            1 => ::std::option::Option::Some(Payload_Action::WITHDRAW),
            2 => ::std::option::Option::Some(Payload_Action::UPDATE_NUMBER),
            3 => ::std::option::Option::Some(Payload_Action::ADD_ACCOUNT),
            4 => ::std::option::Option::Some(Payload_Action::ADD_MERCHANT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Payload_Action] = &[
            Payload_Action::DEPOSIT,
            Payload_Action::WITHDRAW,
            Payload_Action::UPDATE_NUMBER,
            Payload_Action::ADD_ACCOUNT,
            Payload_Action::ADD_MERCHANT,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Payload_Action>("Payload.Action", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Payload_Action {
}

impl ::std::default::Default for Payload_Action {
    fn default() -> Self {
        Payload_Action::DEPOSIT
    }
}

impl ::protobuf::reflect::ProtobufValue for Payload_Action {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rpayload.proto\"\xbf\x02\n\x07Payload\x12)\n\x06action\x18\x01\x20\
    \x02(\x0e2\x0f.Payload.ActionR\x06actionB\0\x12\x14\n\x04name\x18\x02\
    \x20\x02(\tR\x04nameB\0\x12\x18\n\x06number\x18\x03\x20\x01(\rR\x06numbe\
    rB\0\x12\x18\n\x06amount\x18\x05\x20\x01(\x11R\x06amountB\0\x12\x1f\n\nn\
    ew_number\x18\x06\x20\x01(\rR\tnewNumberB\0\x12\x1e\n\ttimestamp\x18\x07\
    \x20\x01(\x12R\ttimestampB\0\x12\x1f\n\npublic_key\x18\x08\x20\x01(\tR\t\
    publicKeyB\0\"[\n\x06Action\x12\x0b\n\x07DEPOSIT\x10\0\x12\x0c\n\x08WITH\
    DRAW\x10\x01\x12\x11\n\rUPDATE_NUMBER\x10\x02\x12\x0f\n\x0bADD_ACCOUNT\
    \x10\x03\x12\x10\n\x0cADD_MERCHANT\x10\x04\x1a\0:\0B\0b\x06proto2\
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
