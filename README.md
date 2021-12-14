# py-ed25519-bindings
Python bindings for the [ed25519-dalek RUST crate](https://crates.io/crates/ed25519-dalek/1.0.1) 

## Documentation

https://docs.rs/py-ed25519-bindings

## Installation

### Install from PyPI

```
pip install py-ed25519-bindings
```

### Compile for local development

```
pip install -r requirements.txt
maturin develop
```
### Build wheelhouses
```
pip install -r requirements.txt

# Build local OS wheelhouse
maturin build

# Build manylinux1 wheelhouse
docker build . --tag polkasource/maturin
docker run --rm -i -v $(pwd):/io konstin2/maturin build

```

## Usage

```python
import bip39
import ed25519_dalek

message = b"test"

# Get private and public key from seed
seed = bip39.bip39_to_mini_secret('daughter song common combine misery cotton audit morning stuff weasel flee field','')
private_key, public_key = ed25519_dalek.ed_from_seed(bytes(seed))

# Generate signature
signature = ed25519_dalek.ed_sign(public_key, private_key, message)
print(signature.hex())

# Verify message with signature
if ed25519_dalek.ed_verify(signature, message, public_key):
    print('Verified')

```


## License
https://github.com/polkascan/py-ed25519-bindings/blob/master/LICENSE
