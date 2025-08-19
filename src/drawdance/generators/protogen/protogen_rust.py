#!/usr/bin/env python3
# Copyright (c) 2025 Claude Code
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

import sys
import re
import protogen
from typing import Dict, List, Any


class RustFieldType:
    """Base class for Rust field type mappings"""
    
    def __init__(self, rust_type: str, is_array: bool = False):
        self.rust_type = rust_type
        self.is_array = is_array
    
    def get_type_name(self, _field) -> str:
        """Get the Rust type name for this field"""
        return self.rust_type
    
    def get_serialize_expr(self, field_name: str) -> str:
        """Get expression for serializing this field"""
        return f"writer.write_{self.rust_type.lower()}({field_name})?;"
    
    def get_deserialize_expr(self, field_name: str) -> str:
        """Get expression for deserializing this field"""
        return f"let {field_name} = reader.read_{self.rust_type.lower()}()?;"
    
    def get_size_expr(self, _field_name: str) -> str:
        """Get expression for calculating size of this field"""
        return f"std::mem::size_of::<{self.rust_type}>()"


class RustIntegerFieldType(RustFieldType):
    """Integer field types"""
    
    def __init__(self, rust_type: str, byte_size: int):
        super().__init__(rust_type)
        self.byte_size = byte_size
    
    def get_serialize_expr(self, field_name: str) -> str:
        if self.rust_type.startswith('i'):
            return f"writer.write_i{self.byte_size * 8}({field_name})?;"
        else:
            return f"writer.write_u{self.byte_size * 8}({field_name})?;"
    
    def get_deserialize_expr(self, field_name: str) -> str:
        if self.rust_type.startswith('i'):
            return f"let {field_name} = reader.read_i{self.byte_size * 8}()?;"
        else:
            return f"let {field_name} = reader.read_u{self.byte_size * 8}()?;"
    
    def get_size_expr(self, _field_name: str) -> str:
        return str(self.byte_size)


class RustBoolFieldType(RustFieldType):
    """Boolean field type"""
    
    def __init__(self):
        super().__init__("bool")
    
    def get_serialize_expr(self, field_name: str) -> str:
        return f"writer.write_u8(if {field_name} {{ 1 }} else {{ 0 }})?;"
    
    def get_deserialize_expr(self, field_name: str) -> str:
        return f"let {field_name} = reader.read_u8()? != 0;"
    
    def get_size_expr(self, _field_name: str) -> str:
        return "1"


class RustStringFieldType(RustFieldType):
    """String field type"""
    
    def __init__(self):
        super().__init__("String", is_array=True)
    
    def get_type_name(self, _field) -> str:
        return "String"
    
    def get_serialize_expr(self, field_name: str) -> str:
        return f"""
        let bytes = {field_name}.as_bytes();
        writer.write_u16(bytes.len() as u16)?;
        writer.write_all(bytes)?;"""
    
    def get_deserialize_expr(self, field_name: str) -> str:
        return f"""
        let len = reader.read_u16()? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        let {field_name} = String::from_utf8(buf)?;"""
    
    def get_size_expr(self, field_name: str) -> str:
        return f"2 + {field_name}.len()"


class RustBytesFieldType(RustFieldType):
    """Byte array field type"""
    
    def __init__(self):
        super().__init__("Vec<u8>", is_array=True)
    
    def get_type_name(self, _field) -> str:
        return "Vec<u8>"
    
    def get_serialize_expr(self, field_name: str) -> str:
        return f"""
        writer.write_u16({field_name}.len() as u16)?;
        writer.write_all(&{field_name})?;"""
    
    def get_deserialize_expr(self, field_name: str) -> str:
        return f"""
        let len = reader.read_u16()? as usize;
        let mut {field_name} = vec![0u8; len];
        reader.read_exact(&mut {field_name})?;"""
    
    def get_size_expr(self, field_name: str) -> str:
        return f"2 + {field_name}.len()"


class RustVectorFieldType(RustFieldType):
    """Vector field types for typed arrays"""
    
    def __init__(self, element_type: str, element_size: int):
        super().__init__(f"Vec<{element_type}>", is_array=True)
        self.element_type = element_type
        self.element_size = element_size
    
    def get_serialize_expr(self, field_name: str) -> str:
        return f"""
        writer.write_u16({field_name}.len() as u16)?;
        for item in &{field_name} {{
            writer.write_{self.element_type.lower().replace('u', 'u').replace('i', 'i')}(*item)?;
        }}"""
    
    def get_deserialize_expr(self, field_name: str) -> str:
        return f"""
        let len = reader.read_u16()? as usize;
        let mut {field_name} = Vec::with_capacity(len);
        for _ in 0..len {{
            {field_name}.push(reader.read_{self.element_type.lower().replace('u', 'u').replace('i', 'i')}()?);
        }}"""
    
    def get_size_expr(self, field_name: str) -> str:
        return f"2 + {field_name}.len() * {self.element_size}"


class RustEnumFieldType(RustFieldType):
    """Enum field type"""
    
    def __init__(self, enum_name: str, variants: List[str]):
        super().__init__(enum_name)
        self.enum_name = enum_name
        self.variants = variants
    
    def get_serialize_expr(self, field_name: str) -> str:
        return f"writer.write_u8({field_name} as u8)?;"
    
    def get_deserialize_expr(self, field_name: str) -> str:
        return f"let {field_name} = {self.enum_name}::from_u8(reader.read_u8()?)?;"
    
    def get_size_expr(self, _field_name: str) -> str:
        return "1"


class RustFlagsFieldType(RustFieldType):
    """Flags field type"""
    
    def __init__(self, flags: List[tuple], size: int):
        super().__init__(f"u{size * 8}")
        self.flags = flags
        self.size = size
    
    def get_serialize_expr(self, field_name: str) -> str:
        return f"writer.write_u{self.size * 8}({field_name})?;"
    
    def get_deserialize_expr(self, field_name: str) -> str:
        return f"let {field_name} = reader.read_u{self.size * 8}()?;"
    
    def get_size_expr(self, field_name: str) -> str:
        return str(self.size)


class RustStructFieldType(RustFieldType):
    """Struct field type for nested structures"""
    
    def __init__(self, struct_name: str):
        super().__init__(f"Vec<{struct_name}>", is_array=True)
        self.struct_name = struct_name
    
    def get_serialize_expr(self, field_name: str) -> str:
        return f"""
        writer.write_u16({field_name}.len() as u16)?;
        for item in &{field_name} {{
            item.serialize(writer)?;
        }}"""
    
    def get_deserialize_expr(self, field_name: str) -> str:
        return f"""
        let len = reader.read_u16()? as usize;
        let mut {field_name} = Vec::with_capacity(len);
        for _ in 0..len {{
            {field_name}.push({self.struct_name}::deserialize(reader)?);
        }}"""
    
    def get_size_expr(self, field_name: str) -> str:
        return f"2 + {field_name}.iter().map(|item| item.serialized_size()).sum::<usize>()"


# Field type registry
RUST_FIELD_TYPES = {
    'i8': RustIntegerFieldType('i8', 1),
    'i16': RustIntegerFieldType('i16', 2),
    'i24': RustIntegerFieldType('i32', 3),  # Use i32 for 24-bit
    'i32': RustIntegerFieldType('i32', 4),
    'u8': RustIntegerFieldType('u8', 1),
    'u16': RustIntegerFieldType('u16', 2),
    'u24': RustIntegerFieldType('u32', 3),  # Use u32 for 24-bit
    'u32': RustIntegerFieldType('u32', 4),
    'bool': RustBoolFieldType(),
    'argb32': RustIntegerFieldType('u32', 4),
    'rgb24': RustIntegerFieldType('u32', 3),
    'blendmode': RustIntegerFieldType('u8', 1),
    'String': RustStringFieldType(),
    'Bytes': RustBytesFieldType(),
    'Vec<u8>': RustVectorFieldType('u8', 1),
    'Vec<u16>': RustVectorFieldType('u16', 2),
    'Vec<u24>': RustVectorFieldType('u32', 3),
    'Vec<u32>': RustVectorFieldType('u32', 4),
    'Vec<i32>': RustVectorFieldType('i32', 4),
}


class RustField:
    """Represents a field in a Rust message struct"""
    
    def __init__(self, field, message_name: str):
        # Handle Rust keywords by prefixing with msg_
        field_name = field.name
        if field_name == 'type':
            field_name = 'msg_type'
        self.name = to_snake_case(field_name)
        self.original_field = field
        self.field_type = self._get_field_type(field, message_name)
        self.comment = getattr(field, 'comment', '')
        
    def _get_field_type(self, field, message_name: str) -> RustFieldType:
        """Determine the Rust field type from protogen field"""
        field_type_name = field.field_type
        
        # Handle enums
        if hasattr(field, 'variants') and field.variants:
            enum_name = f"{to_pascal_case(message_name)}{to_pascal_case(field.name)}"
            return RustEnumFieldType(enum_name, field.variants)
        
        # Handle flags
        if hasattr(field, 'flags') and field.flags:
            size = 1 if len(field.flags) <= 8 else 2 if len(field.flags) <= 16 else 4
            return RustFlagsFieldType(field.flags, size)
        
        # Handle struct fields
        if field_type_name == 'struct':
            struct_name = to_pascal_case(field.struct_name)
            return RustStructFieldType(struct_name)
        
        # Handle regular types
        if field_type_name in RUST_FIELD_TYPES:
            return RUST_FIELD_TYPES[field_type_name]
        
        raise ValueError(f"Unknown field type: {field_type_name}")
    
    @property
    def rust_type(self) -> str:
        """Get the Rust type string for this field"""
        return self.field_type.get_type_name(self.original_field)
    
    @property
    def serialize_expr(self) -> str:
        """Get the serialization expression"""
        return self.field_type.get_serialize_expr(f"self.{self.name}")
    
    @property
    def deserialize_expr(self) -> str:
        """Get the deserialization expression"""
        return self.field_type.get_deserialize_expr(self.name)
    
    @property
    def size_expr(self) -> str:
        """Get the size calculation expression"""
        return self.field_type.get_size_expr(f"self.{self.name}")


class RustMessage:
    """Represents a message struct in Rust"""
    
    def __init__(self, message):
        self.name = message.name
        self.id = message.id
        self.comment = message.comment
        self.reserved = message.reserved
        self.alias = message.alias
        self.fields = []
        self.enums = []
        self.structs = []
        
        if not self.reserved:
            if not self.alias:
                self._process_fields(message)
            else:
                # Alias messages will be handled during init_alias
                pass
    
    def _process_fields(self, message):
        """Process protogen fields into Rust fields"""
        for field in message.fields:
            rust_field = RustField(field, self.name)
            self.fields.append(rust_field)
            
            # Collect enums and structs
            if isinstance(rust_field.field_type, RustEnumFieldType):
                self.enums.append((rust_field.field_type.enum_name, rust_field.field_type.variants))
            elif isinstance(rust_field.field_type, RustStructFieldType):
                self._process_struct_field(field)
    
    def _process_struct_field(self, field):
        """Process struct fields"""
        if hasattr(field, 'subfields'):
            struct_name = to_pascal_case(field.struct_name)
            struct_fields = []
            for subfield in field.subfields:
                rust_subfield = RustField(subfield, struct_name)
                struct_fields.append(rust_subfield)
            self.structs.append((struct_name, struct_fields))
    
    @property
    def rust_name(self) -> str:
        """Get the Rust struct name"""
        return to_pascal_case(self.name)
    
    @property
    def enum_name(self) -> str:
        """Get the message type enum name"""
        return to_screaming_snake_case(self.name)
    
    def init_alias(self, messages):
        """Initialize alias messages by copying from their targets"""
        if self.alias:
            for msg in messages:
                if msg.name == self.alias:
                    self.fields = msg.fields
                    self.enums = msg.enums
                    self.structs = msg.structs
                    break


def to_snake_case(name: str) -> str:
    """Convert CamelCase to snake_case"""
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()


def to_pascal_case(name: str) -> str:
    """Convert snake_case or other formats to PascalCase"""
    # Handle special cases
    if name.lower() == 'acl':
        return 'Acl'
    return ''.join(word.capitalize() for word in re.split(r'[_\s]+', name))


def to_screaming_snake_case(name: str) -> str:
    """Convert to SCREAMING_SNAKE_CASE"""
    return to_snake_case(name).upper()


def generate_rust_code(protocol: Dict[str, Any]) -> str:
    """Generate complete Rust code from protocol definition"""
    
    messages = [RustMessage(msg) for msg in protocol['messages']]
    
    # Initialize alias messages
    for msg in messages:
        msg.init_alias(messages)
    
    non_reserved_messages = [msg for msg in messages if not msg.reserved]
    
    # Collect all enums and structs (deduplicated)
    all_enums = []
    all_structs = []
    seen_enums = set()
    seen_structs = set()
    
    for msg in non_reserved_messages:
        for enum_name, variants in msg.enums:
            if enum_name not in seen_enums:
                all_enums.append((enum_name, variants))
                seen_enums.add(enum_name)
        
        for struct_name, fields in msg.structs:
            if struct_name not in seen_structs:
                all_structs.append((struct_name, fields))
                seen_structs.add(struct_name)
    
    # Generate code sections
    code_parts = []
    
    # File header
    code_parts.append(generate_file_header())
    
    # Imports and modules
    code_parts.append(generate_imports())
    
    # Error types
    code_parts.append(generate_error_types())
    
    # Enums
    if all_enums:
        code_parts.append(generate_enums(all_enums))
    
    # Message type enum
    code_parts.append(generate_message_type_enum(non_reserved_messages))
    
    # Structs for nested data
    if all_structs:
        code_parts.append(generate_structs(all_structs))
    
    # Message structs
    code_parts.append(generate_message_structs(non_reserved_messages))
    
    # Message enum
    code_parts.append(generate_message_enum(non_reserved_messages))
    
    # Serialization/deserialization traits
    code_parts.append(generate_serialization_traits())
    
    # Message implementations
    code_parts.append(generate_message_implementations(non_reserved_messages))
    
    return '\n\n'.join(code_parts)


def generate_file_header() -> str:
    """Generate file header with copyright and description"""
    return '''// SPDX-License-Identifier: MIT
//
// Generated code - do not edit manually
// This file was generated from protocol.yaml

#![allow(clippy::all)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]'''


def generate_imports() -> str:
    """Generate import statements"""
    return '''use std::io::{Read, Write, Result as IoResult};'''


def generate_error_types() -> str:
    """Generate error types"""
    return '''#[derive(Debug)]
pub enum ProtocolError {
    IoError(std::io::Error),
    InvalidEnumValue(u8),
    InvalidMessageType(u8),
    InvalidStringData,
}

impl From<std::io::Error> for ProtocolError {
    fn from(error: std::io::Error) -> Self {
        ProtocolError::IoError(error)
    }
}

impl From<std::string::FromUtf8Error> for ProtocolError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        ProtocolError::InvalidStringData
    }
}

pub type Result<T> = std::result::Result<T, ProtocolError>;'''


def generate_enums(enums: List[tuple]) -> str:
    """Generate enum definitions"""
    enum_code = []
    
    for enum_name, variants in enums:
        enum_code.append(f'''#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum {enum_name} {{''')
        
        for i, variant in enumerate(variants):
            enum_code.append(f'    {to_pascal_case(variant)} = {i},')
        
        enum_code.append('}')
        enum_code.append('')
        
        # Add FromU8 implementation
        enum_code.append(f'''impl {enum_name} {{
    pub fn from_u8(value: u8) -> Result<Self> {{
        match value {{''')
        
        for i, variant in enumerate(variants):
            enum_code.append(f'            {i} => Ok({enum_name}::{to_pascal_case(variant)}),')
        
        enum_code.append(f'''            _ => Err(ProtocolError::InvalidEnumValue(value)),
        }}
    }}
}}''')
        enum_code.append('')
    
    return '\n'.join(enum_code)


def generate_message_type_enum(messages: List[RustMessage]) -> str:
    """Generate the main message type enum"""
    # Only include non-alias messages
    concrete_messages = [msg for msg in messages if not msg.alias]
    
    return f'''#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {{
{chr(10).join(f"    {msg.enum_name} = {msg.id}," for msg in concrete_messages)}
}}

impl MessageType {{
    pub fn from_u8(value: u8) -> Result<Self> {{
        match value {{
{chr(10).join(f"            {msg.id} => Ok(MessageType::{msg.enum_name})," for msg in concrete_messages)}
            _ => Err(ProtocolError::InvalidMessageType(value)),
        }}
    }}
}}'''


def generate_structs(structs: List[tuple]) -> str:
    """Generate struct definitions for nested data"""
    struct_code = []
    
    for struct_name, fields in structs:
        struct_code.append(f'''#[derive(Debug, Clone, PartialEq)]
pub struct {struct_name} {{''')
        
        for field in fields:
            if field.comment:
                struct_code.append(f'    /// {field.comment}')
            struct_code.append(f'    pub {field.name}: {field.rust_type},')
        
        struct_code.append('}')
        struct_code.append('')
    
    return '\n'.join(struct_code)


def generate_message_structs(messages: List[RustMessage]) -> str:
    """Generate message struct definitions"""
    struct_code = []
    
    for msg in messages:
        if msg.alias:
            continue
            
        if msg.comment:
            comment_lines = msg.comment.strip().split('\n')
            for line in comment_lines:
                struct_code.append(f'/// {line.strip()}')
        
        struct_code.append(f'''#[derive(Debug, Clone, PartialEq)]
pub struct {msg.rust_name} {{''')
        
        for field in msg.fields:
            if field.comment:
                struct_code.append(f'    /// {field.comment}')
            struct_code.append(f'    pub {field.name}: {field.rust_type},')
        
        struct_code.append('}')
        struct_code.append('')
    
    return '\n'.join(struct_code)


def generate_message_enum(messages: List[RustMessage]) -> str:
    """Generate the main Message enum"""
    # Only include non-alias messages
    concrete_messages = [msg for msg in messages if not msg.alias]
    
    enum_code = ['''#[derive(Debug, Clone, PartialEq)]
pub enum Message {''']
    
    for msg in concrete_messages:
        if msg.comment:
            comment_lines = msg.comment.strip().split('\n')
            for line in comment_lines:
                enum_code.append(f'    /// {line.strip()}')
        enum_code.append(f'    {msg.rust_name}({msg.rust_name}),')
    
    enum_code.append('}')
    return '\n'.join(enum_code)


def generate_serialization_traits() -> str:
    """Generate serialization traits"""
    return '''pub trait BinaryWriter {
    fn write_u8(&mut self, value: u8) -> IoResult<()>;
    fn write_u16(&mut self, value: u16) -> IoResult<()>;
    fn write_u24(&mut self, value: u32) -> IoResult<()>;
    fn write_u32(&mut self, value: u32) -> IoResult<()>;
    fn write_i8(&mut self, value: i8) -> IoResult<()>;
    fn write_i16(&mut self, value: i16) -> IoResult<()>;
    fn write_i24(&mut self, value: i32) -> IoResult<()>;
    fn write_i32(&mut self, value: i32) -> IoResult<()>;
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()>;
}

pub trait BinaryReader {
    fn read_u8(&mut self) -> IoResult<u8>;
    fn read_u16(&mut self) -> IoResult<u16>;
    fn read_u24(&mut self) -> IoResult<u32>;
    fn read_u32(&mut self) -> IoResult<u32>;
    fn read_i8(&mut self) -> IoResult<i8>;
    fn read_i16(&mut self) -> IoResult<i16>;
    fn read_i24(&mut self) -> IoResult<i32>;
    fn read_i32(&mut self) -> IoResult<i32>;
    fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()>;
}

impl<W: Write> BinaryWriter for W {
    fn write_u8(&mut self, value: u8) -> IoResult<()> {
        self.write_all(&[value])
    }
    
    fn write_u16(&mut self, value: u16) -> IoResult<()> {
        self.write_all(&value.to_be_bytes())
    }
    
    fn write_u24(&mut self, value: u32) -> IoResult<()> {
        let bytes = value.to_be_bytes();
        self.write_all(&bytes[1..])
    }
    
    fn write_u32(&mut self, value: u32) -> IoResult<()> {
        self.write_all(&value.to_be_bytes())
    }
    
    fn write_i8(&mut self, value: i8) -> IoResult<()> {
        self.write_all(&[value as u8])
    }
    
    fn write_i16(&mut self, value: i16) -> IoResult<()> {
        self.write_all(&value.to_be_bytes())
    }
    
    fn write_i24(&mut self, value: i32) -> IoResult<()> {
        let bytes = value.to_be_bytes();
        self.write_all(&bytes[1..])
    }
    
    fn write_i32(&mut self, value: i32) -> IoResult<()> {
        self.write_all(&value.to_be_bytes())
    }
    
    fn write_all(&mut self, buf: &[u8]) -> IoResult<()> {
        Write::write_all(self, buf)
    }
}

impl<R: Read> BinaryReader for R {
    fn read_u8(&mut self) -> IoResult<u8> {
        let mut buf = [0u8; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }
    
    fn read_u16(&mut self) -> IoResult<u16> {
        let mut buf = [0u8; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }
    
    fn read_u24(&mut self) -> IoResult<u32> {
        let mut buf = [0u8; 3];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes([0, buf[0], buf[1], buf[2]]))
    }
    
    fn read_u32(&mut self) -> IoResult<u32> {
        let mut buf = [0u8; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }
    
    fn read_i8(&mut self) -> IoResult<i8> {
        Ok(self.read_u8()? as i8)
    }
    
    fn read_i16(&mut self) -> IoResult<i16> {
        Ok(self.read_u16()? as i16)
    }
    
    fn read_i24(&mut self) -> IoResult<i32> {
        let value = self.read_u24()? as i32;
        // Sign extend 24-bit to 32-bit
        if value & 0x800000 != 0 {
            Ok(value | 0xFF000000u32 as i32)
        } else {
            Ok(value)
        }
    }
    
    fn read_i32(&mut self) -> IoResult<i32> {
        Ok(self.read_u32()? as i32)
    }
    
    fn read_exact(&mut self, buf: &mut [u8]) -> IoResult<()> {
        Read::read_exact(self, buf)
    }
}

pub trait Serializable {
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()>;
    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> where Self: Sized;
    fn serialized_size(&self) -> usize;
}'''


def generate_message_implementations(messages: List[RustMessage]) -> str:
    """Generate Serializable implementations for all messages"""
    impl_code = []
    
    for msg in messages:
        if msg.alias:
            continue
            
        # Generate Serializable implementation for message struct
        if msg.fields:
            impl_code.append(f'''impl Serializable for {msg.rust_name} {{
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {{''')
            for field in msg.fields:
                lines = field.serialize_expr.strip().split('\n')
                for line in lines:
                    if line.strip():
                        impl_code.append(f'        {line.strip()}')
        else:
            impl_code.append(f'''impl Serializable for {msg.rust_name} {{
    fn serialize<W: BinaryWriter>(&self, _writer: &mut W) -> Result<()> {{''')
        
        impl_code.append('''        Ok(())
    }''')
        
        param_name = "reader" if msg.fields else "_reader"
        impl_code.append(f'''    
    fn deserialize<R: BinaryReader>({param_name}: &mut R) -> Result<Self> {{''')
        
        if msg.fields:
            for field in msg.fields:
                lines = field.deserialize_expr.strip().split('\n')
                for line in lines:
                    if line.strip():
                        impl_code.append(f'        {line.strip()}')
        
        impl_code.append(f'''        Ok({msg.rust_name} {{''')
        
        if msg.fields:
            for field in msg.fields:
                impl_code.append(f'            {field.name},')
        
        impl_code.append('''        })
    }
    
    fn serialized_size(&self) -> usize {''')
        
        if msg.fields:
            size_exprs = []
            for field in msg.fields:
                size_exprs.append(field.size_expr)
            impl_code.append(f'        {" + ".join(size_exprs)}')
        else:
            impl_code.append('        0')
        
        impl_code.append('''    }
}''')
        impl_code.append('')
    
    # Generate struct implementations if any (deduplicated)
    implemented_structs = set()
    for msg in messages:
        for struct_name, fields in msg.structs:
            if struct_name in implemented_structs:
                continue
            implemented_structs.add(struct_name)
            
            impl_code.append(f'''impl Serializable for {struct_name} {{
    fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {{''')
            
            for field in fields:
                lines = field.serialize_expr.strip().split('\n')
                for line in lines:
                    if line.strip():
                        impl_code.append(f'        {line.strip()}')
            
            impl_code.append('''        Ok(())
    }
    
    fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {''')
            
            for field in fields:
                lines = field.deserialize_expr.strip().split('\n')
                for line in lines:
                    if line.strip():
                        impl_code.append(f'        {line.strip()}')
            
            impl_code.append(f'''        Ok({struct_name} {{''')
            
            for field in fields:
                impl_code.append(f'            {field.name},')
            
            impl_code.append('''        })
    }
    
    fn serialized_size(&self) -> usize {''')
            
            if fields:
                size_exprs = []
                for field in fields:
                    size_exprs.append(field.size_expr)
                impl_code.append(f'        {" + ".join(size_exprs)}')
            else:
                impl_code.append('        0')
            
            impl_code.append('''    }
}''')
            impl_code.append('')
    
    # Generate Message enum implementation
    concrete_messages = [msg for msg in messages if not msg.alias]
    
    impl_code.append('''impl Message {
    pub fn message_type(&self) -> MessageType {
        match self {''')
    
    for msg in concrete_messages:
        impl_code.append(f'            Message::{msg.rust_name}(_) => MessageType::{msg.enum_name},')
    
    impl_code.append('''        }
    }
    
    pub fn serialize<W: BinaryWriter>(&self, writer: &mut W) -> Result<()> {
        // Write message type first
        writer.write_u8(self.message_type() as u8)?;
        
        // Write message payload
        match self {''')
    
    for msg in concrete_messages:
        impl_code.append(f'            Message::{msg.rust_name}(msg) => msg.serialize(writer),')
    
    impl_code.append('''        }
    }
    
    pub fn deserialize<R: BinaryReader>(reader: &mut R) -> Result<Self> {
        let msg_type = MessageType::from_u8(reader.read_u8()?)?;
        
        match msg_type {''')
    
    for msg in concrete_messages:
        impl_code.append(f'            MessageType::{msg.enum_name} => Ok(Message::{msg.rust_name}({msg.rust_name}::deserialize(reader)?)),')
    
    impl_code.append('''        }
    }
    
    pub fn serialized_size(&self) -> usize {
        1 + match self {''')
    
    for msg in concrete_messages:
        impl_code.append(f'            Message::{msg.rust_name}(msg) => msg.serialized_size(),')
    
    impl_code.append('''        }
    }
}''')
    
    return '\n'.join(impl_code)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"Usage: {sys.argv[0]} <protocol.yaml> <output.rs>")
        sys.exit(1)
    
    protocol_path = sys.argv[1]
    output_path = sys.argv[2]
    
    # Load protocol definition
    protocol = protogen.load_protocol_definition(protocol_path)
    
    # Generate Rust code
    rust_code = generate_rust_code(protocol)
    
    # Write to output file
    with open(output_path, 'w') as f:
        f.write(rust_code)
    
    print(f"Generated Rust code written to {output_path}")