import unittest

import bip39
import ed25519


class MyTestCase(unittest.TestCase):
    def test_sign_and_verify_message(self):

        message = b"test"

        # Get private and public key from seed
        seed = bip39.bip39_to_mini_secret('daughter song common combine misery cotton audit morning stuff weasel flee field','')
        private_key, public_key = ed25519.ed_from_seed(bytes(seed))

        # Generate signature
        signature = ed25519.ed_sign(public_key, private_key, message)

        # Verify message with signature
        self.assertTrue(ed25519.ed_verify(signature, message, public_key))


if __name__ == '__main__':
    unittest.main()
