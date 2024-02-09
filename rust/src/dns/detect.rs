/* Copyright (C) 2019 Open Information Security Foundation
 *
 * You can copy, redistribute or modify this Program under the terms of
 * the GNU General Public License version 2 as published by the Free
 * Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA.
 */

use super::dns::DNSTransaction;
use crate::core::Direction;
use crate::detect::uint::{detect_match_uint, DetectUintData};

/// Perform the DNS opcode match.
///
/// 1 will be returned on match, otherwise 0 will be returned.
#[no_mangle]
pub extern "C" fn rs_dns_opcode_match(
    tx: &mut DNSTransaction, detect: &mut DetectUintData<u8>, flags: u8,
) -> u8 {
    let header_flags = if flags & Direction::ToServer as u8 != 0 {
        if let Some(request) = &tx.request {
            request.header.flags
        } else {
            return 0;
        }
    } else if flags & Direction::ToClient as u8 != 0 {
        if let Some(response) = &tx.response {
            response.header.flags
        } else {
            return 0;
        }
    } else {
        // Not to server or to client??
        return 0;
    };
    let opcode = ((header_flags >> 11) & 0xf) as u8;

    if detect_match_uint(detect, opcode) {
        return 1;
    }
    return 0;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::detect::uint::{detect_parse_uint, DetectUintMode};

    #[test]
    fn parse_opcode_good() {
        assert_eq!(
            detect_parse_uint::<u8>("1").unwrap().1,
            DetectUintData {
                mode: DetectUintMode::DetectUintModeEqual,
                arg1: 1,
                arg2: 0,
            }
        );
        assert_eq!(
            detect_parse_uint::<u8>("123").unwrap().1,
            DetectUintData {
                mode: DetectUintMode::DetectUintModeEqual,
                arg1: 123,
                arg2: 0,
            }
        );
        assert_eq!(
            detect_parse_uint::<u8>("!123").unwrap().1,
            DetectUintData {
                mode: DetectUintMode::DetectUintModeNe,
                arg1: 123,
                arg2: 0,
            }
        );
        assert!(detect_parse_uint::<u8>("").is_err());
        assert!(detect_parse_uint::<u8>("!").is_err());
        assert!(detect_parse_uint::<u8>("!   ").is_err());
        assert!(detect_parse_uint::<u8>("!asdf").is_err());
    }

    #[test]
    fn test_match_opcode() {
        assert!(detect_match_uint(
            &DetectUintData {
                mode: DetectUintMode::DetectUintModeEqual,
                arg1: 0,
                arg2: 0,
            },
            0b0000_0000_0000_0000,
        ));

        assert!(!detect_match_uint(
            &DetectUintData {
                mode: DetectUintMode::DetectUintModeNe,
                arg1: 0,
                arg2: 0,
            },
            0b0000_0000_0000_0000,
        ));

        assert!(detect_match_uint(
            &DetectUintData {
                mode: DetectUintMode::DetectUintModeEqual,
                arg1: 4,
                arg2: 0,
            },
            ((0b0010_0000_0000_0000 >> 11) & 0xf) as u8,
        ));

        assert!(!detect_match_uint(
            &DetectUintData {
                mode: DetectUintMode::DetectUintModeNe,
                arg1: 4,
                arg2: 0,
            },
            ((0b0010_0000_0000_0000 >> 11) & 0xf) as u8,
        ));
    }
}
