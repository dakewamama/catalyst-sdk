export const SUBSCRIPTION_PROGRAM_ID = "De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44";

// snchor compatible event tag: Sha256("anchor:event")[..8] little-endian
export const EVENT_IX_TAG = new Uint8Array([
    0xe4, 0x45, 0xa5, 0x2a, 0x0a, 0xa1, 0x56, 0xe1
]);

// wire format: 8-byte tag + 1-byte discriminator
export const EVENT_DISCRIMINATOR_LEN = 9;

//Event discriminator values
export const EventDiscriminators = {
    SubscriptionEvent: 0,
    SubcriptionCancelled: 1,
    SubscriptionTransfer: 2,
    FixedTransfer: 3,
    RecurringTransfer: 4,
    SubscriptionResumed: 5,
} as const;

