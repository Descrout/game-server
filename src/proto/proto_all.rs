// Automatically generated rust module for 'proto-all.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Error {
    pub title: String,
    pub message: String,
}

impl<'a> MessageRead<'a> for Error {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.title = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.message = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Error {
    fn get_size(&self) -> usize {
        0
        + if self.title == String::default() { 0 } else { 1 + sizeof_len((&self.title).len()) }
        + if self.message == String::default() { 0 } else { 1 + sizeof_len((&self.message).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.title != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.title))?; }
        if self.message != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.message))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
}

impl<'a> MessageRead<'a> for User {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for User {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.id))?; }
        if self.name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Room {
    pub id: u32,
    pub name: String,
    pub password: bool,
    pub players: u32,
}

impl<'a> MessageRead<'a> for Room {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(24) => msg.password = r.read_bool(bytes)?,
                Ok(32) => msg.players = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Room {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.password == false { 0 } else { 1 + sizeof_varint(*(&self.password) as u64) }
        + if self.players == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.players) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.id))?; }
        if self.name != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.name))?; }
        if self.password != false { w.write_with_tag(24, |w| w.write_bool(*&self.password))?; }
        if self.players != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.players))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Handshake {
    pub name: String,
}

impl<'a> MessageRead<'a> for Handshake {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Handshake {
    fn get_size(&self) -> usize {
        0
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Rooms {
    pub rooms: Vec<Room>,
}

impl<'a> MessageRead<'a> for Rooms {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.rooms.push(r.read_message::<Room>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Rooms {
    fn get_size(&self) -> usize {
        0
        + self.rooms.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.rooms { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Users {
    pub users: Vec<User>,
    pub me: u32,
}

impl<'a> MessageRead<'a> for Users {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.users.push(r.read_message::<User>(bytes)?),
                Ok(16) => msg.me = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Users {
    fn get_size(&self) -> usize {
        0
        + self.users.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + if self.me == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.me) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.users { w.write_with_tag(10, |w| w.write_message(s))?; }
        if self.me != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.me))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct CreateRoom {
    pub name: String,
    pub password: String,
}

impl<'a> MessageRead<'a> for CreateRoom {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.password = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for CreateRoom {
    fn get_size(&self) -> usize {
        0
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.password == String::default() { 0 } else { 1 + sizeof_len((&self.password).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if self.password != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.password))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct JoinRoom {
    pub id: u32,
    pub password: String,
}

impl<'a> MessageRead<'a> for JoinRoom {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(18) => msg.password = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for JoinRoom {
    fn get_size(&self) -> usize {
        0
        + if self.id == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.id) as u64) }
        + if self.password == String::default() { 0 } else { 1 + sizeof_len((&self.password).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.id != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.id))?; }
        if self.password != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.password))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Chat {
    pub name: String,
    pub message: String,
}

impl<'a> MessageRead<'a> for Chat {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.message = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Chat {
    fn get_size(&self) -> usize {
        0
        + if self.name == String::default() { 0 } else { 1 + sizeof_len((&self.name).len()) }
        + if self.message == String::default() { 0 } else { 1 + sizeof_len((&self.message).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.name != String::default() { w.write_with_tag(10, |w| w.write_string(&**&self.name))?; }
        if self.message != String::default() { w.write_with_tag(18, |w| w.write_string(&**&self.message))?; }
        Ok(())
    }
}

