# Payment Cryptography Data Plane

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You use the Amazon Web Services Payment Cryptography Data Plane to manage how encryption keys are used for payment-related transaction processing and associated cryptographic operations. You can encrypt, decrypt, generate, verify, and translate payment-related cryptographic operations in Amazon Web Services Payment Cryptography. For more information, see Data operations in the Amazon Web Services Payment Cryptography User Guide . To manage your encryption keys, you use the Amazon Web Services Payment Cryptography Control Plane. You can create, import, export, share, manage, and delete keys.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Payment Cryptography Data Plane workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model workflows exposed by the modelled operation families across the `Generate`, `Verify`, `Translate`, `Decrypt`, `Encrypt` operation families, including `GenerateAs2805KekValidation`, `GenerateCardValidationData`, `GenerateMac`, `GenerateMacEmvPinChange`, `VerifyAuthRequestCryptogram`, `VerifyCardValidationData`.

## Service Identity and Protocol

- AWS model slug: `payment-cryptography-data`
- AWS SDK for Rust slug: `paymentcryptographydata`
- Model version: `2022-02-03`
- Model file: `vendor/api-models-aws/models/payment-cryptography-data/service/2022-02-03/payment-cryptography-data-2022-02-03.json`
- SDK ID: `Payment Cryptography Data`
- Endpoint prefix: `dataplane.payment-cryptography`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Generate` (5), `Verify` (4), `Translate` (2), `Decrypt` (1), `Encrypt` (1), `Re` (1).
- 14 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `KMS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Generate

- Operations: `GenerateAs2805KekValidation`, `GenerateCardValidationData`, `GenerateMac`, `GenerateMacEmvPinChange`, `GeneratePinData`
- Common required input members in this group: `KeyIdentifier`, `GenerationAttributes`, `MessageData`, `PinBlockFormat`

### Verify

- Operations: `VerifyAuthRequestCryptogram`, `VerifyCardValidationData`, `VerifyMac`, `VerifyPinData`
- Common required input members in this group: `KeyIdentifier`, `VerificationAttributes`

### Translate

- Operations: `TranslateKeyMaterial`, `TranslatePinData`
- Common required input members in this group: -

### Decrypt

- Operations: `DecryptData`
- Common required input members in this group: -

### Encrypt

- Operations: `EncryptData`
- Common required input members in this group: -

### Re

- Operations: `ReEncryptData`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DecryptData` | `POST /keys/{KeyIdentifier}/decrypt` | - | `KeyIdentifier`, `CipherText`, `DecryptionAttributes` | - | `DecryptDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Decrypts ciphertext data to plaintext using a symmetric (TDES, AES), asymmetric (RSA), or derived (DUKPT or EMV) encryption key scheme. For more information, see Decrypt data in the Amazon Web Services Payment Crypto ... |
| `EncryptData` | `POST /keys/{KeyIdentifier}/encrypt` | - | `KeyIdentifier`, `PlainText`, `EncryptionAttributes` | - | `EncryptDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Encrypts plaintext data to ciphertext using a symmetric (TDES, AES), asymmetric (RSA), or derived (DUKPT or EMV) encryption key scheme. For more information, see Encrypt data in the Amazon Web Services Payment Crypto ... |
| `GenerateAs2805KekValidation` | `POST /as2805kekvalidation/generate` | - | `KeyIdentifier`, `KekValidationType`, `RandomKeySendVariantMask` | - | `GenerateAs2805KekValidationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Establishes node-to-node initialization between payment processing nodes such as an acquirer, issuer or payment network using Australian Standard 2805 (AS2805). During node-to-node initialization, both communicating ... |
| `GenerateCardValidationData` | `POST /cardvalidationdata/generate` | - | `KeyIdentifier`, `PrimaryAccountNumber`, `GenerationAttributes` | - | `GenerateCardValidationDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Generates card-related validation data using algorithms such as Card Verification Values (CVV/CVV2), Dynamic Card Verification Values (dCVV/dCVV2), or Card Security Codes (CSC). For more information, see Generate car ... |
| `GenerateMac` | `POST /mac/generate` | - | `KeyIdentifier`, `MessageData`, `GenerationAttributes` | - | `GenerateMacOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Generates a Message Authentication Code (MAC) cryptogram within Amazon Web Services Payment Cryptography. You can use this operation to authenticate card-related data by using known data values to generate MAC for da ... |
| `GenerateMacEmvPinChange` | `POST /macemvpinchange/generate` | - | `NewPinPekIdentifier`, `NewEncryptedPinBlock`, `PinBlockFormat`, `SecureMessagingIntegrityKeyIdentifier`, `SecureMessagingConfidentialityKeyIdentifier`, `MessageData`, `DerivationMethodAttributes` | - | `GenerateMacEmvPinChangeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Generates an issuer script mac for EMV payment cards that use offline PINs as the cardholder verification method (CVM). This operation generates an authenticated issuer script response by appending the incoming messa ... |
| `GeneratePinData` | `POST /pindata/generate` | - | `GenerationKeyIdentifier`, `EncryptionKeyIdentifier`, `GenerationAttributes`, `PinBlockFormat` | - | `GeneratePinDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Generates pin-related data such as PIN, PIN Verification Value (PVV), PIN Block, and PIN Offset during new card issuance or reissuance. For more information, see Generate PIN data in the Amazon Web Services Payment C ... |
| `ReEncryptData` | `POST /keys/{IncomingKeyIdentifier}/reencrypt` | - | `IncomingKeyIdentifier`, `OutgoingKeyIdentifier`, `CipherText`, `IncomingEncryptionAttributes`, `OutgoingEncryptionAttributes` | - | `ReEncryptDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Re-encrypt ciphertext using DUKPT or Symmetric data encryption keys. You can either generate an encryption key within Amazon Web Services Payment Cryptography by calling CreateKey or import your own encryption key by ... |
| `TranslateKeyMaterial` | `POST /keymaterial/translate` | - | `IncomingKeyMaterial`, `OutgoingKeyMaterial` | - | `TranslateKeyMaterialOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Translates an cryptographic key between different wrapping keys without importing the key into Amazon Web Services Payment Cryptography. This operation can be used when key material is frequently rotated, such as dur ... |
| `TranslatePinData` | `POST /pindata/translate` | - | `IncomingKeyIdentifier`, `OutgoingKeyIdentifier`, `IncomingTranslationAttributes`, `OutgoingTranslationAttributes`, `EncryptedPinBlock` | - | `TranslatePinDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Translates encrypted PIN block from and to ISO 9564 formats 0,1,3,4. For more information, see Translate PIN data in the Amazon Web Services Payment Cryptography User Guide . PIN block translation involves changing a ... |
| `VerifyAuthRequestCryptogram` | `POST /cryptogram/verify` | - | `KeyIdentifier`, `TransactionData`, `AuthRequestCryptogram`, `MajorKeyDerivationMode`, `SessionKeyDerivationAttributes` | - | `VerifyAuthRequestCryptogramOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException`, `VerificationFailedException` | Verifies Authorization Request Cryptogram (ARQC) for a EMV chip payment card authorization. For more information, see Verify auth request cryptogram in the Amazon Web Services Payment Cryptography User Guide . ARQC g ... |
| `VerifyCardValidationData` | `POST /cardvalidationdata/verify` | - | `KeyIdentifier`, `PrimaryAccountNumber`, `VerificationAttributes`, `ValidationData` | - | `VerifyCardValidationDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException`, `VerificationFailedException` | Verifies card-related validation data using algorithms such as Card Verification Values (CVV/CVV2), Dynamic Card Verification Values (dCVV/dCVV2) and Card Security Codes (CSC). For more information, see Verify card d ... |
| `VerifyMac` | `POST /mac/verify` | - | `KeyIdentifier`, `MessageData`, `Mac`, `VerificationAttributes` | - | `VerifyMacOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException`, `VerificationFailedException` | Verifies a Message Authentication Code (MAC). You can use this operation to verify MAC for message data authentication such as . In this operation, you must use the same message data, secret encryption key and MAC al ... |
| `VerifyPinData` | `POST /pindata/verify` | - | `VerificationKeyIdentifier`, `EncryptionKeyIdentifier`, `VerificationAttributes`, `EncryptedPinBlock`, `PinBlockFormat` | - | `VerifyPinDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException`, `VerificationFailedException` | Verifies pin-related data such as PIN and PIN Offset using algorithms including VISA PVV and IBM3624. For more information, see Verify PIN data in the Amazon Web Services Payment Cryptography User Guide . This operat ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | Message | The request processing has failed because of an unknown error, exception, or failure. |
| `ResourceNotFoundException` | `structure` | ResourceId | The request was denied due to an invalid resource error. |
| `ThrottlingException` | `structure` | Message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, fieldList | The request was denied due to an invalid request error. |
| `VerificationFailedException` | `structure` | Reason, Message | This request failed verification. |
| `DecryptDataInput` | `structure` | KeyIdentifier, CipherText, DecryptionAttributes, WrappedKey | - |
| `DecryptDataOutput` | `structure` | KeyArn, KeyCheckValue, PlainText | - |
| `EncryptDataInput` | `structure` | KeyIdentifier, PlainText, EncryptionAttributes, WrappedKey | - |
| `EncryptDataOutput` | `structure` | KeyArn, KeyCheckValue, CipherText | - |
| `GenerateAs2805KekValidationInput` | `structure` | KeyIdentifier, KekValidationType, RandomKeySendVariantMask | - |
| `GenerateAs2805KekValidationOutput` | `structure` | KeyArn, KeyCheckValue, RandomKeySend, RandomKeyReceive | - |
| `GenerateCardValidationDataInput` | `structure` | KeyIdentifier, PrimaryAccountNumber, GenerationAttributes, ValidationDataLength | - |
| `GenerateCardValidationDataOutput` | `structure` | KeyArn, KeyCheckValue, ValidationData | - |
| `GenerateMacInput` | `structure` | KeyIdentifier, MessageData, GenerationAttributes, MacLength | - |
| `GenerateMacOutput` | `structure` | KeyArn, KeyCheckValue, Mac | - |
| `GenerateMacEmvPinChangeInput` | `structure` | NewPinPekIdentifier, NewEncryptedPinBlock, PinBlockFormat, SecureMessagingIntegrityKeyIdentifier, SecureMessagingConfidentialityKeyIdentifier, MessageData, DerivationMethodAttributes | - |
| `GenerateMacEmvPinChangeOutput` | `structure` | NewPinPekArn, SecureMessagingIntegrityKeyArn, SecureMessagingConfidentialityKeyArn, Mac, EncryptedPinBlock, NewPinPekKeyCheckValue, SecureMessagingIntegrityKeyCheckValue, SecureMessagingConfidentialityKeyCheckValue, VisaAmexDerivationOutputs | - |
| `GeneratePinDataInput` | `structure` | GenerationKeyIdentifier, EncryptionKeyIdentifier, GenerationAttributes, PinDataLength, PrimaryAccountNumber, PinBlockFormat, EncryptionWrappedKey | - |
| `GeneratePinDataOutput` | `structure` | GenerationKeyArn, GenerationKeyCheckValue, EncryptionKeyArn, EncryptionKeyCheckValue, EncryptedPinBlock, PinData | - |
| `ReEncryptDataInput` | `structure` | IncomingKeyIdentifier, OutgoingKeyIdentifier, CipherText, IncomingEncryptionAttributes, OutgoingEncryptionAttributes, IncomingWrappedKey, OutgoingWrappedKey | - |
| `ReEncryptDataOutput` | `structure` | KeyArn, KeyCheckValue, CipherText | - |
| `TranslateKeyMaterialInput` | `structure` | IncomingKeyMaterial, OutgoingKeyMaterial, KeyCheckValueAlgorithm | - |
| `TranslateKeyMaterialOutput` | `structure` | WrappedKey | - |
| `TranslatePinDataInput` | `structure` | IncomingKeyIdentifier, OutgoingKeyIdentifier, IncomingTranslationAttributes, OutgoingTranslationAttributes, EncryptedPinBlock, IncomingDukptAttributes, OutgoingDukptAttributes, IncomingWrappedKey, OutgoingWrappedKey, IncomingAs2805Attributes | - |
| `TranslatePinDataOutput` | `structure` | PinBlock, KeyArn, KeyCheckValue | - |
| `VerifyAuthRequestCryptogramInput` | `structure` | KeyIdentifier, TransactionData, AuthRequestCryptogram, MajorKeyDerivationMode, SessionKeyDerivationAttributes, AuthResponseAttributes | - |
| `VerifyAuthRequestCryptogramOutput` | `structure` | KeyArn, KeyCheckValue, AuthResponseValue | - |
| `VerifyCardValidationDataInput` | `structure` | KeyIdentifier, PrimaryAccountNumber, VerificationAttributes, ValidationData | - |
| `VerifyCardValidationDataOutput` | `structure` | KeyArn, KeyCheckValue | - |
| `VerifyMacInput` | `structure` | KeyIdentifier, MessageData, Mac, VerificationAttributes, MacLength | - |
| `VerifyMacOutput` | `structure` | KeyArn, KeyCheckValue | - |
| `VerifyPinDataInput` | `structure` | VerificationKeyIdentifier, EncryptionKeyIdentifier, VerificationAttributes, EncryptedPinBlock, PrimaryAccountNumber, PinBlockFormat, PinDataLength, DukptAttributes, EncryptionWrappedKey | - |
| `VerifyPinDataOutput` | `structure` | VerificationKeyArn, VerificationKeyCheckValue, EncryptionKeyArn, EncryptionKeyCheckValue | - |
| `DukptDerivationType` | `enum` | TDES_2KEY, TDES_3KEY, AES_128, AES_192, AES_256 | - |
| `DukptEncryptionMode` | `enum` | ECB, CBC | - |
| `DukptKeyVariant` | `enum` | BIDIRECTIONAL, REQUEST, RESPONSE | - |
| `EmvEncryptionMode` | `enum` | ECB, CBC | - |
| `EmvMajorKeyDerivationMode` | `enum` | EMV_OPTION_A, EMV_OPTION_B | - |
| `EncryptionMode` | `enum` | ECB, CBC, CFB, CFB1, CFB8, CFB64, CFB128, OFB | - |
| `KeyDerivationFunction` | `enum` | NIST_SP800, ANSI_X963 | - |
| `KeyDerivationHashAlgorithm` | `enum` | SHA_256, SHA_384, SHA_512 | - |
| `MacAlgorithm` | `enum` | ISO9797_ALGORITHM1, ISO9797_ALGORITHM3, CMAC, HMAC, HMAC_SHA224, HMAC_SHA256, HMAC_SHA384, HMAC_SHA512, AS2805_4_1 | - |
| `MajorKeyDerivationMode` | `enum` | EMV_OPTION_A, EMV_OPTION_B | - |
| `PaddingType` | `enum` | PKCS1, OAEP_SHA1, OAEP_SHA256, OAEP_SHA512 | - |
| `PinBlockFormatForEmvPinChange` | `enum` | ISO_FORMAT_0, ISO_FORMAT_1, ISO_FORMAT_3 | - |
| `PinBlockFormatForPinData` | `enum` | ISO_FORMAT_0, ISO_FORMAT_1, ISO_FORMAT_3, ISO_FORMAT_4 | - |
| `PinBlockLengthPosition` | `enum` | NONE, FRONT_OF_PIN_BLOCK | - |
| `PinBlockPaddingType` | `enum` | NO_PADDING, ISO_IEC_7816_4 | - |
| `RandomKeySendVariantMask` | `enum` | VARIANT_MASK_82C0, VARIANT_MASK_82 | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
