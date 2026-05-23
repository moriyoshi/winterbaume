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
- Common required input members in this group: `DerivationMethodAttributes`, `EncryptionKeyIdentifier`, `GenerationAttributes`, `GenerationKeyIdentifier`, `KekValidationType`, `KeyIdentifier`, `MessageData`, `NewEncryptedPinBlock`, `NewPinPekIdentifier`, `PinBlockFormat`, `PrimaryAccountNumber`, `RandomKeySendVariantMask`, `SecureMessagingConfidentialityKeyIdentifier`, `SecureMessagingIntegrityKeyIdentifier`

### Verify

- Operations: `VerifyAuthRequestCryptogram`, `VerifyCardValidationData`, `VerifyMac`, `VerifyPinData`
- Common required input members in this group: `AuthRequestCryptogram`, `EncryptedPinBlock`, `EncryptionKeyIdentifier`, `KeyIdentifier`, `Mac`, `MajorKeyDerivationMode`, `MessageData`, `PinBlockFormat`, `PrimaryAccountNumber`, `SessionKeyDerivationAttributes`, `TransactionData`, `ValidationData`, `VerificationAttributes`, `VerificationKeyIdentifier`

### Translate

- Operations: `TranslateKeyMaterial`, `TranslatePinData`
- Common required input members in this group: `EncryptedPinBlock`, `IncomingKeyIdentifier`, `IncomingKeyMaterial`, `IncomingTranslationAttributes`, `OutgoingKeyIdentifier`, `OutgoingKeyMaterial`, `OutgoingTranslationAttributes`

### Decrypt

- Operations: `DecryptData`
- Common required input members in this group: `CipherText`, `DecryptionAttributes`, `KeyIdentifier`

### Encrypt

- Operations: `EncryptData`
- Common required input members in this group: `EncryptionAttributes`, `KeyIdentifier`, `PlainText`

### Re

- Operations: `ReEncryptData`
- Common required input members in this group: `CipherText`, `IncomingEncryptionAttributes`, `IncomingKeyIdentifier`, `OutgoingEncryptionAttributes`, `OutgoingKeyIdentifier`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DecryptData` | `POST /keys/{KeyIdentifier}/decrypt` | - | `CipherText`, `DecryptionAttributes`, `KeyIdentifier` | - | `DecryptDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Decrypts ciphertext data to plaintext using a symmetric (TDES, AES), asymmetric (RSA), or derived (DUKPT or EMV) encryption key scheme. For more information, see Decrypt data in the Amazon Web Services Payment Cryptography User Guide . |
| `EncryptData` | `POST /keys/{KeyIdentifier}/encrypt` | - | `EncryptionAttributes`, `KeyIdentifier`, `PlainText` | - | `EncryptDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Encrypts plaintext data to ciphertext using a symmetric (TDES, AES), asymmetric (RSA), or derived (DUKPT or EMV) encryption key scheme. For more information, see Encrypt data in the Amazon Web Services Payment Cryptography User Guide . |
| `GenerateAs2805KekValidation` | `POST /as2805kekvalidation/generate` | - | `KekValidationType`, `KeyIdentifier`, `RandomKeySendVariantMask` | - | `GenerateAs2805KekValidationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Establishes node-to-node initialization between payment processing nodes such as an acquirer, issuer or payment network using Australian Standard 2805 (AS2805). During node-to-node initialization, both communicating nodes must validate that they possess the... |
| `GenerateCardValidationData` | `POST /cardvalidationdata/generate` | - | `GenerationAttributes`, `KeyIdentifier`, `PrimaryAccountNumber` | - | `GenerateCardValidationDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Generates card-related validation data using algorithms such as Card Verification Values (CVV/CVV2), Dynamic Card Verification Values (dCVV/dCVV2), or Card Security Codes (CSC). For more information, see Generate card data in the Amazon Web Services Payment... |
| `GenerateMac` | `POST /mac/generate` | - | `GenerationAttributes`, `KeyIdentifier`, `MessageData` | - | `GenerateMacOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Generates a Message Authentication Code (MAC) cryptogram within Amazon Web Services Payment Cryptography. You can use this operation to authenticate card-related data by using known data values to generate MAC for data validation between the sending and... |
| `GenerateMacEmvPinChange` | `POST /macemvpinchange/generate` | - | `DerivationMethodAttributes`, `MessageData`, `NewEncryptedPinBlock`, `NewPinPekIdentifier`, `PinBlockFormat`, `SecureMessagingConfidentialityKeyIdentifier`, `SecureMessagingIntegrityKeyIdentifier` | - | `GenerateMacEmvPinChangeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Generates an issuer script mac for EMV payment cards that use offline PINs as the cardholder verification method (CVM). This operation generates an authenticated issuer script response by appending the incoming message data (APDU command) with the target... |
| `GeneratePinData` | `POST /pindata/generate` | - | `EncryptionKeyIdentifier`, `GenerationAttributes`, `GenerationKeyIdentifier`, `PinBlockFormat` | - | `GeneratePinDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Generates pin-related data such as PIN, PIN Verification Value (PVV), PIN Block, and PIN Offset during new card issuance or reissuance. For more information, see Generate PIN data in the Amazon Web Services Payment Cryptography User Guide . |
| `ReEncryptData` | `POST /keys/{IncomingKeyIdentifier}/reencrypt` | - | `CipherText`, `IncomingEncryptionAttributes`, `IncomingKeyIdentifier`, `OutgoingEncryptionAttributes`, `OutgoingKeyIdentifier` | - | `ReEncryptDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Re-encrypt ciphertext using DUKPT or Symmetric data encryption keys. You can either generate an encryption key within Amazon Web Services Payment Cryptography by calling CreateKey or import your own encryption key by calling ImportKey. |
| `TranslateKeyMaterial` | `POST /keymaterial/translate` | - | `IncomingKeyMaterial`, `OutgoingKeyMaterial` | - | `TranslateKeyMaterialOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Translates an cryptographic key between different wrapping keys without importing the key into Amazon Web Services Payment Cryptography. This operation can be used when key material is frequently rotated, such as during every card transaction, and there is a... |
| `TranslatePinData` | `POST /pindata/translate` | - | `EncryptedPinBlock`, `IncomingKeyIdentifier`, `IncomingTranslationAttributes`, `OutgoingKeyIdentifier`, `OutgoingTranslationAttributes` | - | `TranslatePinDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Translates encrypted PIN block from and to ISO 9564 formats 0,1,3,4. For more information, see Translate PIN data in the Amazon Web Services Payment Cryptography User Guide . |
| `VerifyAuthRequestCryptogram` | `POST /cryptogram/verify` | - | `AuthRequestCryptogram`, `KeyIdentifier`, `MajorKeyDerivationMode`, `SessionKeyDerivationAttributes`, `TransactionData` | - | `VerifyAuthRequestCryptogramOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException`, `VerificationFailedException` | Verifies Authorization Request Cryptogram (ARQC) for a EMV chip payment card authorization. For more information, see Verify auth request cryptogram in the Amazon Web Services Payment Cryptography User Guide . |
| `VerifyCardValidationData` | `POST /cardvalidationdata/verify` | - | `KeyIdentifier`, `PrimaryAccountNumber`, `ValidationData`, `VerificationAttributes` | - | `VerifyCardValidationDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException`, `VerificationFailedException` | Verifies card-related validation data using algorithms such as Card Verification Values (CVV/CVV2), Dynamic Card Verification Values (dCVV/dCVV2) and Card Security Codes (CSC). For more information, see Verify card data in the Amazon Web Services Payment... |
| `VerifyMac` | `POST /mac/verify` | - | `KeyIdentifier`, `Mac`, `MessageData`, `VerificationAttributes` | - | `VerifyMacOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException`, `VerificationFailedException` | Verifies a Message Authentication Code (MAC). You can use this operation to verify MAC for message data authentication such as . |
| `VerifyPinData` | `POST /pindata/verify` | - | `EncryptedPinBlock`, `EncryptionKeyIdentifier`, `PinBlockFormat`, `VerificationAttributes`, `VerificationKeyIdentifier` | - | `VerifyPinDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException`, `VerificationFailedException` | Verifies pin-related data such as PIN and PIN Offset using algorithms including VISA PVV and IBM3624. For more information, see Verify PIN data in the Amazon Web Services Payment Cryptography User Guide . |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message` | The request processing has failed because of an unknown error, exception, or failure. |
| `ResourceNotFoundException` | `structure` | `ResourceId` | The request was denied due to an invalid resource error. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `fieldList`, `message` | The request was denied due to an invalid request error. |
| `VerificationFailedException` | `structure` | `Message`, `Reason` | This request failed verification. |
| `DecryptDataInput` | `structure` | `CipherText`, `DecryptionAttributes`, `KeyIdentifier`, `WrappedKey` | - |
| `DecryptDataOutput` | `structure` | `KeyArn`, `KeyCheckValue`, `PlainText` | - |
| `EncryptDataInput` | `structure` | `EncryptionAttributes`, `KeyIdentifier`, `PlainText`, `WrappedKey` | - |
| `EncryptDataOutput` | `structure` | `CipherText`, `KeyArn`, `KeyCheckValue` | - |
| `GenerateAs2805KekValidationInput` | `structure` | `KekValidationType`, `KeyIdentifier`, `RandomKeySendVariantMask` | - |
| `GenerateAs2805KekValidationOutput` | `structure` | `KeyArn`, `KeyCheckValue`, `RandomKeyReceive`, `RandomKeySend` | - |
| `GenerateCardValidationDataInput` | `structure` | `GenerationAttributes`, `KeyIdentifier`, `PrimaryAccountNumber`, `ValidationDataLength` | - |
| `GenerateCardValidationDataOutput` | `structure` | `KeyArn`, `KeyCheckValue`, `ValidationData` | - |
| `GenerateMacInput` | `structure` | `GenerationAttributes`, `KeyIdentifier`, `MacLength`, `MessageData` | - |
| `GenerateMacOutput` | `structure` | `KeyArn`, `KeyCheckValue`, `Mac` | - |
| `GenerateMacEmvPinChangeInput` | `structure` | `DerivationMethodAttributes`, `MessageData`, `NewEncryptedPinBlock`, `NewPinPekIdentifier`, `PinBlockFormat`, `SecureMessagingConfidentialityKeyIdentifier`, `SecureMessagingIntegrityKeyIdentifier` | - |
| `GenerateMacEmvPinChangeOutput` | `structure` | `EncryptedPinBlock`, `Mac`, `NewPinPekArn`, `NewPinPekKeyCheckValue`, `SecureMessagingConfidentialityKeyArn`, `SecureMessagingConfidentialityKeyCheckValue`, `SecureMessagingIntegrityKeyArn`, `SecureMessagingIntegrityKeyCheckValue`, `VisaAmexDerivationOutputs` | - |
| `GeneratePinDataInput` | `structure` | `EncryptionKeyIdentifier`, `EncryptionWrappedKey`, `GenerationAttributes`, `GenerationKeyIdentifier`, `PinBlockFormat`, `PinDataLength`, `PrimaryAccountNumber` | - |
| `GeneratePinDataOutput` | `structure` | `EncryptedPinBlock`, `EncryptionKeyArn`, `EncryptionKeyCheckValue`, `GenerationKeyArn`, `GenerationKeyCheckValue`, `PinData` | - |
| `ReEncryptDataInput` | `structure` | `CipherText`, `IncomingEncryptionAttributes`, `IncomingKeyIdentifier`, `IncomingWrappedKey`, `OutgoingEncryptionAttributes`, `OutgoingKeyIdentifier`, `OutgoingWrappedKey` | - |
| `ReEncryptDataOutput` | `structure` | `CipherText`, `KeyArn`, `KeyCheckValue` | - |
| `TranslateKeyMaterialInput` | `structure` | `IncomingKeyMaterial`, `KeyCheckValueAlgorithm`, `OutgoingKeyMaterial` | - |
| `TranslateKeyMaterialOutput` | `structure` | `WrappedKey` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
