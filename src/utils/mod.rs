// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use cbor;
use rustc_serialize::{Decodable, Encodable};

#[allow(unused_must_use)]
/// utility function to serialise an Encodable type
pub fn serialise<T>(data: T) -> Vec<u8> where T : Encodable {
    let mut e = cbor::Encoder::from_memory();
    e.encode(&[data]);
    e.into_bytes()
}

/// utility function to deserialise a Decodable type
pub fn deserialise_parser(data: Vec<u8>) -> ::maidsafe_client::data_parser::Parser {
    let mut d = cbor::Decoder::from_bytes(data);
    d.decode().next().unwrap().unwrap()
}

/// utility function to deserialise a Decodable type
pub fn deserialise<T>(data: Vec<u8>) -> T where T : Decodable {
    let mut d = cbor::Decoder::from_bytes(data);
    d.decode().next().unwrap().unwrap()
    // match d.decode().next().unwrap().unwrap() {
    //     ::data_parser::Parser::StructuredData(obj) => obj,
    //     ::data_parser::Parser::ImmutableData(obj) => obj,
    // }
}

pub mod test {
    #[allow(dead_code)]
    pub fn generate_random_string(length: usize) -> String {
        (0..length).map(|_| ::rand::random::<char>()).collect()
    }

    #[allow(dead_code)]
    pub fn generate_random_pin() -> u32 {
        ::rand::random::<u32>() % 10000
    }
}
