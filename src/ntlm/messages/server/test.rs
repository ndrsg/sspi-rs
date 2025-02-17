use super::*;
use crate::ntlm::messages::test::*;
use crate::ntlm::*;

const LOCAL_AUTHENTICATE_MESSAGE: [u8; 312] = [
    0x4e, 0x54, 0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x03, 0x00, 0x00, 0x00, 0x18, 0x00, 0x18, 0x00, 0x5a, 0x00, 0x00,
    0x00, 0xb6, 0x00, 0xb6, 0x00, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x58, 0x00, 0x00, 0x00, 0x02, 0x00,
    0x02, 0x00, 0x58, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5a, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10, 0x00, 0x28,
    0x01, 0x00, 0x00, 0x35, 0x82, 0x88, 0xe2, 0x06, 0x01, 0xb1, 0x1d, 0x00, 0x00, 0x00, 0x0f, 0xec, 0x30, 0xba, 0x0f,
    0x97, 0xba, 0xbd, 0x1e, 0xa6, 0x21, 0x16, 0x5e, 0x92, 0x51, 0xfc, 0xe7, 0x61, 0x00, 0x7f, 0x04, 0x90, 0xe4, 0xe8,
    0xd6, 0xb6, 0x64, 0xbd, 0xaf, 0x3c, 0x47, 0x26, 0x2f, 0xbe, 0x4c, 0xe5, 0x19, 0x6d, 0x0d, 0xd0, 0x4b, 0x3b, 0x90,
    0xde, 0x07, 0x90, 0x05, 0x66, 0x87, 0x79, 0x7b, 0xd4, 0x66, 0xde, 0xc2, 0xf5, 0xc2, 0x70, 0x88, 0x01, 0x01, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0xb4, 0x45, 0xc6, 0x13, 0xd0, 0xd4, 0x01, 0xe5, 0x19, 0x6d, 0x0d, 0xd0, 0x4b,
    0x3b, 0x90, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x01,
    0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x04, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00,
    0x50, 0x00, 0x43, 0x00, 0x03, 0x00, 0x08, 0x00, 0x56, 0x00, 0x47, 0x00, 0x50, 0x00, 0x43, 0x00, 0x07, 0x00, 0x08,
    0x00, 0x80, 0xb4, 0x45, 0xc6, 0x13, 0xd0, 0xd4, 0x01, 0x06, 0x00, 0x04, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0a, 0x00,
    0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09,
    0x00, 0x1e, 0x00, 0x54, 0x00, 0x45, 0x00, 0x52, 0x00, 0x4d, 0x00, 0x53, 0x00, 0x52, 0x00, 0x56, 0x00, 0x2f, 0x00,
    0x30, 0x00, 0x2e, 0x00, 0x30, 0x00, 0x2e, 0x00, 0x30, 0x00, 0x2e, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0x1d, 0xce, 0x10, 0xc5, 0xd1, 0x95, 0x01,
    0x3f, 0xb1, 0x8c, 0x26, 0x70, 0x9f, 0x50, 0x8b,
];
const LOCAL_MIC: [u8; 16] = [
    0xec, 0x30, 0xba, 0x0f, 0x97, 0xba, 0xbd, 0x1e, 0xa6, 0x21, 0x16, 0x5e, 0x92, 0x51, 0xfc, 0xe7,
];
const LOCAL_ENCRYPTED_RANDOM_SESSION_KEY: [u8; 16] = [
    0xc2, 0x1d, 0xce, 0x10, 0xc5, 0xd1, 0x95, 0x1, 0x3f, 0xb1, 0x8c, 0x26, 0x70, 0x9f, 0x50, 0x8b,
];
const LOCAL_USER_NAME: [u8; 2] = [0x61, 0x00];

const DOMAIN_AUTHENTICATE_MESSAGE: [u8; 570] = [
    0x4e, 0x54, 0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x03, 0x00, 0x00, 0x00, 0x18, 0x00, 0x18, 0x00, 0x98, 0x00, 0x00,
    0x00, 0x7a, 0x01, 0x7a, 0x01, 0xb0, 0x00, 0x00, 0x00, 0x16, 0x00, 0x16, 0x00, 0x58, 0x00, 0x00, 0x00, 0x1a, 0x00,
    0x1a, 0x00, 0x6e, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10, 0x00, 0x88, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10, 0x00, 0x2a,
    0x02, 0x00, 0x00, 0x35, 0x82, 0x88, 0xe2, 0x06, 0x01, 0xb0, 0x1d, 0x00, 0x00, 0x00, 0x0f, 0x12, 0x28, 0x00, 0xa0,
    0xb2, 0x29, 0x47, 0x12, 0x1e, 0x8e, 0x54, 0xf8, 0x29, 0xdb, 0x52, 0x1e, 0x41, 0x00, 0x57, 0x00, 0x41, 0x00, 0x4b,
    0x00, 0x45, 0x00, 0x43, 0x00, 0x4f, 0x00, 0x44, 0x00, 0x49, 0x00, 0x4e, 0x00, 0x47, 0x00, 0x41, 0x00, 0x64, 0x00,
    0x6d, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x69, 0x00, 0x73, 0x00, 0x74, 0x00, 0x72, 0x00, 0x61, 0x00, 0x74, 0x00, 0x6f,
    0x00, 0x72, 0x00, 0x57, 0x00, 0x49, 0x00, 0x4e, 0x00, 0x44, 0x00, 0x4f, 0x00, 0x57, 0x00, 0x53, 0x00, 0x37, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0xf5, 0x61, 0x4e, 0x2f, 0x00, 0xd0, 0x15, 0xb0, 0x70, 0xb0, 0x3e, 0x82, 0x91, 0x5f,
    0xc7, 0x08, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0xfd, 0xae, 0x48, 0x07, 0xcb, 0xcb, 0x01, 0xa5,
    0x00, 0x28, 0x29, 0xcd, 0x07, 0xe3, 0xbc, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x16, 0x00, 0x41, 0x00, 0x57, 0x00,
    0x41, 0x00, 0x4b, 0x00, 0x45, 0x00, 0x43, 0x00, 0x4f, 0x00, 0x44, 0x00, 0x49, 0x00, 0x4e, 0x00, 0x47, 0x00, 0x01,
    0x00, 0x10, 0x00, 0x57, 0x00, 0x49, 0x00, 0x4e, 0x00, 0x32, 0x00, 0x4b, 0x00, 0x38, 0x00, 0x52, 0x00, 0x32, 0x00,
    0x04, 0x00, 0x24, 0x00, 0x61, 0x00, 0x77, 0x00, 0x61, 0x00, 0x6b, 0x00, 0x65, 0x00, 0x63, 0x00, 0x6f, 0x00, 0x64,
    0x00, 0x69, 0x00, 0x6e, 0x00, 0x67, 0x00, 0x2e, 0x00, 0x61, 0x00, 0x74, 0x00, 0x68, 0x00, 0x2e, 0x00, 0x63, 0x00,
    0x78, 0x00, 0x03, 0x00, 0x36, 0x00, 0x57, 0x00, 0x49, 0x00, 0x4e, 0x00, 0x32, 0x00, 0x4b, 0x00, 0x38, 0x00, 0x52,
    0x00, 0x32, 0x00, 0x2e, 0x00, 0x61, 0x00, 0x77, 0x00, 0x61, 0x00, 0x6b, 0x00, 0x65, 0x00, 0x63, 0x00, 0x6f, 0x00,
    0x64, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x67, 0x00, 0x2e, 0x00, 0x61, 0x00, 0x74, 0x00, 0x68, 0x00, 0x2e, 0x00, 0x63,
    0x00, 0x78, 0x00, 0x05, 0x00, 0x24, 0x00, 0x61, 0x00, 0x77, 0x00, 0x61, 0x00, 0x6b, 0x00, 0x65, 0x00, 0x63, 0x00,
    0x6f, 0x00, 0x64, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x67, 0x00, 0x2e, 0x00, 0x61, 0x00, 0x74, 0x00, 0x68, 0x00, 0x2e,
    0x00, 0x63, 0x00, 0x78, 0x00, 0x07, 0x00, 0x08, 0x00, 0x20, 0xfd, 0xae, 0x48, 0x07, 0xcb, 0xcb, 0x01, 0x06, 0x00,
    0x04, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x30, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x7b, 0xd0, 0x9e, 0x33, 0x06, 0x75, 0xe3, 0x3e, 0x52, 0x7b, 0x4a, 0xc4,
    0x75, 0x5f, 0x9b, 0x98, 0x26, 0x5d, 0xcb, 0x05, 0x6a, 0x6a, 0xcc, 0x0f, 0xb8, 0x4f, 0xab, 0x09, 0x22, 0x30, 0x7a,
    0x5d, 0x0a, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x09, 0x00, 0x2a, 0x00, 0x54, 0x00, 0x45, 0x00, 0x52, 0x00, 0x4d, 0x00, 0x53, 0x00, 0x52, 0x00, 0x56,
    0x00, 0x2f, 0x00, 0x31, 0x00, 0x39, 0x00, 0x32, 0x00, 0x2e, 0x00, 0x31, 0x00, 0x36, 0x00, 0x38, 0x00, 0x2e, 0x00,
    0x31, 0x00, 0x2e, 0x00, 0x31, 0x00, 0x35, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x0c, 0x57, 0xc6, 0xb5, 0x0c, 0x14, 0xc1, 0xf0, 0x64, 0xe7, 0xcc, 0x8b, 0xf0, 0x6d, 0x7a, 0x13,
];
const DOMAIN_MIC: [u8; 16] = [
    0x12, 0x28, 0x00, 0xa0, 0xb2, 0x29, 0x47, 0x12, 0x1e, 0x8e, 0x54, 0xf8, 0x29, 0xdb, 0x52, 0x1e,
];
const DOMAIN_USER_NAME: [u8; 26] = [
    0x41, 0x00, 0x64, 0x00, 0x6d, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x69, 0x00, 0x73, 0x00, 0x74, 0x00, 0x72, 0x00, 0x61,
    0x00, 0x74, 0x00, 0x6f, 0x00, 0x72, 0x00,
];
const DOMAIN_DOMAIN_NAME: [u8; 22] = [
    0x41, 0x00, 0x57, 0x00, 0x41, 0x00, 0x4b, 0x00, 0x45, 0x00, 0x43, 0x00, 0x4f, 0x00, 0x44, 0x00, 0x49, 0x00, 0x4e,
    0x00, 0x47, 0x00,
];
const DOMAIN_TARGET_INFO: [u8; 334] = [
    0x02, 0x00, 0x16, 0x00, 0x41, 0x00, 0x57, 0x00, 0x41, 0x00, 0x4b, 0x00, 0x45, 0x00, 0x43, 0x00, 0x4f, 0x00, 0x44,
    0x00, 0x49, 0x00, 0x4e, 0x00, 0x47, 0x00, 0x01, 0x00, 0x10, 0x00, 0x57, 0x00, 0x49, 0x00, 0x4e, 0x00, 0x32, 0x00,
    0x4b, 0x00, 0x38, 0x00, 0x52, 0x00, 0x32, 0x00, 0x04, 0x00, 0x24, 0x00, 0x61, 0x00, 0x77, 0x00, 0x61, 0x00, 0x6b,
    0x00, 0x65, 0x00, 0x63, 0x00, 0x6f, 0x00, 0x64, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x67, 0x00, 0x2e, 0x00, 0x61, 0x00,
    0x74, 0x00, 0x68, 0x00, 0x2e, 0x00, 0x63, 0x00, 0x78, 0x00, 0x03, 0x00, 0x36, 0x00, 0x57, 0x00, 0x49, 0x00, 0x4e,
    0x00, 0x32, 0x00, 0x4b, 0x00, 0x38, 0x00, 0x52, 0x00, 0x32, 0x00, 0x2e, 0x00, 0x61, 0x00, 0x77, 0x00, 0x61, 0x00,
    0x6b, 0x00, 0x65, 0x00, 0x63, 0x00, 0x6f, 0x00, 0x64, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x67, 0x00, 0x2e, 0x00, 0x61,
    0x00, 0x74, 0x00, 0x68, 0x00, 0x2e, 0x00, 0x63, 0x00, 0x78, 0x00, 0x05, 0x00, 0x24, 0x00, 0x61, 0x00, 0x77, 0x00,
    0x61, 0x00, 0x6b, 0x00, 0x65, 0x00, 0x63, 0x00, 0x6f, 0x00, 0x64, 0x00, 0x69, 0x00, 0x6e, 0x00, 0x67, 0x00, 0x2e,
    0x00, 0x61, 0x00, 0x74, 0x00, 0x68, 0x00, 0x2e, 0x00, 0x63, 0x00, 0x78, 0x00, 0x07, 0x00, 0x08, 0x00, 0x20, 0xfd,
    0xae, 0x48, 0x07, 0xcb, 0xcb, 0x01, 0x06, 0x00, 0x04, 0x00, 0x02, 0x00, 0x00, 0x00, 0x08, 0x00, 0x30, 0x00, 0x30,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x7b, 0xd0, 0x9e, 0x33,
    0x06, 0x75, 0xe3, 0x3e, 0x52, 0x7b, 0x4a, 0xc4, 0x75, 0x5f, 0x9b, 0x98, 0x26, 0x5d, 0xcb, 0x05, 0x6a, 0x6a, 0xcc,
    0x0f, 0xb8, 0x4f, 0xab, 0x09, 0x22, 0x30, 0x7a, 0x5d, 0x0a, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x2a, 0x00, 0x54, 0x00, 0x45, 0x00, 0x52,
    0x00, 0x4d, 0x00, 0x53, 0x00, 0x52, 0x00, 0x56, 0x00, 0x2f, 0x00, 0x31, 0x00, 0x39, 0x00, 0x32, 0x00, 0x2e, 0x00,
    0x31, 0x00, 0x36, 0x00, 0x38, 0x00, 0x2e, 0x00, 0x31, 0x00, 0x2e, 0x00, 0x31, 0x00, 0x35, 0x00, 0x30, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const DOMAIN_CLIENT_CHALLENGE: [u8; 8] = [0xa5, 0x00, 0x28, 0x29, 0xcd, 0x07, 0xe3, 0xbc];

const DOMAIN_ENCRYPTED_SESSION_KEY: [u8; 16] = [
    0x0c, 0x57, 0xc6, 0xb5, 0x0c, 0x14, 0xc1, 0xf0, 0x64, 0xe7, 0xcc, 0x8b, 0xf0, 0x6d, 0x7a, 0x13,
];

#[test]
fn read_negotiate_does_not_fail_with_correct_header() {
    let mut context = Ntlm::new();
    context.state = NtlmState::Negotiate;

    let buff = *LOCAL_NEGOTIATE_MESSAGE;
    read_negotiate(&mut context, buff.as_ref()).unwrap();
}

#[test]
fn read_negotiate_fails_with_incorrect_signature() {
    let mut context = Ntlm::new();
    context.state = NtlmState::Negotiate;

    let mut buff = LOCAL_NEGOTIATE_MESSAGE.to_vec();
    buff[1] += 1;
    assert!(read_negotiate(&mut context, buff.as_slice()).is_err());
}

#[test]
fn read_negotiate_fails_with_incorrect_message_type() {
    let mut context = Ntlm::new();
    context.state = NtlmState::Negotiate;

    let mut buff = LOCAL_NEGOTIATE_MESSAGE.to_vec();
    buff[8] = 2;
    assert!(read_negotiate(&mut context, buff.as_slice()).is_err());
}

#[test]
fn read_negotiate_reads_correct_flags() {
    let mut context = Ntlm::new();

    context.state = NtlmState::Negotiate;

    let expected_flags = NegotiateFlags::from_bits(LOCAL_NEGOTIATE_FLAGS).unwrap();

    let buff = *LOCAL_NEGOTIATE_MESSAGE;
    read_negotiate(&mut context, buff.as_ref()).unwrap();

    assert_eq!(expected_flags, context.flags);
}

#[test]
fn read_negotiate_fails_without_needed_flags() {
    let mut context = Ntlm::new();

    context.state = NtlmState::Negotiate;

    let mut buff = *LOCAL_NEGOTIATE_MESSAGE;
    buff[NEGOTIATE_FLAGS_START..NEGOTIATE_FLAGS_START + NEGOTIATE_FLAGS_SIZE]
        .clone_from_slice([0x00; NEGOTIATE_FLAGS_SIZE].as_ref());
    assert!(read_negotiate(&mut context, buff.as_ref()).is_err());
}

#[test]
fn read_negotiate_writes_buffer_to_context() {
    let mut context = Ntlm::new();

    context.state = NtlmState::Negotiate;

    let buff = *LOCAL_NEGOTIATE_MESSAGE;
    read_negotiate(&mut context, buff.as_ref()).unwrap();

    assert_eq!(
        (*LOCAL_NEGOTIATE_MESSAGE).as_ref(),
        context.negotiate_message.unwrap().message.as_slice()
    );
}

#[test]
fn read_negotiate_changes_context_state_on_success() {
    let mut context = Ntlm::new();

    context.state = NtlmState::Negotiate;

    let expected_state = NtlmState::Challenge;

    let buff = *LOCAL_NEGOTIATE_MESSAGE;
    read_negotiate(&mut context, buff.as_ref()).unwrap();

    assert_eq!(expected_state, context.state);
}

#[test]
fn read_negotiate_fails_on_incorrect_state() {
    let mut context = Ntlm::new();

    context.state = NtlmState::Challenge;

    let buff = *LOCAL_NEGOTIATE_MESSAGE;
    assert!(read_negotiate(&mut context, buff.as_ref()).is_err());
}

#[test]
fn write_challenge_writes_correct_signature() {
    let mut context = Ntlm::new();
    context.set_version(LOCAL_CHALLENGE_VERSION);
    context.state = NtlmState::Challenge;
    context.negotiate_message = Some(NegotiateMessage::new(LOCAL_NEGOTIATE_MESSAGE.to_vec()));
    context.flags = NegotiateFlags::from_bits(LOCAL_NEGOTIATE_FLAGS).unwrap();

    let mut buff = Vec::new();
    write_challenge(&mut context, &mut buff).unwrap();

    assert_eq!(NTLM_SIGNATURE, buff[SIGNATURE_START..MESSAGE_TYPE_START]);
}

#[test]
fn write_challenge_writes_correct_message_type() {
    let mut context = Ntlm::new();
    context.set_version(LOCAL_CHALLENGE_VERSION);
    context.state = NtlmState::Challenge;
    context.negotiate_message = Some(NegotiateMessage::new(LOCAL_NEGOTIATE_MESSAGE.to_vec()));
    context.flags = NegotiateFlags::from_bits(LOCAL_NEGOTIATE_FLAGS).unwrap();

    let mut buff = Vec::new();
    write_challenge(&mut context, &mut buff).unwrap();

    assert_eq!(
        CHALLENGE_MESSAGE_TYPE,
        buff[MESSAGE_TYPE_START..CHALLENGE_TARGET_NAME_START]
    );
}

#[test]
fn write_challenge_writes_correct_target_name() {
    let mut context = Ntlm::new();
    context.set_version(LOCAL_CHALLENGE_VERSION);
    context.state = NtlmState::Challenge;
    context.negotiate_message = Some(NegotiateMessage::new(LOCAL_NEGOTIATE_MESSAGE.to_vec()));
    context.flags = NegotiateFlags::from_bits(LOCAL_NEGOTIATE_FLAGS).unwrap();

    let mut buff = Vec::new();
    write_challenge(&mut context, &mut buff).unwrap();

    assert_eq!(
        LOCAL_CHALLENGE_TARGET_NAME_EMPTY,
        buff[CHALLENGE_TARGET_NAME_START..CHALLENGE_FLAGS_START]
    );
}

#[test]
fn write_challenge_writes_correct_flags() {
    let mut context = Ntlm::new();
    context.set_version(LOCAL_CHALLENGE_VERSION);
    context.state = NtlmState::Challenge;
    context.negotiate_message = Some(NegotiateMessage::new(LOCAL_NEGOTIATE_MESSAGE.to_vec()));
    context.flags = NegotiateFlags::from_bits(LOCAL_NEGOTIATE_FLAGS).unwrap();

    let mut buff = Vec::new();
    write_challenge(&mut context, &mut buff).unwrap();

    assert_eq!(
        LOCAL_CHALLENGE_FLAGS.to_le_bytes(),
        buff[CHALLENGE_FLAGS_START..CHALLENGE_SERVER_CHALLENGE_START]
    );

    assert_eq!(NegotiateFlags::from_bits(LOCAL_CHALLENGE_FLAGS).unwrap(), context.flags);
}

#[test]
fn write_challenge_writes_server_challenge() {
    let mut context = Ntlm::new();
    context.set_version(LOCAL_CHALLENGE_VERSION);
    context.state = NtlmState::Challenge;
    context.negotiate_message = Some(NegotiateMessage::new(LOCAL_NEGOTIATE_MESSAGE.to_vec()));
    context.flags = NegotiateFlags::from_bits(LOCAL_NEGOTIATE_FLAGS).unwrap();

    let mut buff = Vec::new();
    write_challenge(&mut context, &mut buff).unwrap();

    assert_ne!(
        [0x00; CHALLENGE_SIZE],
        context.challenge_message.as_ref().unwrap().server_challenge
    );
    assert_eq!(
        context.challenge_message.unwrap().server_challenge,
        buff[CHALLENGE_SERVER_CHALLENGE_START..CHALLENGE_RESERVED_START]
    );
}

#[test]
fn write_challenge_writes_target_info() {
    let mut context = Ntlm::new();
    context.set_version(LOCAL_CHALLENGE_VERSION);
    context.state = NtlmState::Challenge;
    context.negotiate_message = Some(NegotiateMessage::new(LOCAL_NEGOTIATE_MESSAGE.to_vec()));
    context.flags = NegotiateFlags::from_bits(LOCAL_NEGOTIATE_FLAGS).unwrap();

    let mut buff = Vec::new();
    write_challenge(&mut context, &mut buff).unwrap();

    assert!(!context.challenge_message.as_ref().unwrap().target_info.is_empty());
    assert_eq!(
        context.challenge_message.as_ref().unwrap().target_info,
        buff[CHALLENGE_HEADER_SIZE..].as_ref()
    );
}

#[test]
fn write_challenge_writes_correct_version() {
    let mut context = Ntlm::new();
    context.set_version(LOCAL_CHALLENGE_VERSION);
    context.state = NtlmState::Challenge;
    context.negotiate_message = Some(NegotiateMessage::new(LOCAL_NEGOTIATE_MESSAGE.to_vec()));
    context.flags = NegotiateFlags::from_bits(LOCAL_NEGOTIATE_FLAGS).unwrap();

    let mut buff = Vec::new();
    write_challenge(&mut context, &mut buff).unwrap();

    assert_eq!(
        LOCAL_CHALLENGE_VERSION,
        buff[CHALLENGE_VERSION_START..CHALLENGE_HEADER_SIZE]
    );
}

#[test]
fn write_challenge_writes_timestamp() {
    let mut context = Ntlm::new();
    context.set_version(LOCAL_CHALLENGE_VERSION);
    context.state = NtlmState::Challenge;
    context.negotiate_message = Some(NegotiateMessage::new(LOCAL_NEGOTIATE_MESSAGE.to_vec()));
    context.flags = NegotiateFlags::from_bits(LOCAL_NEGOTIATE_FLAGS).unwrap();

    let mut buff = Vec::new();
    write_challenge(&mut context, &mut buff).unwrap();

    assert_ne!(0, context.challenge_message.unwrap().timestamp);
}

#[test]
fn write_challenge_fails_on_incorrect_state() {
    let mut context = Ntlm::new();
    context.set_version(LOCAL_CHALLENGE_VERSION);
    context.state = NtlmState::Authenticate;
    context.negotiate_message = Some(NegotiateMessage::new(LOCAL_NEGOTIATE_MESSAGE.to_vec()));
    context.flags = NegotiateFlags::from_bits(LOCAL_NEGOTIATE_FLAGS).unwrap();

    let mut buff = Vec::new();
    assert!(write_challenge(&mut context, &mut buff).is_err());
}

#[test]
fn read_authenticate_fail_with_incorrect_signature() {
    let mut context = Ntlm::new();

    context.negotiate_message = Some(NegotiateMessage::new(Vec::new()));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.state = NtlmState::Authenticate;

    let mut buffer = LOCAL_AUTHENTICATE_MESSAGE.to_vec();
    buffer[3] = 0x00;
    assert!(read_authenticate(&mut context, buffer.as_slice()).is_err());
}

#[test]
fn read_authenticate_fail_with_incorrect_message_type() {
    let mut context = Ntlm::new();

    context.negotiate_message = Some(NegotiateMessage::new(Vec::new()));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.state = NtlmState::Authenticate;

    let mut buffer = LOCAL_AUTHENTICATE_MESSAGE.to_vec();
    buffer[8] = 0x02;
    assert!(read_authenticate(&mut context, buffer.as_slice()).is_err());
}

#[test]
fn read_authenticate_local_logon_correct_reads_mic() {
    let buffer = LOCAL_AUTHENTICATE_MESSAGE;
    let expected = LOCAL_MIC;
    let mut context = Ntlm::new();

    context.negotiate_message = Some(NegotiateMessage::new(Vec::new()));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.state = NtlmState::Authenticate;

    authenticate::read_authenticate(&mut context, buffer.as_ref()).unwrap();

    assert_eq!(
        expected.as_ref(),
        context.authenticate_message.unwrap().mic.unwrap().value.as_ref()
    );
}

#[test]
fn read_authenticate_local_logon_correct_reads_encrypted_random_session_key() {
    let buffer = LOCAL_AUTHENTICATE_MESSAGE;
    let expected = LOCAL_ENCRYPTED_RANDOM_SESSION_KEY;
    let mut context = Ntlm::new();

    context.negotiate_message = Some(NegotiateMessage::new(Vec::new()));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.state = NtlmState::Authenticate;

    authenticate::read_authenticate(&mut context, buffer.as_ref()).unwrap();

    assert_eq!(
        expected.as_ref(),
        context.authenticate_message.unwrap().encrypted_random_session_key,
    );
}

#[test]
fn read_authenticate_local_logon_correct_reads_user_name() {
    let buffer = LOCAL_AUTHENTICATE_MESSAGE;
    let expected = LOCAL_USER_NAME;
    let mut context = Ntlm::new();
    context.negotiate_message = Some(NegotiateMessage::new(Vec::new()));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.state = NtlmState::Authenticate;

    authenticate::read_authenticate(&mut context, buffer.as_ref()).unwrap();

    assert_eq!(expected.as_ref(), context.identity.as_ref().unwrap().user.as_slice());
}

#[test]
fn read_authenticate_domain_logon_correct_reads_mic() {
    let buffer = DOMAIN_AUTHENTICATE_MESSAGE;
    let expected = DOMAIN_MIC;
    let mut context = Ntlm::new();
    context.negotiate_message = Some(NegotiateMessage::new(Vec::new()));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.state = NtlmState::Authenticate;

    authenticate::read_authenticate(&mut context, buffer.as_ref()).unwrap();

    assert_eq!(
        expected.as_ref(),
        context.authenticate_message.unwrap().mic.unwrap().value.as_ref()
    );
}

#[test]
fn read_authenticate_domain_logon_correct_reads_user_name() {
    let buffer = DOMAIN_AUTHENTICATE_MESSAGE;
    let expected = DOMAIN_USER_NAME;
    let mut context = Ntlm::new();
    context.negotiate_message = Some(NegotiateMessage::new(Vec::new()));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.state = NtlmState::Authenticate;

    authenticate::read_authenticate(&mut context, buffer.as_ref()).unwrap();

    assert_eq!(expected.as_ref(), context.identity.as_ref().unwrap().user.as_slice());
}

#[test]
fn read_authenticate_domain_logon_correct_reads_domain_name() {
    let buffer = DOMAIN_AUTHENTICATE_MESSAGE;
    let expected = DOMAIN_DOMAIN_NAME;
    let mut context = Ntlm::new();
    context.negotiate_message = Some(NegotiateMessage::new(Vec::new()));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.state = NtlmState::Authenticate;

    authenticate::read_authenticate(&mut context, buffer.as_ref()).unwrap();

    assert_eq!(expected.as_ref(), context.identity.as_ref().unwrap().domain.as_slice());
}

#[test]
fn read_authenticate_fails_on_incorrect_state() {
    let buffer = DOMAIN_AUTHENTICATE_MESSAGE;
    let mut context = Ntlm::new();

    context.negotiate_message = Some(NegotiateMessage::new(Vec::new()));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.state = NtlmState::Completion;

    assert!(read_authenticate(&mut context, buffer.as_ref()).is_err());
}

#[test]
fn read_authenticate_fails_with_incorrect_encrypted_key_size() {
    let mut context = Ntlm::new();

    context.negotiate_message = Some(NegotiateMessage::new(Vec::new()));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.state = NtlmState::Authenticate;

    let mut buffer = DOMAIN_AUTHENTICATE_MESSAGE.to_vec();
    buffer[AUTHENTICATE_ENCRYPTED_KEY_START..AUTHENTICATE_ENCRYPTED_KEY_START + 4]
        .clone_from_slice([SESSION_KEY_SIZE as u8 - 1, 0x00, SESSION_KEY_SIZE as u8 - 1, 0x00].as_ref());
    buffer.pop();

    assert!(read_authenticate(&mut context, buffer.as_slice()).is_err());
}

#[test]
fn read_authenticate_fails_without_key_exchange_flag() {
    let mut context = Ntlm::new();

    context.negotiate_message = Some(NegotiateMessage::new(Vec::new()));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.state = NtlmState::Authenticate;

    let mut buffer = DOMAIN_AUTHENTICATE_MESSAGE.to_vec();
    buffer[AUTHENTICATE_FLAGS_START..AUTHENTICATE_FLAGS_START + NEGOTIATE_FLAGS_SIZE]
        .clone_from_slice([0x35, 0xb2, 0x88, 0xa0].as_ref());

    assert!(read_authenticate(&mut context, buffer.as_slice()).is_err());
}

#[test]
fn complete_authenticate_fails_on_incorrect_state() {
    let mut context = Ntlm::new();

    context.state = NtlmState::Authenticate;

    assert!(complete_authenticate(&mut context).is_err());
}

#[test]
fn complete_authenticate_does_not_fail_on_correct_mic() {
    let mut context = Ntlm::new();

    context.identity = Some(TEST_CREDENTIALS.clone());
    context.flags = NegotiateFlags::NTLM_SSP_NEGOTIATE_KEY_EXCH;
    context.state = NtlmState::Completion;
    context.negotiate_message = Some(NegotiateMessage::new(vec![0x01, 0x02, 0x03]));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x04, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.authenticate_message = Some(AuthenticateMessage::new(
        DOMAIN_AUTHENTICATE_MESSAGE.to_vec(),
        Some(Mic::new(
            [
                0xcf, 0x40, 0x63, 0x95, 0xcf, 0xe2, 0x50, 0x4d, 0xbb, 0x1f, 0x7b, 0x3e, 0x7, 0xd4, 0xb6, 0x49,
            ],
            64,
        )),
        DOMAIN_TARGET_INFO.to_vec(),
        DOMAIN_CLIENT_CHALLENGE,
        DOMAIN_ENCRYPTED_SESSION_KEY,
    ));

    complete_authenticate(&mut context).unwrap();
}

#[test]
fn complete_authenticate_fails_on_incorrect_challenge_message() {
    let mut context = Ntlm::new();

    context.identity = Some(TEST_CREDENTIALS.clone());
    context.flags = NegotiateFlags::NTLM_SSP_NEGOTIATE_KEY_EXCH;
    context.state = NtlmState::Completion;
    context.negotiate_message = Some(NegotiateMessage::new(vec![0x01, 0x02, 0x03]));
    context.challenge_message = Some(ChallengeMessage::new(
        vec![0x05, 0x05, 0x06],
        Vec::new(),
        [0x00; CHALLENGE_SIZE],
        0,
    ));
    context.authenticate_message = Some(AuthenticateMessage::new(
        DOMAIN_AUTHENTICATE_MESSAGE.to_vec(),
        Some(Mic::new(
            [
                0xcf, 0x40, 0x63, 0x95, 0xcf, 0xe2, 0x50, 0x4d, 0xbb, 0x1f, 0x7b, 0x3e, 0x7, 0xd4, 0xb6, 0x49,
            ],
            64,
        )),
        DOMAIN_TARGET_INFO.to_vec(),
        DOMAIN_CLIENT_CHALLENGE,
        DOMAIN_ENCRYPTED_SESSION_KEY,
    ));

    assert!(complete_authenticate(&mut context).is_err());
}
