use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use pyo3::{wrap_pyfunction, IntoPy, PyObject};

use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};

pub struct PyKeypair([u8; 32], [u8; 32]);
pub struct PySignature([u8; 64]);

/// Keypair helper function.
fn create_from_pair(pair: &[u8]) -> Keypair {
	match Keypair::from_bytes(pair) {
		Ok(pair) => return pair,
		Err(_) => panic!("Provided pair is invalid.")
	}
}

/// Keypair helper function
fn create_from_parts(public: &[u8], secret: &[u8]) -> Keypair {
	let mut pair = vec![];

	pair.extend_from_slice(secret);
	pair.extend_from_slice(public);

	create_from_pair(&pair)
}

/// Keypair helper function.
fn create_from_seed(seed: &[u8]) -> Keypair {
	let secret = SecretKey::from_bytes(seed).unwrap();
	let public: PublicKey = (&secret).into();

	create_from_parts(public.as_bytes(), seed)
}

/// PublicKey helper
fn create_public(public: &[u8]) -> PublicKey {
	match PublicKey::from_bytes(public) {
		Ok(public) => return public,
		Err(_) => panic!("Provided public key is invalid.")
	}
}

#[pyfunction]
pub fn ed_from_seed(seed: &[u8]) -> PyResult<PyKeypair> {

	let keypair = create_from_seed(seed);

	Ok(PyKeypair(keypair.secret.to_bytes(), keypair.public.to_bytes()))

}

#[pyfunction]
pub fn ed_sign(public: &[u8], secret: &[u8], message: &[u8]) -> PyResult<PySignature> {
	Ok(PySignature(create_from_parts(public, secret)
		.sign(message)
		.to_bytes()))
}

#[pyfunction]
pub fn ed_verify(signature: &[u8], message: &[u8], public: &[u8]) -> bool {
	let signature = match Signature::from_bytes(signature) {
		Ok(signature) => signature,
		Err(_) => return false
	};

	create_public(public)
		.verify(message, &signature)
		.is_ok()
}


/// This module is a Python module implemented in Rust.
#[pymodule]
fn ed25519(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(ed_from_seed))?;
    m.add_wrapped(wrap_pyfunction!(ed_sign))?;
    m.add_wrapped(wrap_pyfunction!(ed_verify))?;

    Ok(())
}

// Convert Keypair object to a Python Keypair tuple
impl IntoPy<PyObject> for PyKeypair {
    fn into_py(self, py: Python) -> PyObject {
        let secret = PyBytes::new(py, &self.0);
        let public = PyBytes::new(py, &self.1);

        PyTuple::new(py, vec![secret, public]).into_py(py)
    }
}

// Convert Keypair object to a Python Keypair tuple
impl IntoPy<PyObject> for PySignature {
    fn into_py(self, py: Python) -> PyObject {
        let sig = PyBytes::new(py, &self.0);
        sig.into_py(py)
    }
}
