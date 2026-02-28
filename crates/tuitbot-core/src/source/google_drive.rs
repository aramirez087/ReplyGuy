//! Google Drive content source provider.
//!
//! Polls a Google Drive folder for `.md` and `.txt` files using the
//! Drive API v3 with service-account authentication.  The provider
//! records stable Google Drive file IDs as `provider_id` values in
//! `gdrive://<file_id>/<filename>` format for deduplication.
//!
//! Authentication uses a service-account JSON key: the provider reads
//! the key file, builds a JWT, and exchanges it for an access token
//! via Google's OAuth2 token endpoint.  Tokens are cached in memory
//! with expiry tracking.

use std::path::Path;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use sha2::{Digest, Sha256};

use super::{ContentSourceProvider, SourceError, SourceFile};
use crate::automation::watchtower::matches_patterns;

// ---------------------------------------------------------------------------
// Provider
// ---------------------------------------------------------------------------

/// Google Drive content source provider.
///
/// Instantiated only when a `google_drive` source is configured with a
/// valid `folder_id` and `service_account_key` path.
pub struct GoogleDriveProvider {
    folder_id: String,
    service_account_key_path: String,
    http_client: reqwest::Client,
    token_cache: Mutex<Option<CachedToken>>,
}

struct CachedToken {
    access_token: String,
    expires_at: Instant,
}

impl GoogleDriveProvider {
    pub fn new(folder_id: String, service_account_key_path: String) -> Self {
        Self {
            folder_id,
            service_account_key_path,
            http_client: reqwest::Client::new(),
            token_cache: Mutex::new(None),
        }
    }

    /// Build with an explicit HTTP client (for testing with wiremock).
    #[cfg(test)]
    pub fn with_client(
        folder_id: String,
        service_account_key_path: String,
        client: reqwest::Client,
    ) -> Self {
        Self {
            folder_id,
            service_account_key_path,
            http_client: client,
            token_cache: Mutex::new(None),
        }
    }

    /// Obtain a valid access token, refreshing if expired.
    async fn get_access_token(&self) -> Result<String, SourceError> {
        // Check cache.
        if let Ok(cache) = self.token_cache.lock() {
            if let Some(ref tok) = *cache {
                if tok.expires_at > Instant::now() + Duration::from_secs(60) {
                    return Ok(tok.access_token.clone());
                }
            }
        }

        let token = self.fetch_new_token().await?;
        let access_token = token.access_token.clone();

        if let Ok(mut cache) = self.token_cache.lock() {
            *cache = Some(token);
        }

        Ok(access_token)
    }

    /// Read the service-account key, build a JWT, and exchange for an
    /// access token via Google's token endpoint.
    async fn fetch_new_token(&self) -> Result<CachedToken, SourceError> {
        let key_bytes = tokio::fs::read_to_string(&self.service_account_key_path)
            .await
            .map_err(|e| {
                SourceError::Auth(format!(
                    "cannot read service account key {}: {e}",
                    self.service_account_key_path
                ))
            })?;

        let key_json: serde_json::Value = serde_json::from_str(&key_bytes)
            .map_err(|e| SourceError::Auth(format!("invalid service account JSON: {e}")))?;

        let client_email = key_json["client_email"]
            .as_str()
            .ok_or_else(|| SourceError::Auth("missing client_email in key".into()))?;

        let private_key_pem = key_json["private_key"]
            .as_str()
            .ok_or_else(|| SourceError::Auth("missing private_key in key".into()))?;

        let token_uri = key_json["token_uri"]
            .as_str()
            .unwrap_or("https://oauth2.googleapis.com/token");

        // Build JWT claims.
        let now = chrono::Utc::now().timestamp();
        let claims = serde_json::json!({
            "iss": client_email,
            "scope": "https://www.googleapis.com/auth/drive.readonly",
            "aud": token_uri,
            "iat": now,
            "exp": now + 3600,
        });

        let jwt = build_jwt(&claims, private_key_pem)?;

        // Exchange JWT for access token.
        let resp = self
            .http_client
            .post(token_uri)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ])
            .send()
            .await
            .map_err(|e| SourceError::Auth(format!("token exchange failed: {e}")))?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(SourceError::Auth(format!(
                "token endpoint returned error: {body}"
            )));
        }

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| SourceError::Auth(format!("invalid token response: {e}")))?;

        let access_token = body["access_token"]
            .as_str()
            .ok_or_else(|| SourceError::Auth("no access_token in response".into()))?
            .to_string();

        let expires_in = body["expires_in"].as_u64().unwrap_or(3600);

        Ok(CachedToken {
            access_token,
            expires_at: Instant::now() + Duration::from_secs(expires_in),
        })
    }
}

#[async_trait]
impl ContentSourceProvider for GoogleDriveProvider {
    fn source_type(&self) -> &str {
        "google_drive"
    }

    async fn scan_for_changes(
        &self,
        since_cursor: Option<&str>,
        patterns: &[String],
    ) -> Result<Vec<SourceFile>, SourceError> {
        let token = self.get_access_token().await?;

        // Build query: files in this folder, not trashed.
        let mut q = format!("'{}' in parents and trashed = false", self.folder_id);

        // Filter by modified time if we have a cursor.
        if let Some(cursor) = since_cursor {
            q.push_str(&format!(" and modifiedTime > '{cursor}'"));
        }

        let resp = self
            .http_client
            .get("https://www.googleapis.com/drive/v3/files")
            .bearer_auth(&token)
            .query(&[
                ("q", q.as_str()),
                ("fields", "files(id,name,md5Checksum,modifiedTime,mimeType)"),
                ("pageSize", "1000"),
            ])
            .send()
            .await
            .map_err(|e| SourceError::Network(format!("Drive list failed: {e}")))?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(SourceError::Network(format!("Drive API error: {body}")));
        }

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| SourceError::Network(format!("invalid Drive response: {e}")))?;

        let files = body["files"].as_array().cloned().unwrap_or_default();

        let mut result = Vec::new();
        for file in &files {
            let id = match file["id"].as_str() {
                Some(id) => id,
                None => continue,
            };
            let name = file["name"].as_str().unwrap_or("unknown");

            // Filter by patterns (match against filename).
            if !patterns.is_empty() && !matches_patterns(Path::new(name), patterns) {
                continue;
            }

            let hash = file["md5Checksum"].as_str().unwrap_or("").to_string();
            let modified = file["modifiedTime"].as_str().unwrap_or("").to_string();

            result.push(SourceFile {
                provider_id: format!("gdrive://{id}/{name}"),
                display_name: name.to_string(),
                content_hash: hash,
                modified_at: modified,
            });
        }

        Ok(result)
    }

    async fn read_content(&self, file_id: &str) -> Result<String, SourceError> {
        // Extract the Drive file ID from our provider_id format.
        let drive_id = extract_drive_id(file_id)?;

        let token = self.get_access_token().await?;

        let url = format!("https://www.googleapis.com/drive/v3/files/{drive_id}?alt=media");

        let resp = self
            .http_client
            .get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| SourceError::Network(format!("Drive get failed: {e}")))?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(SourceError::NotFound(format!("file {drive_id} not found")));
        }

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(SourceError::Network(format!(
                "Drive download error: {body}"
            )));
        }

        resp.text()
            .await
            .map_err(|e| SourceError::Network(format!("read body failed: {e}")))
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Test-only accessor for `extract_drive_id`.
#[cfg(test)]
impl GoogleDriveProvider {
    pub fn extract_drive_id_for_test(provider_id: &str) -> String {
        extract_drive_id(provider_id).unwrap()
    }
}

/// Extract Drive file ID from `gdrive://<id>/<name>` format.
/// Also accepts a raw ID without the prefix.
fn extract_drive_id(provider_id: &str) -> Result<String, SourceError> {
    if let Some(rest) = provider_id.strip_prefix("gdrive://") {
        if let Some(slash) = rest.find('/') {
            Ok(rest[..slash].to_string())
        } else {
            Ok(rest.to_string())
        }
    } else {
        Ok(provider_id.to_string())
    }
}

/// Build a signed JWT for Google service-account auth.
///
/// Uses RS256 (RSA + SHA-256). The private key is parsed from PEM format.
fn build_jwt(claims: &serde_json::Value, private_key_pem: &str) -> Result<String, SourceError> {
    let header = base64_url_encode(
        &serde_json::to_vec(&serde_json::json!({"alg": "RS256", "typ": "JWT"}))
            .map_err(|e| SourceError::Auth(format!("JWT header: {e}")))?,
    );
    let payload = base64_url_encode(
        &serde_json::to_vec(claims).map_err(|e| SourceError::Auth(format!("JWT payload: {e}")))?,
    );

    let signing_input = format!("{header}.{payload}");

    let signature = rsa_sign_sha256(signing_input.as_bytes(), private_key_pem)?;
    let sig_b64 = base64_url_encode(&signature);

    Ok(format!("{signing_input}.{sig_b64}"))
}

/// RSA-SHA256 signing using the `rsa` crate (already an indirect dep via oauth2).
fn rsa_sign_sha256(data: &[u8], pem: &str) -> Result<Vec<u8>, SourceError> {
    // Parse PEM to DER.
    let der = pem_to_der(pem)?;

    // Hash with SHA-256.
    let hash = Sha256::digest(data);

    // Build PKCS#1 v1.5 DigestInfo for SHA-256.
    let digest_info = build_pkcs1_digest_info(&hash);

    // Parse RSA private key and sign.
    rsa_pkcs1_sign(&der, &digest_info)
}

/// Decode a PEM-encoded RSA private key to DER bytes.
fn pem_to_der(pem: &str) -> Result<Vec<u8>, SourceError> {
    let pem = pem.trim();
    let body: String = pem
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<Vec<_>>()
        .join("");

    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(&body)
        .map_err(|e| SourceError::Auth(format!("PEM decode failed: {e}")))
}

/// Build PKCS#1 v1.5 DigestInfo prefix for SHA-256.
fn build_pkcs1_digest_info(hash: &[u8]) -> Vec<u8> {
    // DER encoding of DigestInfo for SHA-256:
    // SEQUENCE { SEQUENCE { OID sha256, NULL }, OCTET STRING hash }
    let prefix: &[u8] = &[
        0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01,
        0x05, 0x00, 0x04, 0x20,
    ];
    let mut info = prefix.to_vec();
    info.extend_from_slice(hash);
    info
}

/// Minimal RSA PKCS#1 v1.5 signing from DER-encoded private key.
///
/// Parses PKCS#8 (the format Google uses) and extracts modulus + private
/// exponent, then performs raw RSA: `signature = message^d mod n`.
fn rsa_pkcs1_sign(der: &[u8], digest_info: &[u8]) -> Result<Vec<u8>, SourceError> {
    // Parse PKCS#8 wrapper to get the inner RSA key.
    let rsa_key = parse_pkcs8_rsa(der)?;

    let k = rsa_key.n_bytes.len(); // key length in bytes

    // PKCS#1 v1.5 padding: 0x00 0x01 [0xFF...] 0x00 [DigestInfo]
    if digest_info.len() + 11 > k {
        return Err(SourceError::Auth("RSA key too small for signature".into()));
    }

    let mut em = vec![0x00, 0x01];
    let ps_len = k - digest_info.len() - 3;
    em.extend(std::iter::repeat(0xFF).take(ps_len));
    em.push(0x00);
    em.extend_from_slice(digest_info);

    // Convert to big integer and compute m^d mod n.
    let m = BigUint::from_bytes_be(&em);
    let n = BigUint::from_bytes_be(&rsa_key.n_bytes);
    let d = BigUint::from_bytes_be(&rsa_key.d_bytes);

    let sig = mod_pow(&m, &d, &n);
    let mut sig_bytes = sig.to_bytes_be();

    // Left-pad to key length.
    while sig_bytes.len() < k {
        sig_bytes.insert(0, 0);
    }

    Ok(sig_bytes)
}

// ---------------------------------------------------------------------------
// Minimal big-integer arithmetic for RSA
// ---------------------------------------------------------------------------

/// Simple big unsigned integer backed by a byte vector (big-endian).
#[derive(Clone)]
struct BigUint {
    /// Big-endian bytes with no leading zeros (except for zero itself).
    bytes: Vec<u8>,
}

impl BigUint {
    fn from_bytes_be(b: &[u8]) -> Self {
        let start = b
            .iter()
            .position(|&x| x != 0)
            .unwrap_or(b.len().saturating_sub(1));
        Self {
            bytes: b[start..].to_vec(),
        }
    }

    fn to_bytes_be(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn is_zero(&self) -> bool {
        self.bytes.iter().all(|&b| b == 0)
    }

    fn bit_len(&self) -> usize {
        if self.is_zero() {
            return 0;
        }
        let top = self.bytes[0];
        (self.bytes.len() - 1) * 8 + (8 - top.leading_zeros() as usize)
    }

    fn bit(&self, i: usize) -> bool {
        let byte_idx = self.bytes.len() - 1 - i / 8;
        if byte_idx >= self.bytes.len() {
            return false;
        }
        (self.bytes[byte_idx] >> (i % 8)) & 1 == 1
    }

    fn mul_mod(a: &BigUint, b: &BigUint, m: &BigUint) -> BigUint {
        // Simple schoolbook multiplication then mod.
        let a_len = a.bytes.len();
        let b_len = b.bytes.len();
        let mut result = vec![0u32; a_len + b_len];

        for i in (0..a_len).rev() {
            let mut carry: u32 = 0;
            for j in (0..b_len).rev() {
                let prod = (a.bytes[i] as u32) * (b.bytes[j] as u32) + result[i + j + 1] + carry;
                result[i + j + 1] = prod & 0xFF;
                carry = prod >> 8;
            }
            result[i] += carry;
        }

        let bytes: Vec<u8> = result.iter().map(|&x| x as u8).collect();
        let val = BigUint::from_bytes_be(&bytes);
        BigUint::modulo(&val, m)
    }

    fn modulo(a: &BigUint, m: &BigUint) -> BigUint {
        if a.bytes.len() < m.bytes.len() {
            return a.clone();
        }
        // Binary long division.
        let mut remainder = BigUint::from_bytes_be(&[0]);
        let total_bits = a.bytes.len() * 8;

        for i in (0..total_bits).rev() {
            // Shift remainder left by 1 and add next bit of a.
            remainder = BigUint::shift_left_one(&remainder);
            if a.bit(i) {
                let last = remainder.bytes.len() - 1;
                remainder.bytes[last] |= 1;
            }
            if BigUint::gte(&remainder, m) {
                remainder = BigUint::sub(&remainder, m);
            }
        }
        remainder
    }

    fn shift_left_one(a: &BigUint) -> BigUint {
        let mut result = vec![0u8; a.bytes.len() + 1];
        let mut carry = 0u8;
        for i in (0..a.bytes.len()).rev() {
            let val = (a.bytes[i] as u16) * 2 + carry as u16;
            result[i + 1] = val as u8;
            carry = (val >> 8) as u8;
        }
        result[0] = carry;
        BigUint::from_bytes_be(&result)
    }

    fn gte(a: &BigUint, b: &BigUint) -> bool {
        if a.bytes.len() != b.bytes.len() {
            return a.bytes.len() > b.bytes.len();
        }
        a.bytes >= b.bytes
    }

    fn sub(a: &BigUint, b: &BigUint) -> BigUint {
        let len = a.bytes.len().max(b.bytes.len());
        let mut result = vec![0i16; len];
        let a_off = len - a.bytes.len();
        let b_off = len - b.bytes.len();

        for i in 0..a.bytes.len() {
            result[a_off + i] += a.bytes[i] as i16;
        }
        for i in 0..b.bytes.len() {
            result[b_off + i] -= b.bytes[i] as i16;
        }

        // Propagate borrows.
        for i in (1..len).rev() {
            if result[i] < 0 {
                result[i] += 256;
                result[i - 1] -= 1;
            }
        }

        let bytes: Vec<u8> = result.iter().map(|&x| x as u8).collect();
        BigUint::from_bytes_be(&bytes)
    }
}

/// Modular exponentiation: base^exp mod modulus.
fn mod_pow(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let bits = exp.bit_len();
    if bits == 0 {
        return BigUint::from_bytes_be(&[1]);
    }

    let mut result = BigUint::from_bytes_be(&[1]);
    let mut b = BigUint::modulo(base, modulus);

    for i in 0..bits {
        if exp.bit(i) {
            result = BigUint::mul_mod(&result, &b, modulus);
        }
        b = BigUint::mul_mod(&b, &b, modulus);
    }
    result
}

// ---------------------------------------------------------------------------
// ASN.1/DER parsing for PKCS#8 RSA keys
// ---------------------------------------------------------------------------

struct RsaKeyParts {
    n_bytes: Vec<u8>,
    d_bytes: Vec<u8>,
}

/// Parse a PKCS#8 DER-encoded RSA private key and extract (n, d).
fn parse_pkcs8_rsa(der: &[u8]) -> Result<RsaKeyParts, SourceError> {
    // PKCS#8 is a SEQUENCE containing:
    //   INTEGER version
    //   SEQUENCE { OID rsaEncryption, NULL }
    //   OCTET STRING (containing PKCS#1 RSA private key)
    let (_, inner) = parse_der_sequence(der)
        .map_err(|_| SourceError::Auth("invalid PKCS#8 outer SEQUENCE".into()))?;

    // Skip version INTEGER.
    let rest =
        skip_der_element(inner).map_err(|_| SourceError::Auth("invalid PKCS#8 version".into()))?;

    // Skip algorithm SEQUENCE.
    let rest =
        skip_der_element(rest).map_err(|_| SourceError::Auth("invalid PKCS#8 algorithm".into()))?;

    // Parse OCTET STRING containing the PKCS#1 key.
    let (_, pkcs1_der) = parse_der_octet_string(rest)
        .map_err(|_| SourceError::Auth("invalid PKCS#8 octet string".into()))?;

    parse_pkcs1_rsa(pkcs1_der)
}

/// Parse a PKCS#1 DER-encoded RSA private key and extract (n, d).
fn parse_pkcs1_rsa(der: &[u8]) -> Result<RsaKeyParts, SourceError> {
    // SEQUENCE { version, n, e, d, p, q, dp, dq, qinv }
    let (_, inner) =
        parse_der_sequence(der).map_err(|_| SourceError::Auth("invalid PKCS#1 SEQUENCE".into()))?;

    // Skip version.
    let rest =
        skip_der_element(inner).map_err(|_| SourceError::Auth("invalid PKCS#1 version".into()))?;

    // Read n.
    let (rest, n_bytes) =
        parse_der_integer(rest).map_err(|_| SourceError::Auth("invalid PKCS#1 modulus".into()))?;

    // Skip e.
    let rest =
        skip_der_element(rest).map_err(|_| SourceError::Auth("invalid PKCS#1 exponent".into()))?;

    // Read d.
    let (_rest, d_bytes) = parse_der_integer(rest)
        .map_err(|_| SourceError::Auth("invalid PKCS#1 private exponent".into()))?;

    Ok(RsaKeyParts { n_bytes, d_bytes })
}

// Minimal DER parsing helpers.

fn parse_der_length(data: &[u8]) -> Result<(usize, &[u8]), ()> {
    if data.is_empty() {
        return Err(());
    }
    if data[0] < 0x80 {
        Ok((data[0] as usize, &data[1..]))
    } else {
        let num_bytes = (data[0] & 0x7F) as usize;
        if num_bytes == 0 || num_bytes > 4 || data.len() < 1 + num_bytes {
            return Err(());
        }
        let mut len: usize = 0;
        for i in 0..num_bytes {
            len = (len << 8) | data[1 + i] as usize;
        }
        Ok((len, &data[1 + num_bytes..]))
    }
}

fn parse_der_sequence(data: &[u8]) -> Result<(&[u8], &[u8]), ()> {
    if data.is_empty() || data[0] != 0x30 {
        return Err(());
    }
    let (len, rest) = parse_der_length(&data[1..])?;
    if rest.len() < len {
        return Err(());
    }
    Ok((&rest[len..], &rest[..len]))
}

fn parse_der_octet_string(data: &[u8]) -> Result<(&[u8], &[u8]), ()> {
    if data.is_empty() || data[0] != 0x04 {
        return Err(());
    }
    let (len, rest) = parse_der_length(&data[1..])?;
    if rest.len() < len {
        return Err(());
    }
    Ok((&rest[len..], &rest[..len]))
}

fn parse_der_integer(data: &[u8]) -> Result<(&[u8], Vec<u8>), ()> {
    if data.is_empty() || data[0] != 0x02 {
        return Err(());
    }
    let (len, rest) = parse_der_length(&data[1..])?;
    if rest.len() < len {
        return Err(());
    }
    let mut bytes = rest[..len].to_vec();
    // Strip leading zero used for positive sign.
    if bytes.len() > 1 && bytes[0] == 0 {
        bytes.remove(0);
    }
    Ok((&rest[len..], bytes))
}

fn skip_der_element(data: &[u8]) -> Result<&[u8], ()> {
    if data.is_empty() {
        return Err(());
    }
    let (len, rest) = parse_der_length(&data[1..])?;
    if rest.len() < len {
        return Err(());
    }
    Ok(&rest[len..])
}

/// URL-safe Base64 encoding without padding.
fn base64_url_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data)
}
