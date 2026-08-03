import { createCipheriv, createDecipheriv, randomBytes } from "node:crypto";

function key() {
  const value = process.env.ACME_BUNDLE_SIGNING_KEY;
  if (!value) throw new Error("ACME_BUNDLE_SIGNING_KEY is not configured");
  const decoded = Buffer.from(value, "base64");
  if (decoded.length !== 32) throw new Error("ACME_BUNDLE_SIGNING_KEY must be 32 random base64 bytes");
  return decoded;
}

export function encryptSecret(secret: string) {
  const iv = randomBytes(12); const cipher = createCipheriv("aes-256-gcm", key(), iv);
  const encrypted = Buffer.concat([cipher.update(secret, "utf8"), cipher.final(), cipher.getAuthTag()]);
  return { ciphertext: encrypted.toString("base64"), iv: iv.toString("base64") };
}

export function decryptSecret(ciphertext: string, iv: string) {
  const bytes = Buffer.from(ciphertext, "base64"); const tag = bytes.subarray(bytes.length - 16); const encrypted = bytes.subarray(0, -16);
  const decipher = createDecipheriv("aes-256-gcm", key(), Buffer.from(iv, "base64")); decipher.setAuthTag(tag);
  return Buffer.concat([decipher.update(encrypted), decipher.final()]).toString("utf8");
}
