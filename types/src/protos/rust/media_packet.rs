// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `types/media_packet.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:MediaPacket)
pub struct MediaPacket {
    // message fields
    // @@protoc_insertion_point(field:MediaPacket.media_type)
    pub media_type: ::protobuf::EnumOrUnknown<media_packet::MediaType>,
    // @@protoc_insertion_point(field:MediaPacket.email)
    pub email: ::std::string::String,
    // @@protoc_insertion_point(field:MediaPacket.video)
    pub video: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:MediaPacket.audio)
    pub audio: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:MediaPacket.video_type)
    pub video_type: ::std::string::String,
    // @@protoc_insertion_point(field:MediaPacket.timestamp)
    pub timestamp: f64,
    // @@protoc_insertion_point(field:MediaPacket.duration)
    pub duration: f64,
    // @@protoc_insertion_point(field:MediaPacket.audio_format)
    pub audio_format: ::std::string::String,
    // @@protoc_insertion_point(field:MediaPacket.audio_number_of_channels)
    pub audio_number_of_channels: u32,
    // @@protoc_insertion_point(field:MediaPacket.audio_number_of_frames)
    pub audio_number_of_frames: u32,
    // @@protoc_insertion_point(field:MediaPacket.audio_sample_rate)
    pub audio_sample_rate: f32,
    // special fields
    // @@protoc_insertion_point(special_field:MediaPacket.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MediaPacket {
    fn default() -> &'a MediaPacket {
        <MediaPacket as ::protobuf::Message>::default_instance()
    }
}

impl MediaPacket {
    pub fn new() -> MediaPacket {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(11);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "media_type",
            |m: &MediaPacket| { &m.media_type },
            |m: &mut MediaPacket| { &mut m.media_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "email",
            |m: &MediaPacket| { &m.email },
            |m: &mut MediaPacket| { &mut m.email },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "video",
            |m: &MediaPacket| { &m.video },
            |m: &mut MediaPacket| { &mut m.video },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "audio",
            |m: &MediaPacket| { &m.audio },
            |m: &mut MediaPacket| { &mut m.audio },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "video_type",
            |m: &MediaPacket| { &m.video_type },
            |m: &mut MediaPacket| { &mut m.video_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "timestamp",
            |m: &MediaPacket| { &m.timestamp },
            |m: &mut MediaPacket| { &mut m.timestamp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "duration",
            |m: &MediaPacket| { &m.duration },
            |m: &mut MediaPacket| { &mut m.duration },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "audio_format",
            |m: &MediaPacket| { &m.audio_format },
            |m: &mut MediaPacket| { &mut m.audio_format },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "audio_number_of_channels",
            |m: &MediaPacket| { &m.audio_number_of_channels },
            |m: &mut MediaPacket| { &mut m.audio_number_of_channels },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "audio_number_of_frames",
            |m: &MediaPacket| { &m.audio_number_of_frames },
            |m: &mut MediaPacket| { &mut m.audio_number_of_frames },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "audio_sample_rate",
            |m: &MediaPacket| { &m.audio_sample_rate },
            |m: &mut MediaPacket| { &mut m.audio_sample_rate },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MediaPacket>(
            "MediaPacket",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MediaPacket {
    const NAME: &'static str = "MediaPacket";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.media_type = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.email = is.read_string()?;
                },
                26 => {
                    self.video = is.read_bytes()?;
                },
                34 => {
                    self.audio = is.read_bytes()?;
                },
                42 => {
                    self.video_type = is.read_string()?;
                },
                49 => {
                    self.timestamp = is.read_double()?;
                },
                57 => {
                    self.duration = is.read_double()?;
                },
                66 => {
                    self.audio_format = is.read_string()?;
                },
                72 => {
                    self.audio_number_of_channels = is.read_uint32()?;
                },
                80 => {
                    self.audio_number_of_frames = is.read_uint32()?;
                },
                93 => {
                    self.audio_sample_rate = is.read_float()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.media_type != ::protobuf::EnumOrUnknown::new(media_packet::MediaType::VIDEO) {
            my_size += ::protobuf::rt::int32_size(1, self.media_type.value());
        }
        if !self.email.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.email);
        }
        if !self.video.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.video);
        }
        if !self.audio.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.audio);
        }
        if !self.video_type.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.video_type);
        }
        if self.timestamp != 0. {
            my_size += 1 + 8;
        }
        if self.duration != 0. {
            my_size += 1 + 8;
        }
        if !self.audio_format.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.audio_format);
        }
        if self.audio_number_of_channels != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.audio_number_of_channels);
        }
        if self.audio_number_of_frames != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.audio_number_of_frames);
        }
        if self.audio_sample_rate != 0. {
            my_size += 1 + 4;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.media_type != ::protobuf::EnumOrUnknown::new(media_packet::MediaType::VIDEO) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.media_type))?;
        }
        if !self.email.is_empty() {
            os.write_string(2, &self.email)?;
        }
        if !self.video.is_empty() {
            os.write_bytes(3, &self.video)?;
        }
        if !self.audio.is_empty() {
            os.write_bytes(4, &self.audio)?;
        }
        if !self.video_type.is_empty() {
            os.write_string(5, &self.video_type)?;
        }
        if self.timestamp != 0. {
            os.write_double(6, self.timestamp)?;
        }
        if self.duration != 0. {
            os.write_double(7, self.duration)?;
        }
        if !self.audio_format.is_empty() {
            os.write_string(8, &self.audio_format)?;
        }
        if self.audio_number_of_channels != 0 {
            os.write_uint32(9, self.audio_number_of_channels)?;
        }
        if self.audio_number_of_frames != 0 {
            os.write_uint32(10, self.audio_number_of_frames)?;
        }
        if self.audio_sample_rate != 0. {
            os.write_float(11, self.audio_sample_rate)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MediaPacket {
        MediaPacket::new()
    }

    fn clear(&mut self) {
        self.media_type = ::protobuf::EnumOrUnknown::new(media_packet::MediaType::VIDEO);
        self.email.clear();
        self.video.clear();
        self.audio.clear();
        self.video_type.clear();
        self.timestamp = 0.;
        self.duration = 0.;
        self.audio_format.clear();
        self.audio_number_of_channels = 0;
        self.audio_number_of_frames = 0;
        self.audio_sample_rate = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MediaPacket {
        static instance: MediaPacket = MediaPacket {
            media_type: ::protobuf::EnumOrUnknown::from_i32(0),
            email: ::std::string::String::new(),
            video: ::std::vec::Vec::new(),
            audio: ::std::vec::Vec::new(),
            video_type: ::std::string::String::new(),
            timestamp: 0.,
            duration: 0.,
            audio_format: ::std::string::String::new(),
            audio_number_of_channels: 0,
            audio_number_of_frames: 0,
            audio_sample_rate: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MediaPacket {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MediaPacket").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MediaPacket {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MediaPacket {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `MediaPacket`
pub mod media_packet {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:MediaPacket.MediaType)
    pub enum MediaType {
        // @@protoc_insertion_point(enum_value:MediaPacket.MediaType.VIDEO)
        VIDEO = 0,
        // @@protoc_insertion_point(enum_value:MediaPacket.MediaType.AUDIO)
        AUDIO = 1,
    }

    impl ::protobuf::Enum for MediaType {
        const NAME: &'static str = "MediaType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<MediaType> {
            match value {
                0 => ::std::option::Option::Some(MediaType::VIDEO),
                1 => ::std::option::Option::Some(MediaType::AUDIO),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [MediaType] = &[
            MediaType::VIDEO,
            MediaType::AUDIO,
        ];
    }

    impl ::protobuf::EnumFull for MediaType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("MediaPacket.MediaType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for MediaType {
        fn default() -> Self {
            MediaType::VIDEO
        }
    }

    impl MediaType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<MediaType>("MediaPacket.MediaType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18types/media_packet.proto\"\xbf\x03\n\x0bMediaPacket\x125\n\nmedia_\
    type\x18\x01\x20\x01(\x0e2\x16.MediaPacket.MediaTypeR\tmediaType\x12\x14\
    \n\x05email\x18\x02\x20\x01(\tR\x05email\x12\x14\n\x05video\x18\x03\x20\
    \x01(\x0cR\x05video\x12\x14\n\x05audio\x18\x04\x20\x01(\x0cR\x05audio\
    \x12\x1d\n\nvideo_type\x18\x05\x20\x01(\tR\tvideoType\x12\x1c\n\ttimesta\
    mp\x18\x06\x20\x01(\x01R\ttimestamp\x12\x1a\n\x08duration\x18\x07\x20\
    \x01(\x01R\x08duration\x12!\n\x0caudio_format\x18\x08\x20\x01(\tR\x0baud\
    ioFormat\x127\n\x18audio_number_of_channels\x18\t\x20\x01(\rR\x15audioNu\
    mberOfChannels\x123\n\x16audio_number_of_frames\x18\n\x20\x01(\rR\x13aud\
    ioNumberOfFrames\x12*\n\x11audio_sample_rate\x18\x0b\x20\x01(\x02R\x0fau\
    dioSampleRate\"!\n\tMediaType\x12\t\n\x05VIDEO\x10\0\x12\t\n\x05AUDIO\
    \x10\x01J\x81\x06\n\x06\x12\x04\0\0\x12\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\n\n\x02\x04\0\x12\x04\x02\0\x12\x01\n\n\n\x03\x04\0\x01\x12\x03\
    \x02\x08\x13\n\x0c\n\x04\x04\0\x04\0\x12\x04\x03\x02\x06\x03\n\x0c\n\x05\
    \x04\0\x04\0\x01\x12\x03\x03\x07\x10\n\r\n\x06\x04\0\x04\0\x02\0\x12\x03\
    \x04\x04\x0e\n\x0e\n\x07\x04\0\x04\0\x02\0\x01\x12\x03\x04\x04\t\n\x0e\n\
    \x07\x04\0\x04\0\x02\0\x02\x12\x03\x04\x0c\r\n\r\n\x06\x04\0\x04\0\x02\
    \x01\x12\x03\x05\x04\x0e\n\x0e\n\x07\x04\0\x04\0\x02\x01\x01\x12\x03\x05\
    \x04\t\n\x0e\n\x07\x04\0\x04\0\x02\x01\x02\x12\x03\x05\x0c\r\n\x0b\n\x04\
    \x04\0\x02\0\x12\x03\x07\x02\x1b\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\x07\
    \x02\x0b\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x07\x0c\x16\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03\x07\x19\x1a\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x08\
    \x02\x13\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x08\x02\x08\n\x0c\n\x05\
    \x04\0\x02\x01\x01\x12\x03\x08\t\x0e\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x03\x08\x11\x12\n\x0b\n\x04\x04\0\x02\x02\x12\x03\t\x02\x12\n\x0c\n\x05\
    \x04\0\x02\x02\x05\x12\x03\t\x02\x07\n\x0c\n\x05\x04\0\x02\x02\x01\x12\
    \x03\t\x08\r\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\t\x10\x11\n\x0b\n\x04\
    \x04\0\x02\x03\x12\x03\n\x02\x12\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\n\
    \x02\x07\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\n\x08\r\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03\n\x10\x11\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x0b\x02\
    \x18\n\x0c\n\x05\x04\0\x02\x04\x05\x12\x03\x0b\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x04\x01\x12\x03\x0b\t\x13\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x0b\
    \x16\x17\n\x0b\n\x04\x04\0\x02\x05\x12\x03\x0c\x02\x17\n\x0c\n\x05\x04\0\
    \x02\x05\x05\x12\x03\x0c\x02\x08\n\x0c\n\x05\x04\0\x02\x05\x01\x12\x03\
    \x0c\t\x12\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\x0c\x15\x16\n\x0b\n\x04\
    \x04\0\x02\x06\x12\x03\r\x02\x16\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\r\
    \x02\x08\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\r\t\x11\n\x0c\n\x05\x04\0\
    \x02\x06\x03\x12\x03\r\x14\x15\n\x0b\n\x04\x04\0\x02\x07\x12\x03\x0e\x02\
    \x1a\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x03\x0e\x02\x08\n\x0c\n\x05\x04\0\
    \x02\x07\x01\x12\x03\x0e\t\x15\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x0e\
    \x18\x19\n\x0b\n\x04\x04\0\x02\x08\x12\x03\x0f\x02&\n\x0c\n\x05\x04\0\
    \x02\x08\x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\
    \x0f\t!\n\x0c\n\x05\x04\0\x02\x08\x03\x12\x03\x0f$%\n\x0b\n\x04\x04\0\
    \x02\t\x12\x03\x10\x02%\n\x0c\n\x05\x04\0\x02\t\x05\x12\x03\x10\x02\x08\
    \n\x0c\n\x05\x04\0\x02\t\x01\x12\x03\x10\t\x1f\n\x0c\n\x05\x04\0\x02\t\
    \x03\x12\x03\x10\"$\n\x0b\n\x04\x04\0\x02\n\x12\x03\x11\x02\x1f\n\x0c\n\
    \x05\x04\0\x02\n\x05\x12\x03\x11\x02\x07\n\x0c\n\x05\x04\0\x02\n\x01\x12\
    \x03\x11\x08\x19\n\x0c\n\x05\x04\0\x02\n\x03\x12\x03\x11\x1c\x1eb\x06pro\
    to3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MediaPacket::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(media_packet::MediaType::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}