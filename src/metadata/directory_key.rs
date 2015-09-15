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

/// DirectoryKey represnts the meta information about a directory
/// A directory can be feteched with the DirectoryKey
#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DirectoryKey {
    id          : ::routing::NameType,
    type_tag    : u64,
    versioned   : bool,
    access_level: ::AccessLevel,
}

impl DirectoryKey {
    /// Creates a new instance of DirectoryKey
    pub fn new(directory_id: ::routing::NameType, type_tag: u64, versioned: bool, access_level: ::AccessLevel) -> DirectoryKey {
        DirectoryKey {
            id          : directory_id,
            type_tag    : type_tag,
            versioned   : versioned,
            access_level: access_level
        }
    }

    /// Returns the id
    pub fn get_id(&self) -> &::routing::NameType {
        &self.id
    }
    /// Returns the type_tag
    pub fn get_type_tag(&self) -> u64 {
        self.type_tag
    }
    /// Returns true if the directory represented by the key is versioned, else returns false
    pub fn is_versioned(&self) -> bool {
        self.versioned
    }
    /// Returns the accesslevel of the directory represented by the key
    pub fn get_access_level(&self) -> &::AccessLevel {
        &self.access_level
    }

}
